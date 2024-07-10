use crate::{BruteForceFn, BruteForceParser, CombinatorTrait, DataEnum, HorizontalData, ParserTrait, seq, Seq2, U8Set, VerticalData};

const NEWLINE_FN: BruteForceFn = |values: &Vec<u8>, horizontal_data: &HorizontalData| {
    let mut i = 0;
    for indent_chunk in &horizontal_data.indents {
        let values_chunk = &values[i..i + indent_chunk.len()];
        if values_chunk != indent_chunk {
            if indent_chunk.starts_with(values_chunk) {
                // This could be a valid indentation, but we need more
                let next_u8 = values_chunk.get(indent_chunk.len()).cloned().unwrap();
                return DataEnum::Vertical(VerticalData { u8set: U8Set::from_u8(next_u8) });
            } else {
                // We have invalid indentation
                return DataEnum::None;
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
    NewlineParser,
    IndentParser,
    DedentParser,
}

impl CombinatorTrait for IndentCombinator {
    type Parser = IndentCombinatorParser

    fn parser(&self, mut horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        match self {
            IndentCombinator::Newline => (self.clone(), vec![], vec![VerticalData { u8set: U8Set::from_chars(" \n") }]),
            IndentCombinator::Indent => {
                if horizontal_data.dedents == 0 {
                    // Fail
                    (self.clone(), vec![], vec![])
                } else {
                    (self.clone(), vec![], vec![VerticalData { u8set: U8Set::from_chars(" ") }])
                }
            },
            IndentCombinator::Dedent => {
                if horizontal_data.dedents == 0 {
                    (self.clone(), vec![], vec![])
                } else {
                    horizontal_data.dedents -= 1;
                    (self.clone(), vec![horizontal_data], vec![])
                }
            }
        }
    }
}

impl ParserTrait for IndentCombinator {
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>) {

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
