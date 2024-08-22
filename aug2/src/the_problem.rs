use std::fmt::Display;

type RightData = ();
type ParseResults = ();
type UnambiguousParseResults = ();
type U8Set = ();

pub trait CombinatorTrait<'a> {
    type Parser: ParserTrait where Self: 'a;
    fn parse(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults);
}
pub trait ParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
}

impl<'a, T: CombinatorTrait<'a> + 'a + ?Sized> CombinatorTrait<'a> for Box<T> {
    type Parser = T::Parser;
    fn parse(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
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
impl<'a> CombinatorTrait<'a> for Terminal {
    type Parser = TerminalParser;
    fn parse(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
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
struct WrapperParser<'a, T: CombinatorTrait<'a>> {
    combinator: &'a T,
    inner: T::Parser,
}
impl<'a, T: CombinatorTrait<'a> + 'a> CombinatorTrait<'a> for Wrapper<T> {
    type Parser = WrapperParser<'a, T>;
    fn parse(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (WrapperParser { combinator: &self.inner, inner }, results)
    }
}
impl<'a, T: CombinatorTrait<'a> + 'a> ParserTrait for WrapperParser<'a, T> {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
    }
}
fn wrapper<'a, T: CombinatorTrait<'a> + 'a>(inner: T) -> Wrapper<T> {
    Wrapper { inner }
}

struct DynWrapper<T> {
    inner: T,
}
impl<'a, T: CombinatorTrait<'a> + 'a> CombinatorTrait<'a> for DynWrapper<T> {
    type Parser = Box<dyn ParserTrait + 'a>;

    fn parse(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (Box::new(inner), results)
    }
}
fn dyn_wrapper<'a, T: CombinatorTrait<'a> + 'a>(inner: T) -> Box<dyn CombinatorTrait<'a, Parser = Box<dyn ParserTrait + 'a>> + 'a> {
    let wrapper = DynWrapper { inner };
    Box::new(wrapper)
}

#[test]
fn test() {
    fn make<'a>() -> impl CombinatorTrait<'a> {
        Terminal
    }

    let c = make();
    let (mut parser, _) = c.parse((), &[]);
    parser.parse(&[]);
}