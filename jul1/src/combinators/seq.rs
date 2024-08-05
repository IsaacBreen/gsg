use std::rc::Rc;
use std::collections::BTreeMap;

use crate::{Combinator, CombinatorTrait, eps, Parser, ParseResults, ParserTrait, profile, profile_internal, RightData, RightDataSquasher, Squash, U8Set};

macro_rules! profile {
    ($name:expr, $expr:expr) => {
        $expr
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq {
    pub(crate) children: Rc<smallvec::SmallVec<[Combinator; 2]>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeqParser {
    pub(crate) parsers: Vec<(usize, Parser)>,
    pub(crate) combinators: Rc<smallvec::SmallVec<[Combinator; 2]>>,
    pub(crate) position: usize,
}

impl CombinatorTrait for Seq {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let start_position = right_data.position;

        let mut parsers: Vec<(usize, Parser)> = vec![];
        let mut final_right_data: smallvec::SmallVec<[RightData; 1]> = smallvec::SmallVec::new();
        let mut parser_initialization_queue: Vec<(usize, smallvec::SmallVec<[RightData; 1]>)> = vec![(0, smallvec::smallvec![right_data])];

        while let Some((combinator_index, mut right_data_vec)) = parser_initialization_queue.pop() {
            for right_data in right_data_vec {
                let offset = right_data.position - start_position;
                let combinator = &self.children[combinator_index];
                let (parser, parse_results) = profile!("seq child parse", {
                    combinator.parse(right_data, &bytes[offset..])
                });
                if combinator_index + 1 < self.children.len() {
                    parser_initialization_queue.push((combinator_index + 1, parse_results.right_data_vec));
                } else {
                    final_right_data.extend(parse_results.right_data_vec);
                }
                if !parse_results.done {
                    parsers.push((combinator_index, parser));
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
        for (_, parser) in &self.parsers {
            u8set = u8set.union(&parser.get_u8set());
        }
        u8set
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut final_right_data: smallvec::SmallVec<[RightData; 1]> = smallvec::SmallVec::new();
        let mut parser_initialization_queue: smallvec::SmallVec<[(usize, smallvec::SmallVec<[RightData; 1]>); 1]> = smallvec::smallvec![];

        self.parsers.retain_mut(|(combinator_index, parser)| {
            let ParseResults { right_data_vec, done } = parser.parse(bytes);
            if *combinator_index + 1 < self.combinators.len() {
                parser_initialization_queue.push((*combinator_index + 1, right_data_vec));
            } else {
                final_right_data.extend(right_data_vec);
            }
            !done
        });

        while let Some((combinator_index, mut right_data_vec)) = parser_initialization_queue.pop() {
            for right_data in right_data_vec {
                let offset = right_data.position - self.position;
                let combinator = &self.combinators[combinator_index];
                let (parser, parse_results) = combinator.parse(right_data, &bytes[offset..]);
                if combinator_index + 1 < self.combinators.len() {
                    parser_initialization_queue.push((combinator_index + 1, parse_results.right_data_vec));
                } else {
                    final_right_data.extend(parse_results.right_data_vec);
                }
                if !parse_results.done {
                    self.parsers.push((combinator_index, parser));
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
        children: Rc::new(v.into()),
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
