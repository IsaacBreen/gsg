
use crate::RightData;
use crate::VecX;
use crate::{profile_internal, CombinatorTrait, ParseResultTrait, ParseResults, ParserTrait, U8Set, VecY, DownData, UpData, OneShotUpData};
// src/combinators/seq.rs
use crate::{BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseResults};
use std::collections::BTreeMap;
use std::hash::Hash;

macro_rules! profile {
    ($name:expr, $expr:expr) => {
        $expr
    };
}

#[derive(Debug)]
pub struct Seq<'a> {
    pub(crate) children: VecX<Box<dyn DynCombinatorTrait + 'a>>,
    pub(crate) start_index: usize,
}

#[derive(Debug)]
pub struct SeqParser<'a> {
    pub(crate) parsers: Vec<(usize, Box<dyn ParserTrait + 'a>)>,
    pub(crate) combinators: &'a VecX<Box<dyn DynCombinatorTrait + 'a>>,
    pub(crate) position: usize,
}

impl DynCombinatorTrait for Seq<'_> {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}
impl CombinatorTrait for Seq<'_> {
    type Parser<'a> = SeqParser<'a> where Self: 'a;
    type Output = Vec<Box<dyn std::any::Any>>;
    type PartialOutput = Vec<Option<Box<dyn std::any::Any>>>;

    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        let mut right_data = down_data.right_data;
        let start_position = right_data.right_data_inner.get_fields1().position;
        for combinator in self.children.iter() {
            let offset = right_data.right_data_inner.get_fields1().position - start_position;
            let result = combinator.one_shot_parse(DownData { right_data }, &bytes[offset..]);
            match result {
                Ok(OneShotUpData { right_data: new_right_data }) => {
                    right_data = new_right_data;
                },
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(OneShotUpData { right_data })
    }

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let start_position = down_data.right_data.right_data_inner.get_fields1().position;

        let mut combinator_index = self.start_index;

        // if combinator_index >= self.children.len() {
        //     return (Parser::FailParser(FailParser), ParseResults::new_single(right_data, true));
        // }

        let combinator = &self.children[combinator_index];
        let (parser, parse_results) = profile!("seq first child parse", {
            combinator.parse(down_data, &bytes)
        });
        let done = parse_results.done();
        if done && parse_results.up_data_vec.is_empty() {
            // Shortcut
            return (SeqParser {
                parsers: vec![],
                combinators: &self.children,
                position: start_position + bytes.len(),
            }, parse_results);
        }
        let mut parsers: Vec<(usize, Box<dyn ParserTrait>)> = if done {
            vec![]
        } else {
            vec![(combinator_index, parser)]
        };
        let mut final_up_data: VecY<UpData>;
        let mut next_up_data_vec: VecY<UpData>;
        if combinator_index + 1 < self.children.len() {
            next_up_data_vec = parse_results.up_data_vec;
            final_up_data = VecY::new();
        } else {
            next_up_data_vec = VecY::new();
            final_up_data = parse_results.up_data_vec;
        }

        combinator_index += 1;

        let mut helper = |down_data: DownData, combinator_index: usize| {
            let offset = down_data.right_data.right_data_inner.get_fields1().position - start_position;
            let combinator = &self.children[combinator_index];
            let (parser, parse_results) = profile!("seq other child parse", {
                combinator.parse(down_data, &bytes[offset..])
            });
            if !parse_results.done() {
                parsers.push((combinator_index, parser));
            }
            if combinator_index + 1 < self.children.len() {
                parse_results.up_data_vec
            } else {
                final_up_data.extend(parse_results.up_data_vec);
                VecY::new()
            }
        };

        while combinator_index < self.children.len() && !next_up_data_vec.is_empty() {
            if next_up_data_vec.len() == 1 {
                let up_data = next_up_data_vec.pop().unwrap();
                next_up_data_vec = helper(DownData { right_data: up_data.right_data }, combinator_index);
            } else {
                let mut next_next_up_data_vec = VecY::new();
                for up_data in next_up_data_vec {
                    next_next_up_data_vec.extend(helper(DownData { right_data: up_data.right_data }, combinator_index));
                }
                next_up_data_vec = next_next_up_data_vec;
            }
            combinator_index += 1;
        }

        if parsers.is_empty() {
            return (SeqParser {
                parsers: vec![],
                combinators: &self.children,
                position: start_position + bytes.len(),
            }, ParseResults::new(final_up_data, true));
        }

        let parser = SeqParser {
            parsers,
            combinators: &self.children,
            position: start_position + bytes.len(),
        };

        let parse_results = ParseResults::new(final_up_data, false);

        (parser, parse_results)
    }
}

impl BaseCombinatorTrait for Seq<'_> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        for child in self.children.iter() {
            f(child);
        }
    }
}

impl ParserTrait for SeqParser<'_> {
    fn get_u8set(&self) -> U8Set {
        let mut u8set = U8Set::none();
        for (_, parser) in &self.parsers {
            u8set = u8set.union(&parser.get_u8set());
        }
        u8set
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        profile!("SeqParser::parse", {
        let mut final_up_data: VecY<UpData> = VecY::new();
        // let mut parser_initialization_queue: BTreeMap<usize, RightDataSquasher> = BTreeMap::new();
        let mut parser_initialization_queue: BTreeMap<usize, Vec<UpData>> = BTreeMap::new();

        // Eliminate duplicate parsers
        // if self.parsers.len() > 10 {
        //     self.parsers = std::mem::take(&mut self.parsers).into_iter().collect::<HashSet<_>>().into_iter().collect();
        // }

        profile!("SeqParser::parse part 1", {
        self.parsers.retain_mut(|(combinator_index, parser)| {
            let parse_results = profile!("SeqParser::parse child Parser::parse", {
                parser.parse(bytes)
            });
            let done = parse_results.done();
            if *combinator_index + 1 < self.combinators.len() {
                profile!("SeqParser::parse extend parser_initialization_queue", {
                    parser_initialization_queue.entry(*combinator_index + 1).or_default().extend(parse_results.up_data_vec);
                });
            } else {
                profile!("SeqParser::parse extend final_up_data", {
                    final_up_data.extend(parse_results.up_data_vec);
                });
            }
            !done
        });
        });

        profile!("SeqParser::parse part 2", {
        while let Some((combinator_index, up_data_vec)) = parser_initialization_queue.pop_first() {
            // for right_data in right_data_squasher.finish() {
            for up_data in up_data_vec {
                let offset = up_data.right_data.right_data_inner.get_fields1().position - self.position;
                let combinator = &self.combinators[combinator_index];
                let (parser, parse_results) = profile!("SeqParser::parse child Combinator::parse", {
                    combinator.parse(DownData { right_data: up_data.right_data }, &bytes[offset..])
                });
                if !parse_results.done() {
                    self.parsers.push((combinator_index, parser));
                }
                if combinator_index + 1 < self.combinators.len() {
                    profile!("SeqParser::parse extend parser_initialization_queue", {
                        parser_initialization_queue.entry(combinator_index + 1).or_default().extend(parse_results.up_data_vec);
                    });
                } else {
                    final_up_data.extend(parse_results.up_data_vec);
                }
            }
        }
        });

        self.position += bytes.len();

        ParseResults::new(final_up_data, self.parsers.is_empty())
            })
    }
}

pub fn _seq(v: Vec<Box<dyn DynCombinatorTrait>>) -> impl CombinatorTrait {
    profile_internal("seq", Seq {
        children: v.into_iter().collect(),
        start_index: 0,
    })
}