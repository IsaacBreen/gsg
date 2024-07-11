use std::cell::RefCell;
use std::rc::Rc;
use crate::{CombinatorTrait, IntoCombinator, left_recursion_guard, ParserTrait};
use crate::parse_state::{RightData, UpData};

#[derive(Clone)]
pub struct ForwardRef where Self: CombinatorTrait {
    a: Rc<RefCell<Option<Rc<dyn CombinatorTrait<Parser = Box<dyn ParserTrait>>>>>>,
}

impl CombinatorTrait for ForwardRef {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        self.a.borrow().as_ref().unwrap().parser(right_data)
    }
}

impl IntoCombinator for &ForwardRef {
    type Output = Rc<dyn CombinatorTrait<Parser = Box<dyn ParserTrait>>>;
    fn into_combinator(self) -> Self::Output {
        if let Some(a) = self.a.borrow().as_ref() {
            a.clone()
        } else {
            left_recursion_guard(self.clone().into_boxed().into()).into_boxed().into()
        }
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