use std::rc::Rc;
use crate::Combinator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Choice {
    pub combinators: Rc<Vec<Combinator>>,
}

pub fn choice(combinators: Vec<Combinator>) -> Choice {
    Choice { combinators: Rc::new(combinators) }
}

impl From<Choice> for Combinator {
    fn from(choice: Choice) -> Self {
        Combinator::Choice(choice)
    }
}


macro_rules! choice {
    ($($combinator:expr),*) => {
        $crate::combinators::choice(vec![$($combinator.into()),*])
    };
}