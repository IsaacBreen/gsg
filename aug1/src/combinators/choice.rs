use std::rc::Rc;
use crate::Combinator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Choice {
    pub combinators: Rc<Vec<Combinator>>,
}

pub fn choice(combinators: Vec<Combinator>) -> Choice {
    Choice { combinators }
}

impl From<Combinator> for Choice {
    fn from(combinator: Combinator) -> Self {
        Choice { combinators: vec![combinator] }
    }
}


macro_rules! choice {
    ($($combinator:expr),*) => {
        $crate::combinators::choice(vec![$($combinator.into()),*])
    };
}