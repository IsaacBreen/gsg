use crate::{brute_force, BruteForceFn, BruteForceParser, Choice2, CombinatorTrait, eat_char_choice, EatU8, Eps, HorizontalData, ParserTrait, repeat, Repeat1, repeat1, seq, Seq2, U8Set, VerticalData};

const DENT_FN: BruteForceFn = |values: &Vec<u8>, horizontal_data: &HorizontalData| {
    let mut i = 0;
    for (indent_num, indent_chunk) in horizontal_data.indents.iter().enumerate() {
        if i == values.len() {
            // We've matched every indent chunk so far in its entirety.
            // If there are remaining chunks, we could continue to match them, or we could match a
            // non-whitespace character and emit dedents - one for each remaining chunk.
            if indent_num < horizontal_data.indents.len() {
                let next_u8 = indent_chunk.get(values.len()).cloned().unwrap();
                let u8set = U8Set::from_u8(next_u8);
                let mut horizontal_data = horizontal_data.clone();
                horizontal_data.dedents = horizontal_data.indents.len() - indent_num;
                horizontal_data.indents.truncate(indent_num);
                return (vec![horizontal_data], vec![VerticalData { u8set } ]);
            }
        }
        let values_chunk = &values[i..(i + indent_chunk.len()).min(values.len())];
        if values_chunk != indent_chunk {
            if indent_chunk.starts_with(values_chunk) {
                // This could be a valid indentation, but we need more
                let next_u8 = indent_chunk.get(values_chunk.len()).cloned().unwrap();
                let u8set = U8Set::from_u8(next_u8);
                return (vec![], vec![VerticalData { u8set } ]);
            } else {
                // We have invalid indentation
                return (vec![], vec![]);
            }
        }
        i += indent_chunk.len();
    }
    if i == values.len() {
        (vec![horizontal_data.clone()], vec![])
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
    IndentParser(Option<HorizontalData>),
    DedentParser,
}

impl CombinatorTrait for IndentCombinator {
    type Parser = IndentCombinatorParser;

    fn parser(&self, mut horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        match self {
            IndentCombinator::Dent => {
                let (parser, horizontal_data_vec, vertical_data_vec) = brute_force(DENT_FN).parser(horizontal_data);
                (IndentCombinatorParser::DentParser(parser), horizontal_data_vec, vertical_data_vec)
            }
            IndentCombinator::Indent => {
                horizontal_data.indents.push(vec![]);
                (IndentCombinatorParser::IndentParser(Some(horizontal_data)), vec![], vec![VerticalData { u8set: U8Set::from_chars(" ") }])
            },
            IndentCombinator::Dedent => {
                let horizontal_data_to_return = if horizontal_data.dedents == 0 {
                    vec![]
                } else {
                    horizontal_data.dedents -= 1;
                    vec![horizontal_data]
                };
                (IndentCombinatorParser::DedentParser, horizontal_data_to_return, vec![])
            }
        }
    }
}

impl ParserTrait for IndentCombinatorParser {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>) {
        match self {
            IndentCombinatorParser::DentParser(parser) => parser.step(c),
            IndentCombinatorParser::IndentParser(maybe_horizontal_data) => {
                if c == b' ' {
                    let mut horizontal_data = maybe_horizontal_data.as_mut().unwrap().clone();
                    horizontal_data.indents.last_mut().unwrap().push(c);
                    (vec![horizontal_data.clone()], vec![VerticalData { u8set: U8Set::from_chars(" ") }])
                } else {
                    // Fail. Purge the horizontal data to poison the parser.
                    maybe_horizontal_data.take();
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