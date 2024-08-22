use std::fmt::Display;
use std::marker::PhantomData;

type RightData = ();
type ParseResults = ();
type UnambiguousParseResults = ();
type U8Set = ();

pub trait CombinatorTrait {
    type Parser: ParserTrait;
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults);
}
pub trait ParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
}

impl<T: CombinatorTrait + ?Sized> CombinatorTrait for Box<T> {
    type Parser = T::Parser;
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        self.as_ref().parse(right_data, bytes)
    }
}
impl<T: ParserTrait + ?Sized> ParserTrait for Box<T> {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.as_mut().parse(bytes)
    }
}

struct Terminal;
struct TerminalParser;
impl CombinatorTrait for Terminal {
    type Parser = TerminalParser;
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        (TerminalParser, ())
    }
}
impl ParserTrait for TerminalParser {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        ()
    }
}

struct Wrapper<'inner, T: CombinatorTrait> {
    inner: T,
    phantom: PhantomData<&'inner T>,
}
struct WrapperParser<'outer, T: CombinatorTrait> {
    combinator: &'outer Wrapper<'outer, T>,
    inner: T::Parser,
}
impl<'outer, T: CombinatorTrait> CombinatorTrait for Wrapper<'outer, T> {
    type Parser = WrapperParser<'outer, T>;
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        let (inner, results) = self.inner.parse(right_data, bytes);
        // (WrapperParser { combinator: self, inner }, results)
        todo!()
    }
}
impl<'a, 'outer, T: CombinatorTrait> ParserTrait for WrapperParser<'outer, T> {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
    }
}
fn wrapper<'b, T: CombinatorTrait>(inner: T) -> Wrapper<'b, T> {
    Wrapper { inner, phantom: PhantomData }
}

struct DynWrapper<'inner, T> {
    inner: T,
    phantom: PhantomData<&'inner T>,
}
impl<'inner, T: CombinatorTrait> CombinatorTrait for DynWrapper<'inner, T> {
    type Parser = Box<dyn ParserTrait + 'inner>;

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (Box::new(inner), results)
    }
}
fn dyn_wrapper<'a, T: CombinatorTrait + 'a>(inner: T) -> impl CombinatorTrait<Parser = Box<dyn ParserTrait + 'a>> {
    let wrapper = DynWrapper { inner, phantom: PhantomData };
    Box::new(wrapper)
}

#[test]
fn test() {
    // fn make() -> impl for<'a> CombinatorTrait<'a> {
    //     Terminal
    // }

    fn make() -> impl CombinatorTrait {
        dyn_wrapper(Terminal)
    }

    let c = make();
    let (mut parser, _) = c.parse((), &[]);
    parser.parse(&[]);
    drop(parser);
}