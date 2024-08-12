use std::rc::Rc;

use crate::{ChoiceParser, Opt, Parser, Repeat1, Repeat1Parser, Seq, SeqParser};

impl Parser {
    pub fn transpose(&mut self) {
        match self {
            Parser::SeqParser(SeqParser { parsers, combinators, position }) => {
                match parsers.as_slice() {
                    [(i, Parser::SeqParser(SeqParser { parsers: parsers2, combinators: combinators2, position: position2 }))] => {
                        match parsers2.as_slice() {
                            [(i2, child)] => {
                                let first = combinators2[*i2].clone();
                                let second = Seq { children: combinators2.clone(), start_index: i2 + 1 }.into();
                                let third = Seq { children: combinators.clone(), start_index: i + 1 }.into();
                                let transposed = Parser::SeqParser(SeqParser {
                                    parsers: vec![(0, child.clone())],
                                    combinators: Rc::new(vec![first, second, third]),
                                    position: *position2,
                                });
                                // println!("transposing seq");
                                *self = transposed;
                                self.transpose();
                            }
                            _ => {},
                        }
                    }
                    _ => {},
                }
            }
            Parser::ChoiceParser(ChoiceParser { parsers, greedy }) => {
                if parsers.len() == 1 {
                    // println!("transposing choice");
                    *self = parsers.first().unwrap().clone();
                    self.transpose();
                }
            }
            Parser::Repeat1Parser(Repeat1Parser { a, a_parsers, position, greedy }) => {
                if a_parsers.len() == 1 {
                    // println!("transposing repeat1");
                    let first = a.as_ref().clone();
                    let second = Opt { inner: Box::new(Repeat1 { a: a.clone(), greedy: *greedy }.into()), greedy: *greedy }.into();
                    *self = Parser::SeqParser(SeqParser {
                        parsers: vec![(0, a_parsers.first().unwrap().clone())],
                        combinators: Rc::new(vec![first, second]),
                        position: *position,
                    });
                    self.transpose();
                }
            }
            Parser::EatU8Parser(_) => {}
            Parser::EatStringParser(_) => {}
            Parser::EpsParser(_) => {}
            Parser::FailParser(_) => {}
            Parser::CacheContextParser(_) => {}
            Parser::CachedParser(_) => {}
            Parser::IndentCombinatorParser(_) => {}
            Parser::EatByteStringChoiceParser(_) => {}
            Parser::ExcludeBytestringsParser(_) => {}
            Parser::ProfiledParser(_) => {},
            Parser::BruteForceParser(_) => {}
            Parser::ContinuationParser(_) => {}
            Parser::FastParserWrapper(_) => {}
        }
    }
}