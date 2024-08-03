use crate::{choice, Combinator, CombinatorTrait, eat_bytes, eps, mutate_right_data, Parser, ParseResults, ParserTrait, RightData, seq, U8Set};
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
                                true
                            }),
                            // Or match the indent and continue
                            seq!(eat_bytes(&indents[0]), make_combinator(&indents[1..], total_indents))
                        ).into()
                    }
                }
                let combinator = make_combinator(&right_data.indents, right_data.indents.len());
                let (parser, parse_results) = combinator.parser_with_steps(right_data, bytes);

                (IndentCombinatorParser::DentParser(Box::new(parser)), parse_results)
            }
            IndentCombinator::Indent if right_data.dedents == 0 => {
                if !bytes.is_empty() && bytes[0] != b' ' {
                    (IndentCombinatorParser::Done, ParseResults {
                        right_data_vec: vec![],
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
                        done: false,
                    })
                }
            }
            IndentCombinator::Dedent if right_data.dedents > 0 => {
                right_data.dedents -= 1;
                (IndentCombinatorParser::Done, ParseResults {
                    right_data_vec: vec![right_data],
                    done: true,
                })
            }
            IndentCombinator::AssertNoDedents if right_data.dedents == 0 => {
                (IndentCombinatorParser::Done, ParseResults {
                    right_data_vec: vec![right_data],
                    done: true,
                })
            }
            _ => (IndentCombinatorParser::Done, ParseResults::empty_finished()),
        };
        (Parser::IndentCombinatorParser(parser), parse_results)
    }
}

impl ParserTrait for IndentCombinatorParser {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResults::empty_unfinished();
        }

        let mut right_data_vec = Vec::new();
        let mut done = false;

        for &byte in bytes {
            match self {
                IndentCombinatorParser::DentParser(parser) => {
                    let ParseResults { right_data_vec: mut new_right_data_vec, done: new_done } = parser.steps(&[byte]);
                    right_data_vec.append(&mut new_right_data_vec);
                    done = new_done;
                    if done {
                        break;
                    }
                }
                IndentCombinatorParser::IndentParser(maybe_right_data) => {
                    if byte == b' ' {
                        let mut right_data = maybe_right_data.as_mut().unwrap();
                        right_data.position += 1;
                        right_data.indents.last_mut().unwrap().push(byte);
                        right_data_vec.push(right_data.clone());
                    } else {
                        maybe_right_data.take();
                        done = true;
                        break;
                    }
                }
                IndentCombinatorParser::Done => {
                    done = true;
                    break;
                }
            }
        }

        ParseResults {
            right_data_vec,
            done,
        }
    }

    fn next_u8set(&self, bytes: &[u8]) -> U8Set {
        match self {
            IndentCombinatorParser::DentParser(parser) => parser.next_u8set(bytes),
            IndentCombinatorParser::IndentParser(Some(right_data)) => {
                let mut u8set = U8Set::from_chars(" ");
                if bytes.starts_with(&right_data.indents.last().unwrap()) {
                    u8set.insert(b'\n');
                }
                u8set
            }
            IndentCombinatorParser::IndentParser(None) |
            IndentCombinatorParser::Done => U8Set::none(),
        }
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
