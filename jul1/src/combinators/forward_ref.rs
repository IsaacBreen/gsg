use std::cell::RefCell;
use std::rc::Rc;
use crate::{CombinatorTrait, ParserTrait};
use crate::parse_state::{HorizontalData, VerticalData};

#[derive(Clone)]
pub struct ForwardRef where Self: CombinatorTrait {
    a: Rc<RefCell<Option<Rc<dyn CombinatorTrait<Parser = Box<dyn ParserTrait>>>>>>,
}

impl CombinatorTrait for ForwardRef {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, horizontal_data: HorizontalData) -> (Self::Parser, Vec<HorizontalData>, Vec<VerticalData>) {
        self.a.borrow().as_ref().unwrap().parser(horizontal_data)
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