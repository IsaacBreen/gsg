use crate::{Combinator, FrameStack, ParseData, Parser, ParseResult, U8Set};

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
    frame_stack: FrameStack,
    values: Vec<u8>,
}

pub struct PushToFrameParser<ParserA> {
    parser: ParserA,
    frame_stack: FrameStack,
    values: Vec<u8>,
}

pub struct PopFromFrameParser<ParserA> {
    parser: ParserA,
    frame_stack: FrameStack,
    values: Vec<u8>,
}

fn try_pop_frame(result: &mut ParseResult) {
    if let Some(parse_data) = &mut result.parse_data {
        if let Some(frame_stack) = &mut parse_data.frame_stack {
            frame_stack.pop();
        }
    }
}

impl<A, ParserA> Combinator for WithNewFrame<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = WithNewFrameParser<ParserA>;

    fn parser(&self, mut parse_data: ParseData) -> (Self::Parser, ParseResult) {
        if let Some(frame_stack) = &mut parse_data.frame_stack {
            frame_stack.push_empty_frame();
        }
        let (parser, mut result) = self.a.parser(parse_data);
        try_pop_frame(&mut result);
        (WithNewFrameParser { parser }, result)
    }
}

impl<ParserA> Parser for WithNewFrameParser<ParserA>
where
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        let mut result = self.parser.step(c);
        try_pop_frame(&mut result);
        result
    }
}

impl<ParserA> FrameStackContainsParser<ParserA> {
    fn helper(&mut self, mut result: ParseResult) -> ParseResult {
        let (u8set, is_complete) = self.frame_stack.next_u8_given_contains_u8slice(self.values.clone().as_slice());
        result.u8set = result.u8set & u8set;
        if result.parse_data.is_some() && !is_complete {
            result.parse_data = None;
        }
        if let Some(parse_data) = &mut result.parse_data {
            parse_data.frame_stack = Some(self.frame_stack.clone());
        }
        result
    }
}

impl<A, ParserA> Combinator for FrameStackContains<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = FrameStackContainsParser<ParserA>;

    fn parser(&self, mut parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let frame_stack = parse_data.frame_stack.take().unwrap();
        let (a_parser, mut result) = self.a.parser(parse_data.clone());
        let mut parser = FrameStackContainsParser { parser: a_parser, frame_stack, values: Vec::new() };

        result = parser.helper(result);
        (parser, result)
    }
}

impl<ParserA> Parser for FrameStackContainsParser<ParserA>
where
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        self.values.push(c);
        let mut result = self.parser.step(c);
        if !self.frame_stack.contains_prefix_u8vec(self.values.clone()) {
            result = ParseResult::default();
        }

        result = self.helper(result);
        result
    }
}

impl<ParserA> PushToFrameParser<ParserA> {
    fn helper(&mut self, mut result: ParseResult) -> ParseResult {
        if let Some(parse_data) = &mut result.parse_data {
            let mut frame_stack = self.frame_stack.clone();
            frame_stack.push_name(&self.values);
            parse_data.frame_stack = Some(frame_stack);
        }
        result
    }
}

impl<A, ParserA> Combinator for PushToFrame<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = PushToFrameParser<ParserA>;

    fn parser(&self, mut parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let frame_stack = parse_data.frame_stack.take().unwrap();
        let (a_parser, mut result) = self.a.parser(parse_data.clone());
        let mut parser = PushToFrameParser { parser: a_parser, frame_stack, values: Vec::new() };

        result = parser.helper(result);
        (parser, result)
    }
}

impl<ParserA> Parser for PushToFrameParser<ParserA>
where
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        self.values.push(c);
        let mut result = self.parser.step(c);

        result = self.helper(result);
        result
    }
}

impl<ParserA> PopFromFrameParser<ParserA> {
    fn helper(&mut self, mut result: ParseResult) -> ParseResult {
        if !self.frame_stack.contains_prefix_u8vec(self.values.clone()) {
            result = ParseResult::default();
        }
        if let Some(parse_data) = &mut result.parse_data {
            let mut frame_stack = self.frame_stack.clone();
            frame_stack.pop_name(&self.values);
            parse_data.frame_stack = Some(frame_stack);
        }
        result
    }
}

impl<A, ParserA> Combinator for PopFromFrame<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = PopFromFrameParser<ParserA>;

    fn parser(&self, mut parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let frame_stack = parse_data.frame_stack.take().unwrap();
        let (a_parser, mut result) = self.a.parser(parse_data.clone());
        let mut parser = PopFromFrameParser { parser: a_parser, frame_stack, values: Vec::new() };

        result = parser.helper(result);
        (parser, result)
    }
}

impl<ParserA> Parser for PopFromFrameParser<ParserA>
where
    ParserA: Parser,
{
    fn step(&mut self, c: u8) -> ParseResult {
        self.values.push(c);
        let mut result = self.parser.step(c);

        result = self.helper(result);
        result
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