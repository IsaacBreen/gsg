use std::any::Any;
use crate::{CombinatorTrait, FrameStack, IntoCombinator, ParseResults, ParserTrait, RightData, Stats, UpData};

#[derive(PartialEq)]
pub struct WithNewFrame<A>
where
    A: CombinatorTrait,
{
    pub a: A,
}

#[derive(PartialEq)]
pub struct WithNewFrameParser<P>
where
    P: ParserTrait,
{
    pub a: P,
}

impl<A> CombinatorTrait for WithNewFrame<A>
where
    A: CombinatorTrait,
{
    type Parser = WithNewFrameParser<A::Parser>;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        right_data.frame_stack.as_mut().unwrap().push_empty_frame();
        let (a, parse_results) = self.a.parser(right_data);
        (WithNewFrameParser { a }, parse_results)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<P> ParserTrait for WithNewFrameParser<P>
where
    P: ParserTrait + 'static,
{
    fn step(&mut self, c: u8) -> ParseResults {
        let ParseResults { right_data_vec: mut right_data_vec, up_data_vec: up_data_vec, cut } = self.a.step(c);
        for right_data in right_data_vec.iter_mut() {
            right_data.frame_stack.as_mut().unwrap().pop();
        }
        ParseResults {
            right_data_vec: right_data_vec,
            up_data_vec: up_data_vec,
            cut,
        }
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        Box::new(std::iter::once(&self.a as &dyn ParserTrait))
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        Box::new(std::iter::once(&mut self.a as &mut dyn ParserTrait))
    }

    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.a.dyn_eq(&other.a)
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FrameStackOpType {
    PushToFrame,
    PopFromFrame,
    FrameStackContains,
}

pub struct FrameStackOp<A>
where
    A: CombinatorTrait,
{
    pub op_type: FrameStackOpType,
    pub a: A,
}

pub struct FrameStackOpParser<P>
where
    P: ParserTrait,
{
    pub op_type: FrameStackOpType,
    pub frame_stack: FrameStack,
    pub values: Vec<u8>,
    pub a: P,
}

impl<A> CombinatorTrait for FrameStackOp<A>
where
    A: CombinatorTrait,
{
    type Parser = FrameStackOpParser<A::Parser>;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        let frame_stack = right_data.frame_stack.take().unwrap();
        let (a, ParseResults { right_data_vec: mut right_data_vec, up_data_vec: mut up_data_vec, cut }) = self.a.parser(right_data);
        let parser = FrameStackOpParser {
            op_type: self.op_type,
            frame_stack,
            values: Vec::new(),
            a,
        };
        match self.op_type {
            FrameStackOpType::PushToFrame => {
                for right_data in right_data_vec.iter_mut() {
                    right_data.frame_stack.as_mut().unwrap().push_empty_frame();
                }
            }
            FrameStackOpType::PopFromFrame => {
                for right_data in right_data_vec.iter_mut() {
                    right_data.frame_stack.as_mut().unwrap().pop();
                }
            }
            FrameStackOpType::FrameStackContains => {
                let (u8set, is_complete) = parser.frame_stack.next_u8_given_contains_u8slice(&parser.values);
                for up_data in up_data_vec.iter_mut() {
                    up_data.u8set = up_data.u8set.intersection(&u8set);
                }
                if !is_complete {
                    // Empty right data
                    right_data_vec = vec![];
                }
            }
        }
        (parser, ParseResults {
            right_data_vec: right_data_vec,
            up_data_vec: up_data_vec,
            cut,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<P> ParserTrait for FrameStackOpParser<P>
where
    P: ParserTrait + 'static,
{
    fn step(&mut self, c: u8) -> ParseResults {
        self.values.push(c);
        match self.op_type {
            FrameStackOpType::PushToFrame => {
                let ParseResults { right_data_vec: mut right_data_vec, up_data_vec: up_data_vec, cut } = self.a.step(c);
                for right_data in right_data_vec.iter_mut() {
                    let mut frame_stack = self.frame_stack.clone();
                    frame_stack.push_name(&self.values);
                    right_data.frame_stack = Some(frame_stack);
                }
                ParseResults {
                    right_data_vec: right_data_vec,
                    up_data_vec: up_data_vec,
                    cut,
                }
            }
            FrameStackOpType::PopFromFrame => {
                let ParseResults { right_data_vec: mut right_data_vec, up_data_vec: up_data_vec, cut } = self.a.step(c);
                for right_data in right_data_vec.iter_mut() {
                    let mut frame_stack = self.frame_stack.clone();
                    frame_stack.pop_name(&self.values);
                    right_data.frame_stack = Some(frame_stack);
                }
                ParseResults {
                    right_data_vec: right_data_vec,
                    up_data_vec: up_data_vec,
                    cut,
                }
            }
            FrameStackOpType::FrameStackContains => {
                let (u8set, is_complete) = self.frame_stack.next_u8_given_contains_u8slice(&self.values);
                let ParseResults { right_data_vec: mut right_data_vec, up_data_vec: mut up_data_vec, cut } = self.a.step(c);
                for up_data in up_data_vec.iter_mut() {
                    up_data.u8set = up_data.u8set.intersection(&u8set);
                }
                if !is_complete {
                    // Empty right data
                    right_data_vec = vec![];
                } else {
                    for right_data in right_data_vec.iter_mut() {
                        right_data.frame_stack = Some(self.frame_stack.clone());
                    }
                }
                ParseResults {
                    right_data_vec: right_data_vec,
                    up_data_vec: up_data_vec,
                    cut,
                }
            }
        }
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        Box::new(std::iter::once(&self.a as &dyn ParserTrait))
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        Box::new(std::iter::once(&mut self.a as &mut dyn ParserTrait))
    }

    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.op_type == other.op_type && self.a.dyn_eq(&other.a)
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn with_new_frame<T>(parser: T) -> WithNewFrame<T::Output>
where
    T: IntoCombinator,
{
    WithNewFrame { a: parser.into_combinator() }
}

pub fn push_to_frame<T>(parser: T) -> FrameStackOp<T::Output>
where
    T: IntoCombinator,
{
    FrameStackOp {
        op_type: FrameStackOpType::PushToFrame,
        a: parser.into_combinator(),
    }
}

pub fn pop_from_frame<T>(parser: T) -> FrameStackOp<T::Output>
where
    T: IntoCombinator,
{
    FrameStackOp {
        op_type: FrameStackOpType::PopFromFrame,
        a: parser.into_combinator(),
    }
}

pub fn frame_stack_contains<T>(parser: T) -> FrameStackOp<T::Output>
where
    T: IntoCombinator,
{
    FrameStackOp {
        op_type: FrameStackOpType::FrameStackContains,
        a: parser.into_combinator(),
    }
}