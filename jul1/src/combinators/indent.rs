use std::any::Any;
use crate::{brute_force, BruteForceFn, BruteForceParser, Choice2, CombinatorTrait, eat_char_choice, EatU8, Eps, IntoCombinator, ParseResults, ParserTrait, repeat0, Repeat1, repeat1, RightData, seq, Seq2, Stats, U8Set, UpData};

const DENT_FN: BruteForceFn = |values: &Vec<u8>, right_data: &RightData| {
    let mut i = 0;
    for (indent_num, indent_chunk) in right_data.indents.iter().enumerate() {
        if i == values.len() {
            // We've matched every indent chunk so far in its entirety.
            // If there are remaining chunks, we could continue to match them, or we could match a
            // non-whitespace character and emit dedents - one for each remaining chunk.
            if indent_num < right_data.indents.len() {
                let next_u8 = indent_chunk[values.len() - i];
                let u8set = U8Set::from_u8(next_u8);
                let mut right_data = right_data.clone();
                right_data.dedents = right_data.indents.len() - indent_num;
                right_data.indents.truncate(indent_num);
                println!("We've matched every indent chunk so far in its entirety. If there are remaining chunks, we could continue to match them, or we could match a non-whitespace character and emit dedents - one for each remaining chunk.");
                return ParseResults {
                    right_data_vec: vec![right_data],
                    up_data_vec: vec![UpData { u8set }],
                    cut: false,
                };
            }
        }
        let values_chunk = &values[i..(i + indent_chunk.len()).min(values.len())];
        if values_chunk != indent_chunk {
            if indent_chunk.starts_with(values_chunk) {
                // This could be a valid indentation, but we need more
                let next_u8 = indent_chunk.get(values_chunk.len()).cloned().unwrap();
                let u8set = U8Set::from_u8(next_u8);
                println!("We have invalid indentation");
                return ParseResults {
                    right_data_vec: vec![],
                    up_data_vec: vec![UpData { u8set }],
                    cut: false,
                };
            } else {
                // We have invalid indentation
                println!("We have invalid indentation");
                return ParseResults {
                    right_data_vec: vec![],
                    up_data_vec: vec![],
                    cut: false,
                };
            }
        }
        i += values_chunk.len();
    }
    // join the indent vecs
    let mut full_indent = Vec::new();
    for indent in right_data.indents.iter() {
        full_indent.extend_from_slice(indent);
    }
    if i == values.len() {
        assert_eq!(&full_indent, values);
        println!("done");
        ParseResults {
            right_data_vec: vec![right_data.clone()],
            up_data_vec: vec![],
            cut: false,
        }
    } else {
        println!("fail");
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        }
    }
};

#[derive(Debug, Clone, PartialEq)]
pub enum IndentCombinator {
    Dent,
    Indent,
    Dedent,
    AssertNoDedents,
}

pub enum IndentCombinatorParser {
    DentParser(BruteForceParser),
    IndentParser(Option<RightData>),
    Done,
}

impl CombinatorTrait for IndentCombinator {
    type Parser = IndentCombinatorParser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        match self {
            IndentCombinator::Dent if right_data.dedents == 0  => {
                let (parser, parse_results) = brute_force(DENT_FN).parser(right_data);
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
                    let mut right_data = maybe_right_data.as_mut().unwrap().clone();
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
