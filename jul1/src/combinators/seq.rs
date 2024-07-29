use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, eps, Parser, ParseResults, ParserTrait, RightData};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq {
    children: Vec<Rc<Combinator>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeqParser {
    pub(crate) parsers: Vec<Vec<Parser>>,
    children: Vec<Rc<Combinator>>,
    right_data: RightData,
}

impl CombinatorTrait for Seq {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let mut parsers = Vec::new();
        let mut current_right_data = vec![right_data.clone()];
        let mut all_up_data = Vec::new();
        let mut all_done = true;

        for (i, child) in self.children.iter().enumerate() {
            let mut child_parsers = Vec::new();
            let mut next_right_data = Vec::new();

            for rd in current_right_data {
                let (parser, ParseResults { right_data_vec, up_data_vec, done }) = child.parser(rd);
                if !done {
                    child_parsers.push(parser);
                    all_done = false;
                }
                next_right_data.extend(right_data_vec);
                all_up_data.extend(up_data_vec);
            }

            parsers.push(child_parsers);
            if i < self.children.len() - 1 {
                current_right_data = next_right_data;
            } else {
                all_up_data.extend(next_right_data);
            }
        }

        let parser = Parser::SeqParser(SeqParser {
            parsers,
            children: self.children.clone(),
            right_data,
        });

        (parser, ParseResults {
            right_data_vec: all_up_data,
            up_data_vec: Vec::new(),
            done: all_done,
        })
    }
}

impl ParserTrait for SeqParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut all_right_data = Vec::new();
        let mut all_up_data = Vec::new();
        let mut all_done = true;

        for (i, parsers) in self.parsers.iter_mut().enumerate() {
            let mut next_parsers = Vec::new();
            let mut current_right_data = Vec::new();

            for parser in parsers.drain(..) {
                let ParseResults { right_data_vec, up_data_vec, done } = parser.step(c);
                if !done {
                    next_parsers.push(parser);
                    all_done = false;
                }
                current_right_data.extend(right_data_vec);
                all_up_data.extend(up_data_vec);
            }

            *parsers = next_parsers;

            if i < self.children.len() - 1 {
                for right_data in current_right_data {
                    let (new_parser, ParseResults { right_data_vec, up_data_vec, done }) =
                        self.children[i + 1].parser(right_data);
                    if !done {
                        self.parsers[i + 1].push(new_parser);
                        all_done = false;
                    }
                    if i == self.children.len() - 2 {
                        all_right_data.extend(right_data_vec);
                    } else {
                        current_right_data = right_data_vec;
                    }
                    all_up_data.extend(up_data_vec);
                }
            } else {
                all_right_data.extend(current_right_data);
            }
        }

        ParseResults {
            right_data_vec: all_right_data,
            up_data_vec: all_up_data,
            done: all_done,
        }
    }
}

pub fn _seq(v: Vec<Combinator>) -> Combinator {
    if v.is_empty() {
        eps().into()
    } else if v.len() == 1 {
        v[0].clone()
    } else {
        Seq {
            children: v.into_iter().map(Rc::new).collect(),
        }.into()
    }
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