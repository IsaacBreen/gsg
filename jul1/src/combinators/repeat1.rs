use crate::UnambiguousParseResults;
use std::any::Any;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, opt_greedy, Parser, ParseResults, ParserTrait, profile_internal, RightDataSquasher, Squash, U8Set, VecY, vecy, Opt, Seq2, IntoCombinator, fail};
use crate::opt;
use crate::parse_state::{RightData, ParseResultTrait};
use crate::VecX;

#[derive(Debug)]
pub struct Repeat1<T: CombinatorTrait> {
    pub(crate) a: T,
    pub(crate) greedy: bool,
}

#[derive(Debug)]
pub struct Repeat1Parser<'a> {
    // TODO: store a_parsers in a Vec<Vec<Parser>> where the index of each inner vec is the repetition count of those parsers. That way, we can easily discard earlier parsers when we get a decisively successful parse result.
    pub(crate) a: &'a dyn CombinatorTrait,
    pub(crate) a_parsers: Vec<Parser<'a>>,
    pub(crate) position: usize,
    pub(crate) greedy: bool,
}

impl<T: CombinatorTrait + 'static> CombinatorTrait for Repeat1<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(&self.a);
    }

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        todo!()
    }

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let start_position = right_data.right_data_inner.fields1.position;
        let (parser, parse_results) = self.a.parse(right_data, bytes);
        if parse_results.done() && parse_results.right_data_vec.is_empty() {
            // Shortcut
            return (parser, parse_results);
        } else if parse_results.right_data_vec.is_empty() {
            return (Parser::Repeat1Parser(Repeat1Parser {
                a: &self.a,
                a_parsers: vec![parser],
                position: start_position + bytes.len(),
                greedy: self.greedy
            }), ParseResults::new(vecy![], false));
        }
        let mut parsers = if parse_results.done() {
            vec![]
        } else {
            vec![parser]
        };
        let mut all_prev_succeeded_decisively = parse_results.succeeds_decisively();
        let mut right_data_vec = parse_results.right_data_vec;

        let mut next_right_data = right_data_vec.clone();
        while next_right_data.len() > 0 {
            for new_right_data in std::mem::take(&mut next_right_data) {
                let offset = new_right_data.right_data_inner.fields1.position - start_position;
                let (parser, parse_results) = self.a.parse(new_right_data, &bytes[offset..]);
                if !parse_results.done() {
                    parsers.push(parser);
                }
                all_prev_succeeded_decisively &= parse_results.succeeds_decisively();
                if self.greedy && all_prev_succeeded_decisively {
                    right_data_vec.clear();
                    parsers.clear();
                }
                // if !(self.greedy && parse_results.succeeds_decisively()) && parse_results.right_data_vec.len() > 0 && right_data_vec.len() > 0 {
                //     println!("parse_results: {:?}", parse_results);
                // }
                next_right_data.extend(parse_results.right_data_vec);
            }
            if !right_data_vec.is_empty() && !next_right_data.is_empty() {
                let end_pos = start_position + bytes.len();
                let pos1 = right_data_vec[0].right_data_inner.fields1.position;
                let pos2 = next_right_data[0].right_data_inner.fields1.position;
                if end_pos < pos1 + 1000 || end_pos < pos2 + 1000 {
                    right_data_vec.clear();
                }
            }
            right_data_vec.extend(next_right_data.clone());
        }

        let done = parsers.is_empty();

        (
            Parser::Repeat1Parser(Repeat1Parser {
                a: &self.a,
                a_parsers: parsers,
                position: start_position + bytes.len(),
                greedy: self.greedy
            }),
            ParseResults::new(right_data_vec, done)
        )
    }
}

impl ParserTrait for Repeat1Parser<'_> {
    fn get_u8set(&self) -> U8Set {
        if self.a_parsers.is_empty() {
            U8Set::none()
        } else {
            self.a_parsers.iter().map(|p| p.get_u8set()).reduce(|acc, e| acc | e).unwrap_or(U8Set::none())
        }
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut right_data_as = VecY::new();
        // let mut right_data_as: BTreeMap<usize, RightDataSquasher> = BTreeMap::new();

        for mut a_parser in std::mem::take(&mut self.a_parsers) {
            let parse_results = a_parser.parse(bytes);
            if !parse_results.done() {
                self.a_parsers.push(a_parser);
            }
            // right_data_as.entry(parse_results.right_data_vec.len()).or_default().extend(parse_results.right_data_vec);
            right_data_as.extend(parse_results.right_data_vec);
        }

        right_data_as.squash();

        let mut i = 0;
        while i < right_data_as.len() {
            let right_data_a = right_data_as[i].clone();
            let offset = right_data_a.right_data_inner.fields1.position - self.position;
            let (a_parser, parse_results) = self.a.parse(right_data_a, &bytes[offset..]);
            if !parse_results.done() {
                self.a_parsers.push(a_parser);
            }
            // right_data_as.entry(i).or_default().extend(parse_results.right_data_vec);
            right_data_as.extend(parse_results.right_data_vec);
            i += 1;
        }

        self.position += bytes.len();

        ParseResults::new(right_data_as, self.a_parsers.is_empty())
    }
}

pub fn repeat1<T: IntoCombinator>(a: T)-> impl CombinatorTrait where T::Output: 'static {
    profile_internal("repeat1", Repeat1 {
        a: a.into_combinator(),
        greedy: false,
    })
}

pub fn repeat1_greedy<T: IntoCombinator>(a: T)-> impl CombinatorTrait where T::Output: 'static {
    profile_internal("repeat1_greedy", Repeat1 {
        a: a.into_combinator(),
        greedy: true,
    })
}

pub fn repeat0(a: impl CombinatorTrait + 'static)-> impl CombinatorTrait {
    Opt { inner: Repeat1 { a, greedy: false }, greedy: false }
}

pub fn repeat0_greedy(a: impl CombinatorTrait + 'static)-> impl CombinatorTrait {
    Opt { inner: Repeat1 { a, greedy: true }, greedy: true }
}

pub fn seprep1(a: impl CombinatorTrait + Clone, b: impl CombinatorTrait)-> impl CombinatorTrait {
    // Seq2 {
    //     c0: Box::new(a.clone()),
    //     c1: Opt { inner: Repeat1 { a: Seq2 {
    //         c0: b,
    //         c1: a
    //     }.into(), greedy: false }, greedy: false }.into(),
    // }
    todo!("fix this");
    fail()
}

// impl From<Repeat1<Combinator>> for Combinator {
//     fn from(value: Repeat1<Combinator>) -> Self {
//         value
//     }
// }
