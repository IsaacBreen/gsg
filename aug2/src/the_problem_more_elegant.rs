use std::fmt::Display;

pub trait CombinatorTrait {
    type Parser<'a>: ParserTrait where Self: 'a;
    fn init_parser<'a>(&'a self) -> Self::Parser<'a>;
}
pub trait ParserTrait {
    fn parse(&mut self, bytes: &[u8]) -> bool;
}

struct Eat {
    byte: u8,
}
struct EatParser;
impl CombinatorTrait for Eat {
    type Parser<'a> = EatParser;
    fn init_parser<'a>(&'a self) -> Self::Parser<'a> {
        EatParser
    }
}
impl ParserTrait for EatParser {
    fn parse(&mut self, bytes: &[u8]) -> bool {
        todo!()
    }
}

struct Seq<L: CombinatorTrait, R: CombinatorTrait> {
    left: L,
    right: R,
}
enum SeqParser<'a, L: CombinatorTrait, R: CombinatorTrait> where Self: 'a {
    Left {
        // Left parser is active, so store it
        left: L::Parser<'a>,
        // Store the right parser so we can initialize it once the left parser is done
        right: &'a R,
    },
    Right {
        right: R::Parser<'a>,
        // No need to store the left parser here - we're not going to backtrack to it
    },
    // Why not initialise both parsers at the same time? One reason is performance.
    // We'd have to initialise the whole grammar at once, even though there are many parts of it
    // that we might never touch during a particular parse.
    // A more important reason is that the parser would fail if we had any recursion in our grammar.
    // But this example is already too complex, so I'm not going to get into defining recursive
    // combinators.
}
impl<L: CombinatorTrait, R: CombinatorTrait> ParserTrait for SeqParser<'_, L, R> {
    fn parse(&mut self, bytes: &[u8]) -> bool {
        todo!()
    }
}
impl<L: CombinatorTrait, R: CombinatorTrait> CombinatorTrait for Seq<L, R> {
    type Parser<'a> = SeqParser<'a, L, R> where Self: 'a;
    fn init_parser<'a>(&'a self) -> Self::Parser<'a> {
        SeqParser::Left {
            left: self.left.init_parser(),
            right: &self.right,
        }
    }
}

pub struct DynCombinator<T> {
    inner: T,
}
pub struct DynParser<'a> {
    inner: Box<dyn ParserTrait + 'a>,
}
impl<T: CombinatorTrait> CombinatorTrait for DynCombinator<T> {
    type Parser<'a> = DynParser<'a> where Self: 'a;
    fn init_parser<'a>(&'a self) -> Self::Parser<'a> {
        let inner = self.inner.init_parser();
        DynParser { inner: Box::new(inner) }
    }
}
impl ParserTrait for DynParser<'_> {
    fn parse(&mut self, bytes: &[u8]) -> bool {
        self.inner.parse(bytes)
    }
}

// Helper functions
fn eat(byte: u8) -> impl CombinatorTrait {
    Eat { byte }
}
fn seq(left: impl CombinatorTrait, right: impl CombinatorTrait) -> impl CombinatorTrait {
    Seq { left, right }
}
fn make_dyn(inner: impl CombinatorTrait) -> impl CombinatorTrait {
    DynCombinator { inner }
}

#[test]
fn test() {
    let eat_a = Eat { byte: b'a' };
    let eat_b = Eat { byte: b'b' };
    let eat_ab = seq(eat_a, eat_b);
    let dyn_eat_ab = make_dyn(eat_ab);
    let mut parser = dyn_eat_ab.init_parser();
    assert!(parser.parse(&b"ab"[..]));
    assert!(!parser.parse(&b"ac"[..]));
}