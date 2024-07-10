use crate::{CombinatorTrait, Eps, FrameStack, RightData, ParserTrait, UpData};

pub struct WithNewFrame<A> where A: CombinatorTrait
{
    pub a: A,
}

pub struct WithNewFrameParser<P> where P: ParserTrait
{
    pub a: P,
}

impl<A> CombinatorTrait for WithNewFrame<A> where A: CombinatorTrait
{
    type Parser = WithNewFrameParser<A::Parser>;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        right_data.frame_stack.as_mut().unwrap().push_empty_frame();
        let (a, right_data_vec, up_data_vec) = self.a.parser(right_data);
        (WithNewFrameParser { a }, right_data_vec, up_data_vec)
    }
}

impl<P> ParserTrait for WithNewFrameParser<P> where P: ParserTrait
{
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        let (mut right_data_vec, up_data_vec) = self.a.step(c);
        for right_data in right_data_vec.iter_mut() {
            right_data.frame_stack.as_mut().unwrap().pop();
        }
        (right_data_vec, up_data_vec)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FrameStackOpType {
    PushToFrame,
    PopFromFrame,
    FrameStackContains,
}

pub struct FrameStackOp<A> where A: CombinatorTrait
{
    pub op_type: FrameStackOpType,
    pub a: A,
}

pub struct FrameStackOpParser<P> where P: ParserTrait
{
    pub op_type: FrameStackOpType,
    pub frame_stack: FrameStack,
    pub values: Vec<u8>,
    pub a: P,
}

impl<A> CombinatorTrait for FrameStackOp<A> where A: CombinatorTrait
{
    type Parser = FrameStackOpParser<A::Parser>;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let frame_stack = right_data.frame_stack.take().unwrap();
        let (a, mut right_data_vec, mut up_data_vec) = self.a.parser(right_data);
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
        (parser, right_data_vec, up_data_vec)
    }
}

impl<P> ParserTrait for FrameStackOpParser<P> where P: ParserTrait
{
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        self.values.push(c);
        match self.op_type {
            FrameStackOpType::PushToFrame => {
                let (mut right_data_vec, up_data_vec) = self.a.step(c);
                for right_data in right_data_vec.iter_mut() {
                    let mut frame_stack = self.frame_stack.clone();
                    frame_stack.push_name(&self.values);
                    right_data.frame_stack = Some(frame_stack);
                }
                (right_data_vec, up_data_vec)
            }
            FrameStackOpType::PopFromFrame => {
                let (mut right_data_vec, up_data_vec) = self.a.step(c);
                for right_data in right_data_vec.iter_mut() {
                    let mut frame_stack = self.frame_stack.clone();
                    frame_stack.pop_name(&self.values);
                    right_data.frame_stack = Some(frame_stack);
                }
                (right_data_vec, up_data_vec)
            }
            FrameStackOpType::FrameStackContains => {
                let (u8set, is_complete) = self.frame_stack.next_u8_given_contains_u8slice(&self.values);
                let (mut right_data_vec, mut up_data_vec) = self.a.step(c);
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
                (right_data_vec, up_data_vec)
            }
        }
    }
}

pub fn with_new_frame<T>(parser: T) -> WithNewFrame<T> where T: CombinatorTrait {
    WithNewFrame { a: parser }
}

pub fn push_to_frame<T>(parser: T) -> FrameStackOp<T> where T: CombinatorTrait {
    FrameStackOp {
        op_type: FrameStackOpType::PushToFrame,
        a: parser,
    }
}

pub fn pop_from_frame<T>(parser: T) -> FrameStackOp<T> where T: CombinatorTrait {
    FrameStackOp {
        op_type: FrameStackOpType::PopFromFrame,
        a: parser,
    }
}

pub fn frame_stack_contains<T>(parser: T) -> FrameStackOp<T> where T: CombinatorTrait {
    FrameStackOp {
        op_type: FrameStackOpType::FrameStackContains,
        a: parser,
    }
}