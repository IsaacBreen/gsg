use std::rc::Rc;
use std::collections::BTreeMap;

use crate::{Combinator, CombinatorTrait, eps, Parser, ParseResults, ParserTrait, profile_internal, RightData, Squash, U8Set};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq {
    pub(crate) children: Rc<Vec<Combinator>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeqParser {
    pub(crate) parsers: BTreeMap<usize, Vec<Parser>>,
    pub(crate) combinators: Rc<Vec<Combinator>>,
    pub(crate) position: usize,
}

impl CombinatorTrait for Seq {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let start_position = right_data.position;

        let mut parsers: BTreeMap<usize, Vec<Parser>> = BTreeMap::new();
        let mut final_right_data: Vec<RightData> = vec![];
        let mut parser_initialization_queue: BTreeMap<usize, Vec<RightData>> = BTreeMap::new();
        parser_initialization_queue.insert(0, vec![right_data]);

        while let Some((combinator_index, mut right_data_vec)) = parser_initialization_queue.pop_first() {
            right_data_vec.squash();
            for right_data in right_data_vec {
                let offset = right_data.position - start_position;
                let combinator = &self.children[combinator_index];
                let (parser, parse_results) = combinator.parse(right_data, &bytes[offset..]);
                if combinator_index + 1 < self.children.len() {
                    parser_initialization_queue.entry(combinator_index + 1).or_default().extend(parse_results.right_data_vec);
                } else {
                    final_right_data.extend(parse_results.right_data_vec);
                }
                if !parse_results.done {
                    parsers.entry(combinator_index).or_default().push(parser);
                }
            }
        }

        let parsers_is_empty = parsers.is_empty();

        let parser = Parser::SeqParser(SeqParser {
            parsers,
            combinators: self.children.clone(),
            position: start_position + bytes.len(),
        });

        let parse_results = ParseResults {
            right_data_vec: final_right_data,
            done: parsers_is_empty,
        };

        (parser.into(), parse_results)
    }
}

impl ParserTrait for SeqParser {
    fn get_u8set(&self) -> U8Set {
        let mut u8set = U8Set::none();
        for (_, parser_vec) in &self.parsers {
            for parser in parser_vec {
                u8set = u8set.union(&parser.get_u8set());
            }
        }
        u8set
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut final_right_data: Vec<RightData> = vec![];
        let mut parser_initialization_queue: BTreeMap<usize, Vec<RightData>> = BTreeMap::new();

        self.parsers.retain(|&combinator_index, parser_vec| {
            parser_vec.retain_mut(|parser| {
                let ParseResults { right_data_vec, done } = parser.parse(bytes);
                if combinator_index + 1 < self.combinators.len() {
                    parser_initialization_queue.entry(combinator_index + 1).or_default().extend(right_data_vec);
                } else {
                    final_right_data.extend(right_data_vec);
                }
                !done
            });
            !parser_vec.is_empty()
        });

        while let Some((combinator_index, right_data_vec)) = parser_initialization_queue.pop_first() {
            for right_data in right_data_vec {
                let offset = right_data.position - self.position;
                let combinator = &self.combinators[combinator_index];
                let (parser, parse_results) = combinator.parse(right_data, &bytes[offset..]);
                if combinator_index + 1 < self.combinators.len() {
                    parser_initialization_queue.entry(combinator_index + 1).or_default().extend(parse_results.right_data_vec);
                } else {
                    final_right_data.extend(parse_results.right_data_vec);
                }
                if !parse_results.done {
                    self.parsers.entry(combinator_index).or_default().push(parser);
                }
            }
        }

        self.position += bytes.len();

        ParseResults {
            right_data_vec: final_right_data,
            done: self.parsers.is_empty(),
        }
    }
}

pub fn _seq(v: Vec<Combinator>) -> Combinator {
    profile_internal("seq", Seq {
        children: Rc::new(v),
    })
}

#[macro_export]
macro_rules! seq {
    ($($expr:expr),+ $(,)?) => {
        $crate::_seq(vec![$($expr.into()),+])
    };
}

impl From<Seq> for Combinator {
    fn from(value: Seq) -> Self {
        Combinator::Seq(value)
    }
}