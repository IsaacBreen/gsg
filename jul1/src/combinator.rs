use std::rc::Rc;
use crate::parse_state::{RightData, UpData};

pub trait CombinatorTrait where Self: 'static {
    type Parser: ParserTrait;
    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>);
    fn into_boxed(self) -> Box<DynCombinator> where Self: Sized {
        Box::new(DynWrapper(self))
    }
}

pub trait ParserTrait {
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>);
}

impl ParserTrait for Box<dyn ParserTrait> {
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        (**self).step(c)
    }
}

impl<C> CombinatorTrait for Rc<C> where C: CombinatorTrait + ?Sized {
    type Parser = C::Parser;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        (**self).parser(right_data)
    }
}

struct DynWrapper<T>(T);

impl<T, P> CombinatorTrait for DynWrapper<T>
where
    T: CombinatorTrait<Parser = P>,
    P: ParserTrait + 'static,
{
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (parser, right_data, up_data) = self.0.parser(right_data);
        (Box::new(parser), right_data, up_data)
    }
}

impl CombinatorTrait for Box<DynCombinator> {
    type Parser = Box<dyn ParserTrait>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        (**self).parser(right_data)
    }
}

pub type DynCombinator = dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>;

// trait IntoCombinator {
//     type Output: CombinatorTrait;
//     fn into_combinator(self) -> Self::Output;
// }
//
// impl<T> IntoCombinator for T where T: CombinatorTrait {
//     type Output = T;
//     fn into_combinator(self) -> Self::Output {
//         self
//     }
// }
//
// impl<T> IntoCombinator for &T where T: CombinatorTrait + Clone {
//     type Output = T;
//     fn into_combinator(self) -> Self::Output {
//         self
//     }
// }

// pub trait FromCombinator<T> where Self: Sized {
//     fn from_combinator(t: T) -> Self;
// }
//
// impl<T> FromCombinator<T> for T where T: CombinatorTrait {
//     fn from_combinator(t: T) -> Self {
//         t
//     }
// }
//
// impl<T> FromCombinator<&T> for T where T: CombinatorTrait + Clone {
//     fn from_combinator(t: &T) -> Self {
//         t.clone()
//     }
// }

pub trait IntoCombinator {
    type Output: CombinatorTrait;
    fn into_combinator(self) -> Self::Output;
}

impl<T> IntoCombinator for T where T: CombinatorTrait {
    type Output = T;
    fn into_combinator(self) -> Self::Output {
        self
    }
}

