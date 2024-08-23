use std::fmt::Display;
use std::marker::PhantomData;

type ParseResult = Result<bool, String>;

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

struct Seq<L: CombinatorTrait, R: CombinatorTrait> {
    left: L,
    right: R,
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
impl<L: CombinatorTrait, R: CombinatorTrait + 'static> CombinatorTrait for Seq<L, R> {
    type Parser = SeqParser<'static, L, R>;
    fn init_parser<'a, 'b>(&'a self) -> Self::Parser where Self::Parser: 'b, 'a: 'b {
        SeqParser::Left {
            left: self.left.init_parser(),
            right: unsafe { std::mem::transmute(&self.right) },
        }
    }
}

pub struct DynCombinator<T> {
    inner: T,
}
pub struct DynParser<'a> {
    inner: Box<dyn ParserTrait + 'a>,
}
impl<T: CombinatorTrait + 'static> CombinatorTrait for DynCombinator<T> {
    type Parser = Box<dyn ParserTrait>;
    fn init_parser<'a, 'b>(&'a self) -> Self::Parser where Self::Parser: 'b, 'a: 'b {
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
fn seq(left: impl CombinatorTrait, right: impl CombinatorTrait + 'static) -> Seq<impl CombinatorTrait, impl CombinatorTrait> {
    Seq { left, right }
}
fn make_dyn(inner: impl CombinatorTrait + 'static) -> Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>> {
    Box::new(DynCombinator { inner })
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
    let mut parser = combinator.init_parser();
    drop(combinator);
    assert_eq!(parser.parse('a'), Ok(false));
    assert_eq!(parser.parse('b'), Ok(true));

    let mut parser;
    {
        let combinator = seq(eat('a'), eat('b'));
        parser = combinator.init_parser();
    }
    // miri should report:
    // 143 |     let right_combinator = if let SeqParser::Left { ref left, right } = parser {
    //     |                                                               ^^^^^ constructing invalid value: encountered a dangling reference (use-after-free)
    let right_combinator = if let SeqParser::Left { ref left, right } = parser {
        right
    } else {
        unreachable!()
    };
    let mut right_parser1 = right_combinator.init_parser();
    let mut right_parser2 = right_combinator.init_parser();
    let mut right_parser3 = right_combinator.init_parser();

    assert_eq!(parser.parse('a'), Ok(false));
    assert_eq!(parser.parse('b'), Ok(true));

    assert_eq!(right_parser1.parse('b'), Ok(true));

    assert_eq!(right_parser2.parse('b'), Ok(true));

    assert_eq!(right_parser3.parse('b'), Ok(true));
}