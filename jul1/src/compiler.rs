use std::rc::Rc;
use std::collections::HashMap;
use crate::*;

pub trait Compile {
    fn compile(self) -> Combinator;
}

impl Compile for Combinator {
    fn compile(self) -> Combinator {
        return self;
    }
}

impl Compile for Seq {
    fn compile(self) -> Combinator {
        return Combinator::Seq(self);
    }
}

impl Compile for Choice {
    fn compile(self) -> Combinator {
        return Combinator::Choice(self);
    }
}

impl Compile for EatU8 {
    fn compile(self) -> Combinator {
        Combinator::EatU8(self)
    }
}

impl Compile for EatString {
    fn compile(self) -> Combinator {
        Combinator::EatString(self)
    }
}

impl Compile for Eps {
    fn compile(self) -> Combinator {
        Combinator::Eps(self)
    }
}

impl Compile for Fail {
    fn compile(self) -> Combinator {
        Combinator::Fail(self)
    }
}

impl Compile for Repeat1 {
    fn compile(self) -> Combinator {
        let compiled_a = self.a.as_ref().clone().compile();
        match compiled_a {
            Combinator::Fail(_) => Combinator::Fail(Fail),
            _ => Combinator::Repeat1(Repeat1 { a: Rc::new(compiled_a), greedy: self.greedy }),
        }
    }
}

impl Compile for EatByteStringChoice {
    fn compile(self) -> Combinator {
        Combinator::EatByteStringChoice(self)
    }
}
