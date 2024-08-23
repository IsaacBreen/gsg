use std::fmt::Display;
use std::marker::PhantomData;

type ParseResult = Result<bool, String>;

pub trait CombinatorTrait<'a> {
    type Parser: ParserTrait + 'a where Self: 'a;
    fn init_parser(&'a self) -> Self::Parser;
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
impl<'a> CombinatorTrait<'a> for Eat {
    type Parser = EatParser;
    fn init_parser(&'a self) -> Self::Parser {
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

struct Seq<'a, L: CombinatorTrait<'a>, R: CombinatorTrait<'a>> {
    left: L,
    right: R,
    phantom: PhantomData<&'a ()>,
}
enum SeqParser<'a, L: CombinatorTrait<'a>, R: CombinatorTrait<'a>> where Self: 'a {
    Left {
        left: L::Parser,
        right: &'a R,
    },
    Right {
        right: R::Parser,
    },
    Done,
}
impl<'a, L: CombinatorTrait<'a>, R: CombinatorTrait<'a>> ParserTrait for SeqParser<'a, L, R> {
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
impl<'a, L: CombinatorTrait<'a>, R: CombinatorTrait<'a>> CombinatorTrait<'a> for Seq<'a, L, R> {
    type Parser = SeqParser<'a, L, R> where Self: 'a;
    fn init_parser(&'a self) -> Self::Parser {
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
impl<'a, T: CombinatorTrait<'a>> CombinatorTrait<'a> for DynCombinator<T> {
    type Parser = Box<dyn ParserTrait + 'a> where Self: 'a;
    fn init_parser(&'a self) -> Self::Parser {
        let inner = self.inner.init_parser();
        Box::new(inner)
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
fn seq<'a>(left: impl CombinatorTrait<'a>, right: impl CombinatorTrait<'a>) -> impl CombinatorTrait<'a> {
    Seq { left, right, phantom: PhantomData }
}
fn make_dyn<'a>(inner: impl CombinatorTrait<'a>) -> impl CombinatorTrait<'a> {
    // let boxed_dyn: Box<dyn for<'a> CombinatorTrait<Parser=Box<dyn ParserTrait>>> = Box::new(DynCombinator { inner });
    // boxed_dyn
    todo!();
    inner
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
}