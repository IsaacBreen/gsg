use crate::{brute_force, BruteForceFn, BruteForceParser, Choice2, CombinatorTrait, eat_char_choice, EatU8, Eps, RightData, ParserTrait, repeat, Repeat1, repeat1, seq, Seq2, U8Set, UpData, DownData};

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
                return (vec![right_data], vec![UpData { u8set, left_recursion_guard_data: Default::default() } ]);
            }
        }
        let values_chunk = &values[i..(i + indent_chunk.len()).min(values.len())];
        if values_chunk != indent_chunk {
            if indent_chunk.starts_with(values_chunk) {
                // This could be a valid indentation, but we need more
                let next_u8 = indent_chunk.get(values_chunk.len()).cloned().unwrap();
                let u8set = U8Set::from_u8(next_u8);
                return (vec![], vec![UpData { u8set, left_recursion_guard_data: Default::default() } ]);
            } else {
                // We have invalid indentation
                return (vec![], vec![]);
            }
        }
        i += indent_chunk.len();
    }
    if i == values.len() {
        (vec![right_data.clone()], vec![])
    } else {
        (vec![], vec![])
    }
};

#[derive(Debug, Clone, PartialEq)]
pub enum IndentCombinator {
    Dent,
    Indent,
    Dedent,
}

pub enum IndentCombinatorParser {
    DentParser(BruteForceParser),
    IndentParser(Option<RightData>),
    DedentParser,
}

impl CombinatorTrait for IndentCombinator {
    type Parser = IndentCombinatorParser;

    fn parser(&self, mut right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        match self {
            IndentCombinator::Dent => {
                let (parser, right_data_vec, up_data_vec) = brute_force(DENT_FN).parser(right_data, down_data);
                (IndentCombinatorParser::DentParser(parser), right_data_vec, up_data_vec)
            }
            IndentCombinator::Indent => {
                right_data.indents.push(vec![]);
                (IndentCombinatorParser::IndentParser(Some(right_data)), vec![], vec![UpData { u8set: U8Set::from_chars(" "), left_recursion_guard_data: Default::default() }])
            },
            IndentCombinator::Dedent => {
                let right_data_to_return = if right_data.dedents == 0 {
                    vec![]
                } else {
                    right_data.dedents -= 1;
                    vec![right_data]
                };
                (IndentCombinatorParser::DedentParser, right_data_to_return, vec![])
            }
        }
    }
}

impl ParserTrait for IndentCombinatorParser {
    fn step(&mut self, c: u8, down_data: DownData) -> (Vec<RightData>, Vec<UpData>) {
        match self {
            IndentCombinatorParser::DentParser(parser) => parser.step(c, down_data),
            IndentCombinatorParser::IndentParser(maybe_right_data) => {
                if c == b' ' {
                    let mut right_data = maybe_right_data.as_mut().unwrap().clone();
                    right_data.indents.last_mut().unwrap().push(c);
                    (vec![right_data.clone()], vec![UpData { u8set: U8Set::from_chars(" "), left_recursion_guard_data: Default::default() }])
                } else {
                    // Fail. Purge the right data to poison the parser.
                    maybe_right_data.take();
                    (vec![], vec![])
                }
            }
            IndentCombinatorParser::DedentParser => (vec![], vec![]),
        }
    }
}

pub fn newline() -> Seq2<Choice2<Repeat1<EatU8>, Eps>, EatU8> {
    seq!(repeat(eat_char_choice(" ")), eat_char_choice("\n"))
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

pub fn with_indent<A>(a: A) -> Seq2<IndentCombinator, Seq2<A, IndentCombinator>> where A: CombinatorTrait
{
    seq!(indent(), a, dedent())
}

pub fn python_newline() -> Seq2<Repeat1<Seq2<Choice2<Repeat1<EatU8>, Eps>, EatU8>>, IndentCombinator> {
    seq!(repeat1(newline()), dent())
}