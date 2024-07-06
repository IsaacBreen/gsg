use crate::{Combinator, ParseData, Parser, ParseResult};

pub struct WithNewFrame<A> {
    a: A,
}

pub struct FrameStackContains<A> {
    a: A,
}

pub struct PushToFrame<A> {
    a: A,
}

pub struct PopFromFrame<A> {
    a: A,
}

pub struct WithNewFrameParser<ParserA> {
    parser: ParserA,
}

pub struct FrameStackContainsParser<ParserA> {
    parser: ParserA,
    values: Vec<u8>,
}

pub struct PushToFrameParser<ParserA> {
    parser: ParserA,
    values: Vec<u8>,
}

pub struct PopFromFrameParser<ParserA> {
    parser: ParserA,
    values: Vec<u8>,
}


impl<A, ParserA> Combinator for WithNewFrame<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = WithNewFrameParser<ParserA>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let (parser, result) = self.a.parser(parse_data);
        (WithNewFrameParser { parser }, result)
    }
}

impl<A, ParserA> Combinator for FrameStackContains<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = FrameStackContainsParser<ParserA>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let (parser, result) = self.a.parser(parse_data.clone());
        (FrameStackContainsParser { parser, values: Vec::new() }, result)
    }
}

impl<A, ParserA> Combinator for PushToFrame<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = PushToFrameParser<ParserA>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        todo!()
    }
}

impl<A, ParserA> Combinator for PopFromFrame<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = PopFromFrameParser<ParserA>;

    fn parser(&self, parse_data: ParseData) -> (Self::Parser, ParseResult) {
        todo!()
    }
}

impl<ParserA> Parser for WithNewFrameParser<ParserA>
where
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        todo!()
    }
}

impl<ParserA> Parser for FrameStackContainsParser<ParserA>
where
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        let result = self.parser.step(c);
        result
    }
}

impl<ParserA> Parser for PushToFrameParser<ParserA>
where
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        todo!()
    }
}

impl<ParserA> Parser for PopFromFrameParser<ParserA>
where
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        todo!()
    }
}

pub fn frame_stack_contains<A>(a: A) -> FrameStackContains<A> {
    FrameStackContains { a }
}

pub fn with_new_frame<A>(a: A) -> WithNewFrame<A> {
    WithNewFrame { a }
}

pub fn push_to_frame<A>(a: A) -> PushToFrame<A> {
    PushToFrame { a }
}

pub fn pop_from_frame<A>(a: A) -> PopFromFrame<A> {
    PopFromFrame { a }
}