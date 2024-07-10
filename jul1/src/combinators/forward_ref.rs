use std::cell::RefCell;
use std::rc::Rc;
use crate::{CombinatorTrait, DownData, ParserTrait};
use crate::parse_state::{RightData, UpData};

#[derive(Clone)]
pub struct ForwardRef where Self: CombinatorTrait {
    a: Rc<RefCell<Option<Rc<dyn CombinatorTrait<Parser = Box<dyn ParserTrait>>>>>>,
}

impl CombinatorTrait for ForwardRef {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData, down_data: DownData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        self.a.borrow().as_ref().unwrap().parser(right_data, down_data)
    }
}

pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: Rc::new(RefCell::new(None)) }
}

impl ForwardRef {
    pub fn set<A: CombinatorTrait<Parser = P> + 'static, P: ParserTrait + 'static>(&mut self, a: A) {
        *self.a.borrow_mut() = Some(a.into_boxed().into());
    }
}