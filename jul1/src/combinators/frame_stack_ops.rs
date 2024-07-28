use crate::{Combinator, CombinatorTrait, FrameStack, Parser, ParseResults, ParserTrait, RightData, Stats};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FrameStackOpType {
    PushToFrame,
    PopFromFrame,
    FrameStackContains,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithNewFrame {
    pub a: Box<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WithNewFrameParser {
    pub a: Option<Box<Parser>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameStackOp {
    pub op_type: FrameStackOpType,
    pub a: Box<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameStackOpParser {
    pub op_type: FrameStackOpType,
    pub frame_stack: FrameStack,
    pub values: Vec<u8>,
    pub a: Box<Parser>,
}

impl CombinatorTrait for WithNewFrame {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let (a, ParseResults { right_data_vec: mut right_data_vec, up_data_vec: up_data_vec, done}) = self.a.parser(right_data);
        (Parser::WithNewFrame(WithNewFrameParser { a: Some(Box::new(a)) }), ParseResults {
            right_data_vec: right_data_vec,
            up_data_vec: up_data_vec,
            done,
        })
    }
}

impl ParserTrait for WithNewFrameParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let ParseResults { right_data_vec: mut right_data_vec, up_data_vec: up_data_vec, done } = self.a.as_mut().unwrap().step(c);
        for right_data in right_data_vec.iter_mut() {
            right_data.frame_stack = Some(FrameStack::default());
        }
        ParseResults {
            right_data_vec: right_data_vec,
            up_data_vec: up_data_vec,
            done,
        }
    }

    fn collect_stats(&self, stats: &mut Stats) {
        todo!()
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
        todo!()
    }
}

impl CombinatorTrait for FrameStackOp {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        let frame_stack = right_data.frame_stack.take().unwrap();
        let (a, ParseResults { right_data_vec: mut right_data_vec, up_data_vec: mut up_data_vec, done}) = self.a.parser(right_data);
        let parser = FrameStackOpParser {
            op_type: self.op_type,
            frame_stack,
            values: Vec::new(),
            a: Box::new(a),
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
        (Parser::FrameStackOp(parser), ParseResults {
            right_data_vec: right_data_vec,
            up_data_vec: up_data_vec,
            done,
        })
    }
}

impl ParserTrait for FrameStackOpParser {
    fn step(&mut self, c: u8) -> ParseResults {
        self.values.push(c);
        match self.op_type {
            FrameStackOpType::PushToFrame => {
                let ParseResults { right_data_vec: mut right_data_vec, up_data_vec: up_data_vec, done } = self.a.step(c);
                for right_data in right_data_vec.iter_mut() {
                    let mut frame_stack = self.frame_stack.clone();
                    frame_stack.push_name(&self.values);
                    right_data.frame_stack = Some(frame_stack);
                }
                ParseResults {
                    right_data_vec: right_data_vec,
                    up_data_vec: up_data_vec,
                    done,
                }
            }
            FrameStackOpType::PopFromFrame => {
                let ParseResults { right_data_vec: mut right_data_vec, up_data_vec: up_data_vec, done } = self.a.step(c);
                for right_data in right_data_vec.iter_mut() {
                    let mut frame_stack = self.frame_stack.clone();
                    frame_stack.pop_name(&self.values);
                    right_data.frame_stack = Some(frame_stack);
                }
                ParseResults {
                    right_data_vec: right_data_vec,
                    up_data_vec: up_data_vec,
                    done,
                }
            }
            FrameStackOpType::FrameStackContains => {
                let (u8set, is_complete) = self.frame_stack.next_u8_given_contains_u8slice(&self.values);
                let ParseResults { right_data_vec: mut right_data_vec, up_data_vec: mut up_data_vec, done } = self.a.step(c);
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
                    done,
                }
            }
        }
    }

    fn collect_stats(&self, stats: &mut Stats) {
        todo!()
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a Parser> + 'a> {
        todo!()
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut Parser> + 'a> {
        todo!()
    }
}

pub fn with_new_frame(a: Combinator) -> Combinator {
    Combinator::WithNewFrame(WithNewFrame { a: Box::new(a) })
}

pub fn push_to_frame(a: Combinator) -> Combinator {
    Combinator::FrameStackOp(FrameStackOp {
        op_type: FrameStackOpType::PushToFrame,
        a: Box::new(a),
    })
}

pub fn pop_from_frame(a: Combinator) -> Combinator {
    Combinator::FrameStackOp(FrameStackOp {
        op_type: FrameStackOpType::PopFromFrame,
        a: Box::new(a),
    })
}

pub fn frame_stack_contains(a: Combinator) -> Combinator {
    Combinator::FrameStackOp(FrameStackOp {
        op_type: FrameStackOpType::FrameStackContains,
        a: Box::new(a),
    })
}
