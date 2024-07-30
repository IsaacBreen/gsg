use std::rc::Rc;
use crate::Combinator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seq {
    pub combinators: Rc<Vec<Combinator>>,
}

pub fn seq(combinators: Vec<Combinator>) -> Seq {
    Seq { combinators }
}

impl From<Combinator> for Seq {
    fn from(combinator: Combinator) -> Self {
        Seq { combinators: vec![combinator] }
    }
}

#[macro_export]
macro_rules! seq {
    ($($combinator:expr),*) => {
        $crate::combinators::seq(vec![$($combinator.into()),*])
    };
}