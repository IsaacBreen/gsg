use std::fmt::Display;

enum ParseResult {
    Ok,    // this means 'parse is ok so far, but expects to eat more bytes'
    Done,
    Err,
}

pub trait CombinatorTrait {
    type Parser<'a>: ParserTrait where Self: 'a;
    fn init_parser<'a>(&'a self) -> Self::Parser<'a>;
}
pub trait ParserTrait {
    fn parse(&mut self, byte: u8) -> ParseResult;
}

struct Eat {
    byte: u8,
}
struct EatParser {
    byte: u8,
}
impl CombinatorTrait for Eat {
    type Parser<'a> = EatParser;
    fn init_parser<'a>(&'a self) -> Self::Parser<'a> {
        EatParser { byte: self.byte }
    }
}
impl ParserTrait for EatParser {
    fn parse(&mut self, byte: u8) -> ParseResult {
        if byte == self.byte {
            ParseResult::Done
        } else {
            ParseResult::Err

    }
}

struct Seq<L: CombinatorTrait, R: CombinatorTrait> {
    left: L,
    right: R,
}
enum SeqParser<'a, L: CombinatorTrait, R: CombinatorTrait> where Self: 'a {
    Left {
        left: L::Parser<'a>,
        right: &'a R,
    },
    Right {
        right: R::Parser<'a>,
    },
}
impl<L: CombinatorTrait, R: CombinatorTrait> ParserTrait for SeqParser<'_, L, R> {
    fn parse(&mut self, byte: u8) -> ParseResult {
        match self {
            SeqParser::Left { left, right } => {
                let result = left.parse(byte);
                *self = SeqParser::Right {
                    right: right.init_parser(),
                };
                result
            }
            SeqParser::Right { right } => {
                right.parse(byte)
            }
        }
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
    type Parser<'a> = Box<dyn ParserTrait + 'a> where Self: 'a;
    fn init_parser<'a>(&'a self) -> Self::Parser<'a> {
        let inner = self.inner.init_parser();
        Box::new(inner)
    }
}
impl ParserTrait for Box<dyn ParserTrait + '_> {
    fn parse(&mut self, byte: u8) -> ParseResult {
        (**self).parse(byte)
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
    assert!(parser.parse(b'a'));
    assert!(parser.parse(b'b'));

    let mut parser = dyn_eat_ab.init_parser();
    assert!(parser.parse(b'a'));
    assert!(!parser.parse(b'c'));
}