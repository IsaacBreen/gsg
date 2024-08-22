use std::fmt::Display;
use std::marker::PhantomData;

type RightData = ();
type ParseResults = ();
type UnambiguousParseResults = ();
type U8Set = ();

pub trait CombinatorTrait {
    type Parser: ParserTrait;
    fn parse<'a, 'b>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'b, 'a: 'b;
}
pub trait ParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
}

impl<T: CombinatorTrait + ?Sized> CombinatorTrait for Box<T> {
    type Parser = T::Parser;
    fn parse<'a, 'b>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'b, 'a: 'b {
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
    fn parse<'a, 'b>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'b, 'a: 'b {
        (TerminalParser, ())
    }
}
impl ParserTrait for TerminalParser {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        ()
    }
}

struct Wrapper<'inner, T> {
    inner: T,
    phantom: PhantomData<&'inner T>,
}
struct WrapperParser<'a, T: CombinatorTrait> {
    combinator: &'a T,
    inner: T::Parser,
}
impl<'inner, T: CombinatorTrait> CombinatorTrait for Wrapper<'inner, T> {
    type Parser = WrapperParser<'inner, T>;
    fn parse<'a, 'b>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'b, 'a: 'b {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (WrapperParser { combinator: &self.inner, inner }, results)
    }
}
impl<'a, T: CombinatorTrait> ParserTrait for WrapperParser<'a, T> {
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

    fn parse<'a, 'b>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'b, 'a: 'b {
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