use std::fmt::Display;

type RightData = ();
type ParseResults = ();
type UnambiguousParseResults = ();
type U8Set = ();

pub trait CombinatorTrait {
    type Parser: ParserTrait;
    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'a;
}
pub trait ParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
}

impl<T: CombinatorTrait + ?Sized> CombinatorTrait for Box<T> {
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

struct Terminal;
struct TerminalParser;
impl CombinatorTrait for Terminal {
    type Parser = TerminalParser;
    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'a {
        (TerminalParser, ())
    }
}
impl ParserTrait for TerminalParser {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        ()
    }
}

struct Wrapper<T> {
    inner: T,
}
struct WrapperParser<'a, T: CombinatorTrait> {
    combinator: &'a T,
    inner: T::Parser,
}
impl<T: CombinatorTrait> CombinatorTrait for Wrapper<T> {
    type Parser = WrapperParser<'b, T>;
    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'a {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (WrapperParser { combinator: &self.inner, inner }, results)
    }
}
impl<'a, T: CombinatorTrait> ParserTrait for WrapperParser<'a, T> {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
    }
}
fn wrapper<T: CombinatorTrait>(inner: T) -> Wrapper<T> {
    Wrapper { inner }
}

struct DynWrapper<T> {
    inner: T,
}
impl<'b, T: CombinatorTrait> CombinatorTrait for DynWrapper<T> {
    type Parser = Box<dyn ParserTrait + 'b>;

    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'a {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (Box::new(inner), results)
    }
}
fn dyn_wrapper(inner: impl CombinatorTrait) -> impl CombinatorTrait<Parser = impl ParserTrait> {
    let wrapper = DynWrapper { inner };
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