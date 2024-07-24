use std::any::Any;
use crate::{brute_force, BruteForceFn, BruteForceParser, choice, Choice2, CombinatorTrait, DynCombinator, eat_bytes, eat_char_choice, eat_string, EatU8, Eps, eps, IntoCombinator, mutate_right_data, ParseResults, ParserTrait, repeat0, Repeat1, repeat1, RightData, seq, Seq2, Stats, U8Set, UpData};

#[derive(Debug, Clone, PartialEq)]
pub enum IndentCombinator {
    Dent,
    Indent,
    Dedent,
    AssertNoDedents,
}

pub enum IndentCombinatorParser {
    DentParser(Box<dyn ParserTrait>),
    IndentParser(Option<RightData>),
    Done,
}

impl CombinatorTrait for IndentCombinator {
    type Parser = IndentCombinatorParser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        // println!("Indents: {:?}", right_data.indents);
        match self {
            IndentCombinator::Dent if right_data.dedents == 0  => {
                fn make_combinator(mut indents: &[Vec<u8>], total_indents: usize) -> Box<DynCombinator> {
                    if indents.is_empty() {
                        return mutate_right_data(move |right_data: &mut RightData| {
                                // println!("Done with {} dedents", total_indents);
                                true
                            }).into_box_dyn();
                    } else {
                        let dedents = indents.len();
                        choice!(
                            // Exit here and register dedents
                            mutate_right_data(move |right_data: &mut RightData| {
                                // println!("Registering {} dedents", dedents);
                                right_data.dedents = dedents;
                                // Remove the last `dedents` indents from the indent stack
                                right_data.indents.truncate(right_data.indents.len() - dedents);
                                true
                            }),
                            // Or match the indent and continue
                            seq!(eat_bytes(&indents[0]), make_combinator(&indents[1..], total_indents))
                        ).into_box_dyn()
                    }
                }
                let combinator = make_combinator(&right_data.indents, right_data.indents.len());
                let (parser, parse_results) = combinator.parser(right_data);

                (IndentCombinatorParser::DentParser(parser), parse_results)
            }
            IndentCombinator::Indent if right_data.dedents == 0 => {
                right_data.indents.push(vec![]);
                (IndentCombinatorParser::IndentParser(Some(right_data)), ParseResults {
                    right_data_vec: vec![],
                    up_data_vec: vec![UpData { u8set: U8Set::from_chars(" ") }],
                    cut: false,
                })
            }
            IndentCombinator::Dedent if right_data.dedents > 0 => {
                right_data.dedents -= 1;
                // println!("Decremented dedents to {}", right_data.dedents);
                (IndentCombinatorParser::Done, ParseResults {
                    right_data_vec: vec![right_data],
                    up_data_vec: vec![],
                    cut: false,
                })
            }
            IndentCombinator::AssertNoDedents if right_data.dedents == 0 => {
                (IndentCombinatorParser::Done, ParseResults {
                    right_data_vec: vec![right_data],
                    up_data_vec: vec![],
                    cut: false,
                })
            }
            _ => (IndentCombinatorParser::Done, ParseResults::no_match()),
        }
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
                    ParseResults {
                        right_data_vec: vec![right_data.clone()],
                        up_data_vec: vec![UpData { u8set: U8Set::from_chars(" ") }],
                        cut: false,
                    }
                } else {
                    // Fail. Purge the right data to poison the parser.
                    maybe_right_data.take();
                    ParseResults {
                        right_data_vec: vec![],
                        up_data_vec: vec![],
                        cut: false,
                    }
                }
            }
            IndentCombinatorParser::Done => ParseResults::no_match(),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
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

pub fn with_indent<A>(a: A) -> Seq2<IndentCombinator, Seq2<A::Output, IndentCombinator>>
where
    A: IntoCombinator,
{
    seq!(indent(), a.into_combinator(), dedent())
}
