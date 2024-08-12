use std::rc::Rc;
use crate::{ChoiceParser, Combinator, Parser, ParseResults, Seq, SeqParser};

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
                                let second: Combinator = Seq { children: combinators2.clone(), start_index: i2 + 1 }.into();
                                let third = Seq { children: combinators.clone(), start_index: i + 1 }.into();
                                let transposed = Parser::SeqParser(SeqParser {
                                    parsers: vec![(0, child.clone())],
                                    combinators: Rc::new(vec![first, second, third]),
                                    position: *position2,
                                });
                                let self_str = format!("{:?}", self);
                                let transposed_str = format!("{:?}", transposed);
                                let self_str_truncated = self_str.chars().take(100).collect::<String>();
                                let transposed_str_truncated = transposed_str.chars().take(100).collect::<String>();
                                println!("transpose! {:?} => {:?}", self_str_truncated, transposed_str_truncated);
                                return transposed;

                            }
                            _ => self,
                        }
                    }
                    _ => self,
                }
            }
            Parser::ChoiceParser(ChoiceParser { parsers, greedy }) => {
                self
            }
            _ => self,
        }
    }
}
