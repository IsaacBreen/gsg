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

struct Terminal;
struct TerminalParser;

// struct Wrapper<T> {
//     inner: T,
// }
// struct WrapperParser<'a, T: CombinatorTrait> {
//     combinator: &'a T,
//     inner: T::Parser,
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
impl<T: CombinatorTrait> CombinatorTrait for DynWrapper<T> where T::Parser: 'static {
    type Parser = Box<dyn ParserTrait>;

    fn parse<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) where Self::Parser: 'a {
        let (inner, results) = self.inner.parse(right_data, bytes);
        (Box::new(inner), results)
    }
}
fn dyn_wrapper<T: CombinatorTrait + 'static>(inner: T) -> Box<dyn CombinatorTrait<Parser = Box<dyn ParserTrait>>> {
    Box::new(DynWrapper { inner })
}

#[test]
fn test() {
    let terminal = dyn_wrapper(Terminal);
    let mut parser = terminal.parse((), &[]);
    assert_eq!(parser.parse(&[]), ());
}