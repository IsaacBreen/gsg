use std::fmt::Display;

// Removed Parser enum
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

impl<T: CombinatorTrait> CombinatorTrait for Box<T> {
    type Parser = T::Parser;

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        (**self).parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for Box<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        (**self).as_any()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        (**self).apply_to_children(f);
    }
}

struct Wrapper<T> {
    inner: T,
}
struct WrapperParser<T> {
    inner: T,
}

impl<T: BaseCombinatorTrait> BaseCombinatorTrait for Wrapper<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self.inner.as_any()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        self.inner.apply_to_children(f);
    }
}
impl<T: ParserTrait> ParserTrait for Wrapper<T> {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
    }
}

#[test]
fn test() {}