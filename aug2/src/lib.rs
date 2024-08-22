use std::fmt::Display;

type RightData = ();
type ParseResults = ();
type UnambiguousParseResults = ();
type U8Set = ();

pub trait CombinatorTrait: BaseCombinatorTrait {
    type Parser: ParserTrait;
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults);
}
pub trait BaseCombinatorTrait {
    fn as_any(&self) -> &dyn std::any::Any;
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {}
}
pub trait ParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
}

struct Wrapper<T> {
    inner: T,
}
struct WrapperParser<'a, T: CombinatorTrait> {
    combinator: &'a T,
    inner: T::Parser,
}
impl<T: BaseCombinatorTrait> BaseCombinatorTrait for Wrapper<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self.inner.as_any()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        self.inner.apply_to_children(f);
    }
}
impl<'a, T: CombinatorTrait> CombinatorTrait for Wrapper<T> {
    type Parser = WrapperParser<'a, T>;

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (WrapperParser { combinator: self, inner }, results)
    }
}
impl<T: ParserTrait> ParserTrait for Wrapper<T> {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
    }
}

// struct DynWrapper<T> {
//     inner: T,
// }
// impl<T: BaseCombinatorTrait> BaseCombinatorTrait for DynWrapper<T> {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self.inner.as_any()
//     }
//     fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
//         self.inner.apply_to_children(f);
//     }
// }
// impl<T: CombinatorTrait> CombinatorTrait for DynWrapper<T> {
//     type Parser = Box<dyn ParserTrait>;
//
//     fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
//         self.inner.parse(right_data, bytes)
//     }
// }

#[test]
fn test() {}