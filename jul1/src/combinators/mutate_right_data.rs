use std::any::Any;
use std::rc::Rc;
use crate::*;

#[derive(PartialEq)]
pub struct MutateRightData<F: Fn(&mut RightData) -> bool> {
    pub run: Rc<F>,
}

impl<F: Fn(&mut RightData) -> bool + 'static> CombinatorTrait for MutateRightData<F> {
    type Parser = MutateRightData<F>;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        if (self.run)(&mut right_data) {
            (MutateRightData { run: self.run.clone() }, ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
                cut: false,
                done: true
            })
        } else {
            (MutateRightData { run: self.run.clone() }, ParseResults {
                right_data_vec: vec![],
                up_data_vec: vec![],
                cut: false,
                done: true,
            })
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<F: Fn(&mut RightData) -> bool + 'static> ParserTrait for MutateRightData<F> {
    fn step(&mut self, c: u8) -> ParseResults {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
            done: true,
        }
    }

    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            std::ptr::eq(self, other)
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn mutate_right_data<F: Fn(&mut RightData) -> bool>(run: F) -> MutateRightData<F> {
    MutateRightData { run: Rc::new(run) }
}
