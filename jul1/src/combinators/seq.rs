// src/combinators/seq.rs
use crate::{dumb_one_shot_parse, BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseResults};
use crate::RightData;
use std::any::Any;
use std::rc::Rc;
use std::collections::{BTreeMap, HashSet};
use std::hash::{Hash, Hasher};
use lru::DefaultHasher;
use crate::{CombinatorTrait, eps, FailParser, ParseResults, ParserTrait, profile, profile_internal, ParseResultTrait, RightDataSquasher, Squash, U8Set, VecY};
use crate::VecX;

macro_rules! profile {
    ($name:expr, $expr:expr) => {
        $expr
    };
}

#[derive(Debug)]
pub struct Seq {
    pub(crate) children: VecX<Box<dyn DynCombinatorTrait>>,
    pub(crate) start_index: usize,
}

#[derive(Debug)]
pub struct SeqParser<'a> {
    pub(crate) parsers: Vec<(usize, Box<dyn ParserTrait>)>,
    pub(crate) combinators: &'a VecX<Box<dyn DynCombinatorTrait>>,
    pub(crate) position: usize,
}

impl CombinatorTrait for Seq {
    type Parser<'a> = SeqParser<'a>;

    fn one_shot_parse(&self, mut right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let start_position = right_data.right_data_inner.fields1.position;
        for combinator in self.children.iter() {
            let offset = right_data.right_data_inner.fields1.position - start_position;
            right_data = combinator.one_shot_parse(right_data, &bytes[offset..])?;
        }
        Ok(right_data)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let start_position = right_data.right_data_inner.fields1.position;

        let mut combinator_index = self.start_index;

        // if combinator_index >= self.children.len() {
        //     return (Parser::FailParser(FailParser), ParseResults::new_single(right_data, true));
        // }

        let combinator = &self.children[combinator_index];
        let (parser, parse_results) = profile!("seq first child parse", {
            combinator.parse(right_data, &bytes)
        });
        let done = parse_results.done();
        if done && parse_results.right_data_vec.is_empty() {
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
        let mut final_right_data: VecY<RightData>;
        let mut next_right_data_vec: VecY<RightData>;
        if combinator_index + 1 < self.children.len() {
            next_right_data_vec = parse_results.right_data_vec;
            final_right_data = VecY::new();
        } else {
            next_right_data_vec = VecY::new();
            final_right_data = parse_results.right_data_vec;
        }

        combinator_index += 1;

        let mut helper = |right_data: RightData, combinator_index: usize| {
            let offset = right_data.right_data_inner.fields1.position - start_position;
            let combinator = &self.children[combinator_index];
            let (parser, parse_results) = profile!("seq other child parse", {
                combinator.parse(right_data, &bytes[offset..])
            });
            if !parse_results.done() {
                parsers.push((combinator_index, parser));
            }
            if combinator_index + 1 < self.children.len() {
                parse_results.right_data_vec
            } else {
                final_right_data.extend(parse_results.right_data_vec);
                VecY::new()
            }
        };

        while combinator_index < self.children.len() && !next_right_data_vec.is_empty() {
            if next_right_data_vec.len() == 1 {
                let right_data = next_right_data_vec.pop().unwrap();
                next_right_data_vec = helper(right_data, combinator_index);
            } else {
                let mut next_next_right_data_vec = VecY::new();
                for right_data in next_right_data_vec {
                    next_next_right_data_vec.extend(helper(right_data, combinator_index));
                }
                next_right_data_vec = next_next_right_data_vec;
            }
            combinator_index += 1;
        }

        if parsers.is_empty() {
            return (SeqParser {
                parsers: vec![],
                combinators: &self.children,
                position: start_position + bytes.len(),
            }, ParseResults::new(final_right_data, true));
        }

        let parser = SeqParser {
            parsers,
            combinators: &self.children,
            position: start_position + bytes.len(),
        };

        let parse_results = ParseResults::new(final_right_data, false);

        (parser, parse_results)
    }
}

impl BaseCombinatorTrait for Seq {
    fn as_any(&self) -> &dyn std::any::Any {
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
        let mut final_right_data: VecY<RightData> = VecY::new();
        let mut parser_initialization_queue: BTreeMap<usize, RightDataSquasher> = BTreeMap::new();

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
                    parser_initialization_queue.entry(*combinator_index + 1).or_default().extend(parse_results.right_data_vec);
                });
            } else {
                profile!("SeqParser::parse extend final_right_data", {
                    final_right_data.extend(parse_results.right_data_vec);
                });
            }
            !done
        });
        });

        profile!("SeqParser::parse part 2", {
        while let Some((combinator_index, right_data_squasher)) = parser_initialization_queue.pop_first() {
            for right_data in right_data_squasher.finish() {
                let offset = right_data.right_data_inner.fields1.position - self.position;
                let combinator = &self.combinators[combinator_index];
                let (parser, parse_results) = profile!("SeqParser::parse child Combinator::parse", {
                    combinator.parse(right_data, &bytes[offset..])
                });
                if !parse_results.done() {
                    self.parsers.push((combinator_index, parser));
                }
                if combinator_index + 1 < self.combinators.len() {
                    profile!("SeqParser::parse extend parser_initialization_queue", {
                        parser_initialization_queue.entry(combinator_index + 1).or_default().extend(parse_results.right_data_vec);
                    });
                } else {
                    final_right_data.extend(parse_results.right_data_vec);
                }
            }
        }
        });

        self.position += bytes.len();

        ParseResults::new(final_right_data, self.parsers.is_empty())
            })
    }
}

pub fn _seq(v: Vec<Box<dyn DynCombinatorTrait>>) -> impl CombinatorTrait {
    profile_internal("seq", Seq {
        children: v.into_iter().collect(),
        start_index: 0,
    })
}

// #[macro_export]
// macro_rules! seq {
//     ($($expr:expr),+ $(,)?) => {
//         $crate::_seq(vec![$($expr.into()),+])
//     };
// }
//
// impl From<Seq> for Combinator {
//     fn from(value: Seq) -> Self {
//         Combinator::Seq(value)
//     }
// }