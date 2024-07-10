use crate::{brute_force, BruteForceFn, BruteForceParser, CombinatorTrait, DataEnum, HorizontalData, ParserTrait, seq, Seq2, U8Set, VerticalData};

const NEWLINE_FN: BruteForceFn = |values: &Vec<u8>, horizontal_data: &HorizontalData| {
    // Skip to the position just **after** the last newline
    let mut i = values.len();
    while i > 0 && values[i - 1] != b'\n' {
        i -= 1;
    }
    if i == 0 {
        // We have no newlines yet
        return DataEnum::Vertical(VerticalData { u8set: U8Set::from_chars(" \n") });
    }
    debug_assert!(values[i - 1] == b'\n' && !values[i..].iter().any(|&c| c == b'\n'));
    for indent_chunk in &horizontal_data.indents {
        let values_chunk = &values[i..i + indent_chunk.len()];
        if values_chunk != indent_chunk {
            if indent_chunk.starts_with(values_chunk) {
                // This could be a valid indentation, but we need more
                let next_u8 = values_chunk.get(indent_chunk.len()).cloned().unwrap();
                let mut u8set = U8Set::from_u8(next_u8);
                u8set.insert(b'\n');
                return DataEnum::Vertical(VerticalData { u8set } );
            } else {
                // We have invalid indentation, but it's ok, we can still match a blank line as long as we haven't encountered any non-whitespace characters.
                if values.iter().all(|&c| c.is_ascii_whitespace()) {
                    return DataEnum::Vertical(VerticalData { u8set: U8Set::from_chars(" \n") });
                } else {
                    // We have invalid indentation
                    return DataEnum::None;
                }
            }
        }
        i += indent_chunk.len();
    }
    DataEnum::Horizontal(horizontal_data.clone())
};

#[derive(Debug, Clone, PartialEq)]
pub enum IndentCombinator {
    Newline,
    Indent,
    Dedent,
}

pub enum IndentCombinatorParser {
    NewlineParser(BruteForceParser),
    IndentParser,
    DedentParser,
}

impl CombinatorTrait for IndentCombinator {
    type Parser = IndentCombinatorParser;

    fn parser(&self, mut horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        // match self {
        //     IndentCombinator::Newline => (self.clone(), vec![], vec![VerticalData { u8set: U8Set::from_chars(" \n") }]),
        //     IndentCombinator::Indent => {
        //         if horizontal_data.dedents == 0 {
        //             // Fail
        //             (self.clone(), vec![], vec![])
        //         } else {
        //             (self.clone(), vec![], vec![VerticalData { u8set: U8Set::from_chars(" ") }])
        //         }
        //     },
        //     IndentCombinator::Dedent => {
        //         if horizontal_data.dedents == 0 {
        //             (self.clone(), vec![], vec![])
        //         } else {
        //             horizontal_data.dedents -= 1;
        //             (self.clone(), vec![horizontal_data], vec![])
        //         }
        //     }
        // }
        match self {
            IndentCombinator::Newline => {
                let (parser, horizontal_data_vec, vertical_data_vec) = brute_force(NEWLINE_FN).parser(horizontal_data);
                (IndentCombinatorParser::NewlineParser(parser), horizontal_data_vec, vertical_data_vec)
            }
            IndentCombinator::Indent => (IndentCombinatorParser::IndentParser, vec![], vec![VerticalData { u8set: U8Set::from_chars(" ") }]),
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
            IndentCombinatorParser::NewlineParser(parser) => parser.step(c),
            IndentCombinatorParser::IndentParser => (vec![], vec![VerticalData { u8set: U8Set::from_chars(" ") }]),
            IndentCombinatorParser::DedentParser => (vec![], vec![]),
        }
    }
}

pub fn newline() -> IndentCombinator {
    IndentCombinator::Newline
}

pub fn with_indent<A>(a: A) -> Seq2<IndentCombinator, Seq2<A, IndentCombinator>> where A: CombinatorTrait
{
    seq!(indent(), a, dedent())
}

pub fn indent() -> IndentCombinator {
    IndentCombinator::Indent
}

pub fn dedent() -> IndentCombinator {
    IndentCombinator::Dedent
}
