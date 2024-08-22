use std::fmt::Display;

type RightData = ();
type ParseResults = ();
type UnambiguousParseResults = ();
type U8Set = ();

pub trait CombinatorTrait {
    type Parser<'a>: ParserTrait + ?Sized where Self: Sized + 'a;
    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'a>, ParseResults) where Self::Parser<'a>: Sized;
}
pub trait ParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
}

impl<T: CombinatorTrait> CombinatorTrait for Box<T> where for<'a> T::Parser<'a>: Sized {
    type Parser<'a> = T::Parser<'a> where T: 'a;
    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'a>, ParseResults) where Self::Parser<'a>: Sized {
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
    type Parser<'a> = TerminalParser;
    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'a>, ParseResults) where Self::Parser<'a>: Sized {
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
struct WrapperParser<'a, T: CombinatorTrait + 'a> {
    combinator: &'a T,
    inner: T::Parser<'a>,
}
impl<T: CombinatorTrait> CombinatorTrait for Wrapper<T> where for<'a> T::Parser<'a>: Sized {
    type Parser<'a> = WrapperParser<'a, T> where T: 'a;
    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'a>, ParseResults) where Self::Parser<'a>: Sized {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (WrapperParser { combinator: &self.inner, inner }, results)
    }
}
impl<T: CombinatorTrait> ParserTrait for WrapperParser<'_, T> {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
    }
}

struct DynWrapper<T> {
    inner: T,
}
impl<T: CombinatorTrait> CombinatorTrait for DynWrapper<T> {
    type Parser<'a> = Box<dyn ParserTrait + 'a> where T: 'a;

    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'a>, ParseResults) where Self::Parser<'a>: Sized {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (Box::new(inner), results)
    }
}
fn dyn_wrapper<'a, T: CombinatorTrait>(inner: T) -> Box<dyn CombinatorTrait<Parser<'a> = Box<dyn ParserTrait>>> {
    let wrapper = DynWrapper { inner };
    Box::new(wrapper)
}

#[test]
fn test() {
    fn make() -> Box<dyn for<'a> CombinatorTrait<Parser<'a> = Box<dyn ParserTrait>>> {
        let terminal = dyn_wrapper(Terminal);
        let wrapper = dyn_wrapper(terminal);
        wrapper
    }

    let c = make();
    let (mut parser, _) = c.parse((), &[]);
    parser.parse(&[]);
}