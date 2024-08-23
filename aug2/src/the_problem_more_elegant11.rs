use std::fmt::Display;
use std::marker::PhantomData;

type ParseResult = Result<bool, String>;

struct Wrapper<'a, T> {
    inner: T,
    phantom: PhantomData<&'a ()>,
    // marker: DropMarker<'a>,
}
impl<T> Drop for Wrapper<'_, T> {
    fn drop(&mut self) {}
}
impl<'a, T> Wrapper<'a, T> {
    fn new(inner: T) -> Self {
        Wrapper {
            inner,
            phantom: PhantomData,
            // marker: DropMarker { phantom: PhantomData }
        }
    }
    fn into_inner(self) -> T where T: 'a {
        unsafe { std::ptr::read(&self.into_inner()) }
    }
}

struct DropMarker<'a> {
    phantom: PhantomData<&'a ()>,
}
impl Drop for DropMarker<'_> {
    fn drop(&mut self) {}
}

impl<'a, T> From<T> for Wrapper<'a, T> {
    fn from(inner: T) -> Self {
        Wrapper {
            inner,
            phantom: PhantomData,
            // marker: DropMarker { phantom: PhantomData },
        }
    }
}

pub trait CombinatorTrait {
    type Parser: ParserTrait;
    fn init_parser<'a>(&'a self) -> Wrapper<Self::Parser>;
}
pub trait ParserTrait {
    fn parse(&mut self, c: char) -> ParseResult;
}

struct Eat {
    c: char,
}
struct EatParser {
    c: char,
}
impl CombinatorTrait for Eat {
    type Parser = EatParser;
    fn init_parser<'a>(&'a self) -> Wrapper<Self::Parser> {
        Wrapper::from(EatParser { c: self.c })
    }
}
impl ParserTrait for EatParser {
    fn parse(&mut self, c: char) -> ParseResult {
        if c == self.c {
            Ok(true)
        } else {
            Err(format!("Expected {}, got {}", self.c, c))
        }
    }
}

struct Seq<'b, L: CombinatorTrait, R: CombinatorTrait> {
    left: L,
    right: R,
    phantom: PhantomData<&'b ()>,
}
enum SeqParser<'a, L: CombinatorTrait, R: CombinatorTrait> {
    Left {
        left: L::Parser,
        right: &'a R,
    },
    Right {
        right: R::Parser,
    },
    Done,
}
impl<L: CombinatorTrait, R: CombinatorTrait> ParserTrait for SeqParser<'_, L, R> {
    fn parse(&mut self, c: char) -> ParseResult {
        match self {
            SeqParser::Left { left, right } => {
                let mut result = left.parse(c);
                if let Ok(true) = result {
                    result = Ok(false);
                    *self = SeqParser::Right {
                        right: right.init_parser().into_inner(),
                    };
                } else {
                    *self = SeqParser::Done;
                }
                result
            }
            SeqParser::Right { right } => {
                let result = right.parse(c);
                *self = SeqParser::Done;
                result
            }
            SeqParser::Done => {
                Err("Sequence already exhausted".to_string())
            }
        }
    }
}
impl<'b, L: CombinatorTrait, R: CombinatorTrait + 'b> CombinatorTrait for Seq<'b, L, R> where R: 'b {
    type Parser = SeqParser<'b, L, R>;
    fn init_parser<'a>(&'a self) -> Wrapper<Self::Parser> {
        SeqParser::Left {
            left: self.left.init_parser().into_inner(),
            right: unsafe { std::mem::transmute(&self.right) },
            // right: &self.right,
        }.into()
    }
}

pub struct DynCombinator<'a, T> {
    inner: T,
    phantom: PhantomData<&'a ()>,
}
impl<'b, T: CombinatorTrait> CombinatorTrait for DynCombinator<'b, T> where T: 'b {
    type Parser = Box<dyn ParserTrait + 'b>;
    fn init_parser<'a>(&'a self) -> Wrapper<Self::Parser> {
        let inner = self.inner.init_parser().into_inner();
        let boxed_dyn: Box<dyn ParserTrait + 'b> = Box::new(inner);
        Wrapper::from(boxed_dyn)
    }
}
impl ParserTrait for Box<dyn ParserTrait + '_> {
    fn parse(&mut self, c: char) -> ParseResult {
        (**self).parse(c)
    }
}

// Helper functions
fn eat(c: char) -> Eat {
    Eat { c }
}
fn seq<'a>(left: impl CombinatorTrait, right: impl CombinatorTrait) -> Seq<'a, impl CombinatorTrait, impl CombinatorTrait> {
    Seq { left, right, phantom: PhantomData }
}
fn make_dyn<'a>(inner: impl CombinatorTrait<Parser=impl ParserTrait + 'a> + 'a) -> Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait + 'a>> + 'a> {
    Box::new(DynCombinator { inner, phantom: PhantomData })
}

#[test]
fn test() {
    let eat_a = eat('a');
    let eat_b = eat('b');
    let eat_ab = seq(eat_a, eat_b);
    let dyn_eat_ab = make_dyn(eat_ab);

    let mut parser = dyn_eat_ab.init_parser().into_inner();
    assert_eq!(parser.parse('a'), Ok(false));
    assert_eq!(parser.parse('b'), Ok(true));

    let mut parser = dyn_eat_ab.init_parser().into_inner();
    assert_eq!(parser.parse('a'), Ok(false));
    assert_eq!(parser.parse('c'), Err("Expected b, got c".to_string()));

    // Ensure the combinator can't be dropped before the parser
    let combinator = seq(eat('a'), eat('b'));
    let wrapped = combinator.init_parser();
    let mut parser = wrapped.into_inner();
    // drop(combinator);

    // WTF
    let combinator = seq(eat('a'), eat('b'));
    let mut parser = combinator.init_parser().into_inner();
    drop(combinator);
    parser.parse('a');
}