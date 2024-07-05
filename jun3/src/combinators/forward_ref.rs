use crate::{Combinator, ParseData, Parser, ParseResult, U8Set};

pub struct ForwardRef<A> {
    a: Option<A>,
}

pub struct ForwardRefParser<ParserA> {
    a: ParserA,
}

impl<A, ParserA> Combinator for ForwardRef<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = ForwardRefParser<ParserA>;

    fn parser(&self, parse_data: ParseData) -> Self::Parser {
        ForwardRefParser {
            a: self.a.as_ref().expect("ForwardRef::parser called before parser").parser(parse_data),
        }
    }
}

impl<ParserA> Parser for ForwardRefParser<ParserA>
where
    ParserA: Parser,
{
    fn result(&self) -> ParseResult {
        self.a.result()
    }

    fn step(&mut self, c: u8) {
        self.a.step(c);
    }
}

pub fn forward_ref<A>() -> ForwardRef<A> {
    ForwardRef { a: None }
}