use std::cell::RefCell;
use std::rc::Rc;
use crate::{CombinatorTrait, ParserTrait};
use crate::parse_state::{HorizontalData, VerticalData};

#[derive(Clone)]
pub struct LeftRecursionGuard<A> where A: CombinatorTrait {
    a: Rc<A>,
}

impl<A> CombinatorTrait for LeftRecursionGuard<A> where A: CombinatorTrait {
    type Parser = A::Parser;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        // if
        // let (a, horizontal_data_a, vertical_data_a) = self.a.parser(horizontal_data.clone());
        // (a, horizontal_data_a, vertical_data_a)
        todo!()
    }
}

pub fn left_recursion_guard<A>(a: A) -> LeftRecursionGuard<A> where A: CombinatorTrait {
    LeftRecursionGuard { a: Rc::new(a) }
}
