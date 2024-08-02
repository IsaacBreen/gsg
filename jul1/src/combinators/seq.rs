use std::rc::Rc;

use crate::{Combinator, CombinatorTrait, eps, Parser, ParseResults, ParserTrait, RightData, Squash};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq {
    pub(crate) children: Vec<Rc<Combinator>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeqParser {
    pub(crate) children: Vec<(Rc<Combinator>, Vec<Parser>)>,
    pub(crate) position: usize,
}

impl CombinatorTrait for Seq {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let mut children = Vec::new();
        let mut current_right_data = vec![right_data.clone()];
        let mut all_up_data = Vec::new();
        let mut all_done = true;

        for child in self.children.iter() {
            let mut new_parsers = Vec::new();
            let mut new_right_data = Vec::new();

            for right_data in current_right_data.into_iter() {
                let (parser, ParseResults { right_data_vec, up_data_vec, done }) = child.parser(right_data);
                if !done {
                    new_parsers.push(parser);
                    all_done = false;
                }
                new_right_data.extend(right_data_vec);
                all_up_data.extend(up_data_vec);
            }

            children.push((child.clone(), new_parsers));
            current_right_data = new_right_data;
        }

        let parser = Parser::SeqParser(SeqParser { children, position: right_data.position });

        let parse_results = ParseResults {
            right_data_vec: current_right_data,
            up_data_vec: all_up_data,
            done: all_done,
        };

        (parser.into(), parse_results)
    }

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut children = Vec::new();
        let mut current_right_data = vec![right_data.clone()];
        let mut all_up_data = Vec::new();
        let mut all_done = true;
        let start_position = right_data.position;

        for (i, child) in self.children.iter().enumerate() {
            let mut new_parsers = Vec::new();
            let mut new_right_data = Vec::new();

            for right_data in current_right_data.into_iter() {
                let offset = right_data.position - start_position;
                let (parser, ParseResults { right_data_vec, up_data_vec, done }) = child.parser_with_steps(right_data, &bytes[offset..]);
                if !done {
                    new_parsers.push(parser);
                    all_done = false;
                }
                new_right_data.extend(right_data_vec);
                all_up_data.extend(up_data_vec);
            }

            children.push((child.clone(), new_parsers));
            current_right_data = new_right_data;
        }

        let parser = Parser::SeqParser(SeqParser { children, position: right_data.position });

        let parse_results = ParseResults {
            right_data_vec: current_right_data,
            up_data_vec: all_up_data,
            done: all_done,
        };

        (parser.into(), parse_results)
    }
}

impl ParserTrait for SeqParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut current_right_data = vec![];
        let mut all_up_data = Vec::new();
        let mut all_done = true;

        for (i, (combinator, parsers)) in self.children.iter_mut().enumerate() {
            let mut next_right_data = Vec::new();

            parsers.retain_mut(|mut parser| {
                let ParseResults { right_data_vec, up_data_vec, done } = parser.step(c);
                if !done {
                    all_done = false;
                }
                next_right_data.extend(right_data_vec);
                all_up_data.extend(up_data_vec);
                !done
            });

            for right_data in current_right_data.into_iter() {
                let (parser, ParseResults { right_data_vec, up_data_vec, done }) = combinator.parser(right_data);
                if !done {
                    parsers.push(parser);
                    all_done = false;
                }
                next_right_data.extend(right_data_vec);
                all_up_data.extend(up_data_vec);
            }

            current_right_data = next_right_data.squashed();
        }

        self.position += 1;

        ParseResults {
            right_data_vec: current_right_data,
            up_data_vec: all_up_data,
            done: all_done,
        }
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        let mut current_right_data: Vec<RightData> = vec![];
        let mut all_up_data = Vec::new();
        let mut all_done = true;

        for (combinator, parsers) in &mut self.children {
            let mut next_right_data = Vec::new();

            parsers.retain_mut(|mut parser| {
                let ParseResults { right_data_vec, up_data_vec, done } = parser.steps(bytes);
                if !done {
                    all_done = false;
                }
                next_right_data.extend(right_data_vec);
                all_up_data.extend(up_data_vec);
                !done
            });

            for right_data in current_right_data.into_iter() {
                let offset = right_data.position - self.position;
                let (parser, ParseResults { right_data_vec, up_data_vec, done }) = combinator.parser_with_steps(right_data, &bytes[offset..]);
                next_right_data.extend(right_data_vec);
                all_up_data.extend(up_data_vec);
                if !done {
                    parsers.push(parser);
                    all_done = false;
                }
            }

            current_right_data = next_right_data.squashed();
        }

        self.position += bytes.len();

        ParseResults {
            right_data_vec: current_right_data,
            up_data_vec: all_up_data,
            done: all_done,
        }
    }
}

pub fn _seq(v: Vec<Combinator>) -> Combinator {
    Seq {
        children: v.into_iter().map(Rc::new).collect(),
    }.into()
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
