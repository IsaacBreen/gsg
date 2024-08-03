use crate::{choice, Combinator, CombinatorTrait, eat_bytes, eps, mutate_right_data, Parser, ParseResults, ParserTrait, RightData, seq, U8Set, UpData};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndentCombinator {
    Dent,
    Indent,
    Dedent,
    AssertNoDedents,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndentCombinatorParser {
    DentParser(Box<Parser>),
    IndentParser(Option<RightData>),
    Done,
}

impl CombinatorTrait for IndentCombinator {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        self.parser_with_steps(right_data, &[])
    }

    fn parser_with_steps(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, parse_results) = match self {
            IndentCombinator::Dent if right_data.dedents == 0 => {
                fn make_combinator(mut indents: &[Vec<u8>], total_indents: usize) -> Combinator { // TODO: Make this a macro
                    if indents.is_empty() {
                        eps().into()
                    } else {
                        let dedents = indents.len();
                        choice!(
                            // Exit here and register dedents
                            mutate_right_data(move |right_data: &mut RightData| {
                                right_data.dedents = dedents;
                                // Remove the last `dedents` indents from the indent stack
                                right_data.indents.truncate(right_data.indents.len() - dedents);
                                // println!("Registering {} dedents. Right data: {:?}", dedents, right_data);
                                true
                            }),
                            // Or match the indent and continue
                            seq!(eat_bytes(&indents[0]), make_combinator(&indents[1..], total_indents))
                        ).into()
                    }
                }
                // println!("Made dent parser with right_data: {:?}", right_data);
                let combinator = make_combinator(&right_data.indents, right_data.indents.len());
                let (parser, parse_results) = combinator.parser_with_steps(right_data, bytes);

                (IndentCombinatorParser::DentParser(Box::new(parser)), parse_results)
            }
            IndentCombinator::Indent if right_data.dedents == 0 => {
                if !bytes.is_empty() && bytes[0] != b' ' {
                    (IndentCombinatorParser::Done, ParseResults {
                        right_data_vec: vec![],
                        up_data_vec: vec![],
                        done: true,
                    })
                } else {
                    // Consume as many spaces as possible
                    let mut i = 0;
                    while i < bytes.len() && bytes[i] == b' ' {
                        i += 1;
                    }
                    right_data.position += i;
                    right_data.indents.push(bytes[0..i].to_vec());
                    (IndentCombinatorParser::IndentParser(Some(right_data.clone())), ParseResults {
                        right_data_vec: vec![right_data],
                        up_data_vec: vec![UpData { u8set: U8Set::from_chars(" ") }],
                        done: false,
                    })
                }
            }
            IndentCombinator::Dedent if right_data.dedents > 0 => {
                right_data.dedents -= 1;
                // println!("Decremented dedents to {}", right_data.dedents);
                (IndentCombinatorParser::Done, ParseResults {
                    right_data_vec: vec![right_data],
                    up_data_vec: vec![],
                    done: true,
                })
            }
            IndentCombinator::AssertNoDedents if right_data.dedents == 0 => {
                (IndentCombinatorParser::Done, ParseResults::finished(right_data))
            }
            _ => (IndentCombinatorParser::Done, ParseResults::empty_finished()),
        };        (Parser::IndentCombinatorParser(parser), parse_results)
    }
}

impl ParserTrait for IndentCombinatorParser {
    fn step(&mut self, c: u8) -> ParseResults {
        match self {
            IndentCombinatorParser::DentParser(parser) => parser.step(c),
            IndentCombinatorParser::IndentParser(maybe_right_data) => {
                if c == b' ' {
                    let mut right_data = maybe_right_data.as_mut().unwrap();
                    right_data.position += 1;
                    right_data.indents.last_mut().unwrap().push(c);
                    // println!("Indent parser: {:?}", right_data);
                    ParseResults {
                        right_data_vec: vec![right_data.clone()],
                        up_data_vec: vec![UpData { u8set: U8Set::from_chars(" ") }],
                        done: false,
                    }
                } else {
                    // Fail. Purge the right data to poison the parser.
                    maybe_right_data.take();
                    ParseResults::empty_finished()
                }
            }
            IndentCombinatorParser::Done => ParseResults::empty_finished(),
        }
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }
        let mut right_data_vec = Vec::new();
        for i in 0..bytes.len() {
            let ParseResults { right_data_vec: mut new_right_data_vec, up_data_vec, done } = self.step(bytes[i]);
            right_data_vec.append(&mut new_right_data_vec);
            if done || i == bytes.len() - 1 {
                return ParseResults {
                    right_data_vec,
                    up_data_vec,
                    done,
                };
            }
        }
        unreachable!();
    }
}

pub fn dent() -> IndentCombinator {
    IndentCombinator::Dent
}

pub fn indent() -> IndentCombinator {
    IndentCombinator::Indent
}

pub fn dedent() -> IndentCombinator {
    IndentCombinator::Dedent
}

pub fn assert_no_dedents() -> IndentCombinator {
    IndentCombinator::AssertNoDedents
}

pub fn with_indent(a: impl Into<Combinator>) -> Combinator {
    seq!(indent(), a, dedent())
}

impl From<IndentCombinator> for Combinator {
    fn from(value: IndentCombinator) -> Self {
        Combinator::IndentCombinator(value)
    }
}
