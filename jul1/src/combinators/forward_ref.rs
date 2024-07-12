use std::cell::RefCell;
use std::rc::Rc;

use crate::{CombinatorTrait, IntoCombinator, LeftRecursionGuard, ParserTrait};
use crate::parse_state::{RightData, UpData};

pub struct ForwardRef {
    a: Rc<RefCell<Option<Rc<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>>>>>,
}

impl CombinatorTrait for ForwardRef {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        self.a.parser(right_data)
    }
}

impl CombinatorTrait for RefCell<Option<Rc<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>>>> {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        self.borrow().as_ref().unwrap().parser(right_data)
    }
}

impl IntoCombinator for &ForwardRef {
    type Output = Rc<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>>;
    fn into_combinator(self) -> Self::Output {
        if let Some(a) = self.a.borrow().as_ref() {
            a.clone()
        } else {
            LeftRecursionGuard { a: self.a.clone() }.into_boxed().into()
        }
    }
}

pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: Rc::new(RefCell::new(None)) }
}

impl ForwardRef {
    // pub fn set<A: CombinatorTrait<Parser=P> + 'static, P: ParserTrait + 'static>(&mut self, a: A) -> Rc<A> {
    //     let a = Rc::new(a);
    //     *self.a.borrow_mut() = Some(a.clone().into_boxed().into());
    //     a
    // }
    pub fn set<A: IntoCombinator<Output = B>, B: CombinatorTrait<Parser=P> + 'static, P: ParserTrait + 'static>(&mut self, a: A) -> Rc<B> {
        let a = Rc::new(a.into_combinator());
        *self.a.borrow_mut() = Some(a.clone().into_boxed().into());
        a
    }
}