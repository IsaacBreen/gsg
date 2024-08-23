use std::fmt::Display;
use std::marker::PhantomData;

type ParseResult = Result<bool, String>;

struct Wrapper<'a, T> {
    inner: T,
    marker: DropMarker<'a>,
}
// impl<'a, T> Drop for Wrapper<'a, T> {
//     fn drop(&mut self) {}
// }

struct DropMarker<'a> {
    phantom: PhantomData<&'a ()>,
}
impl Drop for DropMarker<'_> {
    fn drop(&mut self) {}
}

impl<'a, T> From<T> for Wrapper<'_, T> {
    fn from(inner: T) -> Self {
        Wrapper {
            inner,
            marker: DropMarker { phantom: PhantomData },
        }
    }
}

pub trait CombinatorTrait {
    type Parser: ParserTrait;
    fn init_parser<'a, 'b>(&'a self) -> Self::Parser where Self::Parser: 'b, 'a: 'b;
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
    fn init_parser<'a, 'b>(&'a self) -> Self::Parser where Self::Parser: 'b, 'a: 'b {
        EatParser { c: self.c }
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
                        right: right.init_parser(),
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
impl<'c, L: CombinatorTrait, R: CombinatorTrait + 'c> CombinatorTrait for Seq<'c, L, R> where R: 'c
{
    type Parser = SeqParser<'c, L, R>;
    fn init_parser<'a, 'b>(&'a self) -> Self::Parser where Self::Parser: 'b, 'a: 'b
    {
        SeqParser::Left {
            left: self.left.init_parser(),
            right: unsafe { std::mem::transmute(&self.right) },
            // right: &self.right,
        }.into()
    }
}

pub struct DynCombinator<'a, T> {
    inner: T,
    phantom: PhantomData<&'a ()>,
}
impl<'c, T: CombinatorTrait> CombinatorTrait for DynCombinator<'c, T> where T: 'c
{
    type Parser = Box<dyn ParserTrait + 'c>;
    fn init_parser<'a, 'b>(&'a self) -> Self::Parser where Self::Parser: 'b, 'a: 'b
    {
        let inner = self.init_parser();
        let boxed_dyn: Box<dyn ParserTrait> = Box::new(inner);
        boxed_dyn
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

    let mut parser = dyn_eat_ab.init_parser();
    assert_eq!(parser.parse('a'), Ok(false));
    assert_eq!(parser.parse('b'), Ok(true));

    let mut parser = dyn_eat_ab.init_parser();
    assert_eq!(parser.parse('a'), Ok(false));
    assert_eq!(parser.parse('c'), Err("Expected b, got c".to_string()));

    // Ensure the combinator can't be dropped before the parser
    let combinator = seq(eat('a'), eat('b'));
    let wrapped = combinator.init_parser();
    let mut parser = wrapped;
    // drop(combinator);

    // WTF
    let combinator = seq(eat('a'), eat('b'));
    let mut parser = combinator.init_parser();
    drop(combinator);
    parser.parse('a');
}