use std::cell::RefCell;
use std::rc::Rc;

use crate::{CombinatorTrait, IntoCombinator, ParseResults, ParserTrait};
use crate::parse_state::{RightData, UpData};

pub struct ForwardRef {
    a: Rc<RefCell<Option<Rc<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>>>>>,
}

impl CombinatorTrait for ForwardRef {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        self.a.parser(right_data)
    }
}

impl CombinatorTrait for RefCell<Option<Rc<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>>>> {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        self.borrow().as_ref().unwrap().parser(right_data)
    }
}

impl IntoCombinator for &ForwardRef {
    type Output = Rc<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>>;
    fn into_combinator(self) -> Self::Output {
        if let Some(a) = self.a.borrow().as_ref() {
            a.clone()
        } else {
            Rc::new(self.a.clone()).into_box_dyn().into()
        }
    }
}

pub fn forward_ref() -> ForwardRef {
    ForwardRef { a: Rc::new(RefCell::new(None)) }
}

impl ForwardRef {
    pub fn set<A: IntoCombinator<Output = B>, B: CombinatorTrait<Parser=P> + 'static, P: ParserTrait + 'static>(&mut self, a: A) -> Rc<B> {
        let a = Rc::new(a.into_combinator());
        *self.a.borrow_mut() = Some(a.clone().into_box_dyn().into());
        a
    }
}

#[macro_export]
macro_rules! forward_decls {
    ($($name:ident),* $(,)?) => {
        $(
            let mut $name = forward_ref();
        )*
    };
}