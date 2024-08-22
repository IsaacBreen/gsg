use std::fmt::Display;

type RightData = ();
type ParseResults = ();
type UnambiguousParseResults = ();
type U8Set = ();

pub trait CombinatorTrait: BaseCombinatorTrait {
    type Parser: ParserTrait;
    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'a;
}
pub trait BaseCombinatorTrait {
    fn as_any(&self) -> &dyn std::any::Any;
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {}
}
pub trait ParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
}

impl<T: BaseCombinatorTrait> BaseCombinatorTrait for Box<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self.as_ref().as_any()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        self.as_ref().apply_to_children(f);
    }
}
impl<T: CombinatorTrait> CombinatorTrait for Box<T> {
    type Parser = T::Parser;

    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'a {
        self.as_ref().parse(right_data, bytes)
    }
}
impl<T: ParserTrait + ?Sized> ParserTrait for Box<T> {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.as_mut().parse(bytes)
    }
}

// struct Wrapper<T> {
//     inner: T,
// }
// struct WrapperParser<'a, T: CombinatorTrait> {
//     combinator: &'a T,
//     inner: T::Parser,
// }
// impl<T: BaseCombinatorTrait> BaseCombinatorTrait for Wrapper<T> {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self.inner.as_any()
//     }
//     fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
//         self.inner.apply_to_children(f);
//     }
// }
// impl<T: CombinatorTrait> CombinatorTrait for Wrapper<T> {
//     type Parser = WrapperParser<T>;
//
//     fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
//         let (inner, results) = self.inner.parse(right_data, bytes);
//         (WrapperParser { combinator: self, inner }, results)
//     }
// }
// impl<T: ParserTrait> ParserTrait for Wrapper<T> {
//     fn parse(&mut self, bytes: &[u8]) -> ParseResults {
//         self.inner.parse(bytes)
//     }
// }

struct DynWrapper<T> {
    inner: T,
}
impl<T: BaseCombinatorTrait> BaseCombinatorTrait for DynWrapper<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self.inner.as_any()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        self.inner.apply_to_children(f);
    }
}
impl<'b, T: CombinatorTrait + 'b> CombinatorTrait for DynWrapper<T> where T::Parser: 'b, Self: 'b {
    type Parser = Box<dyn ParserTrait>;

    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'a {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (Box::new(inner), results)
    }
}
// fn dyn_wrapper<T: CombinatorTrait>(inner: T) -> Box<dyn CombinatorTrait<Parser = Box<dyn ParserTrait>>> {
//     Box::new(DynWrapper { inner })
// }

#[test]
fn test() {}