use std::rc::Rc;
use crate::Combinator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq {
    pub combinators: Rc<Vec<Combinator>>,
}

pub fn seq(combinators: Vec<Combinator>) -> Seq {
    Seq { combinators: Rc::new(combinators) }
}

impl From<Seq> for Combinator {
    fn from(seq: Seq) -> Self {
        Combinator::Seq(seq)
    }
}

#[macro_export]
macro_rules! seq {
    ($($combinator:expr),*) => {
        $crate::combinators::seq(vec![$($combinator.into()),*])
    };
}