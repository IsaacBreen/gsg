use std::cell::RefCell;
use std::rc::Rc;
use crate::{CombinatorTrait, DownData, ParserTrait};
use crate::parse_state::{RightData, UpData};

#[derive(Clone)]
pub struct LeftRecursionGuard<A> where A: CombinatorTrait {
    a: Rc<A>,
}

impl<A> CombinatorTrait for LeftRecursionGuard<A> where A: CombinatorTrait {
    type Parser = A::Parser;

    fn parser(&self, right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        // if
        // let (a, right_data_a, up_data_a) = self.a.parser(right_data.clone());
        // (a, right_data_a, up_data_a)
        todo!()
    }
}

pub fn left_recursion_guard<A>(a: A) -> LeftRecursionGuard<A> where A: CombinatorTrait {
    LeftRecursionGuard { a: Rc::new(a) }
}
