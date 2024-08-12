use std::rc::Rc;

use crate::{ChoiceParser, Opt, Parser, Repeat1, Repeat1Parser, Seq, SeqParser};

impl Parser {
    pub fn transpose(mut self) -> Self {
        // Converts a parser into a form where the outer expression is a choice between sequences that begin with a terminal.
        //
        // Here, a 'terminal' is defined (loosely) as any combinator that is not a sequence or a choice.
        //
        // Example transpositions:
        //
        // seq(seq(a, b, c), d, e)) =>
        // choice(seq(a, seq(b, c), seq(d, e))))
        //
        // seq(choice(a, b, c), d, e)) =>
        // choice(seq(a, seq(d, e)), seq(b, seq(d, e)), seq(c, seq(d, e)))
        //
        // There are four patterns we support:
        //
        // choice choice
        // choice seq
        // seq seq
        // seq choice
        //
        // i.e.
        // choice parsers where
        match &self {
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
                                // let self_str = format!("{:?}", self);
                                // let transposed_str = format!("{:?}", transposed);
                                // let self_str_truncated = self_str.chars().take(50).collect::<String>();
                                // let transposed_str_truncated = transposed_str.chars().take(50).collect::<String>();
                                // println!("transpose! {:?} => {:?}", self_str_truncated, transposed_str_truncated);
                                // println!("transposing seq");
                                transposed
                            }
                            _ => self,
                        }
                    }
                    _ => self,
                }
            }
            Parser::ChoiceParser(ChoiceParser { parsers, greedy }) => {
                if parsers.len() == 1 {
                    // println!("transposing choice");
                    parsers.first().unwrap().clone()
                } else {
                    self
                }
            }
            Parser::Repeat1Parser(Repeat1Parser { a, a_parsers, position, greedy }) => {
                if a_parsers.len() == 1 {
                    // println!("transposing repeat1");
                    let first = a.as_ref().clone();
                    let second = Opt { inner: Box::new(Repeat1 { a: a.clone(), greedy: *greedy }.into()), greedy: *greedy }.into();
                    Parser::SeqParser(SeqParser {
                        parsers: vec![(0, a_parsers.first().unwrap().clone())],
                        combinators: Rc::new(vec![first, second]),
                        position: *position,
                    })
                } else {
                    self
                }
            }
            _ => self,
        }
    }
}