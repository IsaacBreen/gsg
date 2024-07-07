use crate::{Combinator, ParseData, Parser, ParseResult};

enum IndentOpType {
    Newline,
    Indent,
    Dedent,
}

struct IndentOp {
    indent_op_type: IndentOpType,
}

struct IndentOpParser {
    indent_op_type: IndentOpType,
}

impl Combinator for IndentOp {
    type Parser = IndentOpParser;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        todo!()
    }
}

impl Parser for IndentOpParser {
    fn step(&mut self, c: u8) -> ParseResult {
        todo!()
    }
}