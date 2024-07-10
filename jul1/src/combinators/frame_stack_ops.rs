use crate::{CombinatorTrait, Eps, FrameStack, HorizontalData, ParserTrait, VerticalData};

pub enum FrameStackOpType {
    WithNewFrame,
    PushToFrame,
    PopFromFrame,
    FrameStackContains,
}

struct FrameStackOp<A> where A: CombinatorTrait
{
    pub op_type: FrameStackOpType,
    pub a: A,
}

pub struct FrameStackOpParser<P> where P: ParserTrait
{
    pub op_type: FrameStackOpType,
    pub frame_stack: Option<FrameStack>,
    pub values: Vec<u8>,
    pub a: P,
}

impl<A> CombinatorTrait for FrameStackOp<A> where A: CombinatorTrait
{
    type Parser = FrameStackOpParser<A::Parser>;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        let (a, horizontal_data_vec, mut vertical_data_vec) = self.a.parser(horizontal_data);
        // vertical_data_vec.retain(|v| horizontal_data.frame_stack.cont
        todo!();
    }
}

impl<P> ParserTrait for FrameStackOpParser<P> where P: ParserTrait
{
    fn step(&mut self, c: u8) -> (Vec<HorizontalData>, Vec<VerticalData>) {
        todo!()
    }
}

pub fn with_new_frame<T>(parser: T) -> Eps {
    todo!()
}

pub fn push_to_frame<T>(parser: T) -> Eps {
    todo!()
}

pub fn pop_from_frame<T>(parser: T) -> Eps {
    todo!()
}

pub fn frame_stack_contains<T>(parser: T) -> Eps {
    todo!()
}