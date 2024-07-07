use crate::{Combinator, FrameStack, ParseData, Parser, ParseResult, U8Set};

pub struct WithNewFrame<A> {
    a: A,
}

pub struct WithNewFrameParser<ParserA> {
    parser: ParserA,
}


#[derive(Clone, Copy)]
pub enum FrameOperationType {
    Contains,
    Push,
    Pop,
}

pub struct FrameOperation<A> {
    a: A,
    operation: FrameOperationType,
}

pub struct FrameOperationParser<ParserA> {
    parser: ParserA,
    frame_stack: FrameStack,
    values: Vec<u8>,
    operation: FrameOperationType,
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

impl<A, ParserA> Combinator for FrameOperation<A>
where
    A: Combinator<Parser = ParserA>,
    ParserA: Parser,
{
    type Parser = FrameOperationParser<ParserA>;

    fn parser(&self, mut parse_data: ParseData) -> (Self::Parser, ParseResult) {
        let frame_stack = parse_data.frame_stack.take().unwrap();
        let (parser, mut result) = self.a.parser(parse_data);
        let mut parser = FrameOperationParser {
            parser,
            frame_stack,
            values: Vec::new(),
            operation: self.operation,
        };
        result = parser.helper(result);
        (parser, result)
    }
}

impl<ParserA> FrameOperationParser<ParserA> {
    fn helper(&mut self, mut result: ParseResult) -> ParseResult {
        if matches!(self.operation, FrameOperationType::Contains | FrameOperationType::Pop) && !self.frame_stack.contains_prefix_u8vec(self.values.clone()) {
            result = ParseResult::default();
        }
        if matches!(self.operation, FrameOperationType::Contains) {
            let (u8set, is_complete) = self.frame_stack.next_u8_given_contains_u8slice(self.values.clone().as_slice());
            result.u8set = result.u8set & u8set;
            if result.parse_data.is_some() && !is_complete {
                result.parse_data = None;
            }
        }
        if let Some(parse_data) = &mut result.parse_data {
            let mut frame_stack = self.frame_stack.clone();
            match self.operation {
                FrameOperationType::Contains => {}
                FrameOperationType::Push => { frame_stack.push_name(&self.values); }
                FrameOperationType::Pop => { frame_stack.pop_name(&self.values); }
            }
            parse_data.frame_stack = Some(frame_stack);
        }
        result
    }
}

impl<ParserA> Parser for FrameOperationParser<ParserA>
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

pub fn with_new_frame<A>(a: A) -> WithNewFrame<A> {
    WithNewFrame { a }
}

pub fn frame_stack_contains<A>(a: A) -> FrameOperation<A> {
    FrameOperation { a, operation: FrameOperationType::Contains }
}

pub fn push_to_frame<A>(a: A) -> FrameOperation<A> {
    FrameOperation { a, operation: FrameOperationType::Push }
}

pub fn pop_from_frame<A>(a: A) -> FrameOperation<A> {
    FrameOperation { a, operation: FrameOperationType::Pop }
}