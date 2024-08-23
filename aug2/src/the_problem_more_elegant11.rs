use std::fmt::Display;
use std::marker::PhantomData;

type ParseResult = Result<bool, String>;

pub trait CombinatorTrait<'a> {
    type Parser: ParserTrait;
    fn init_parser(&'a self) -> Self::Parser where Self::Parser: 'a;
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

struct Seq<'a, L, R> {
    left: L,
    right: R,
    phantom: PhantomData<&'a ()>,
}
enum SeqParser<'a, L: CombinatorTrait<'a>, R: CombinatorTrait<'a>> {
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
impl<'b, L: CombinatorTrait<'b>, R: CombinatorTrait<'b>> CombinatorTrait<'b> for Seq<'b, L, R> {
    type Parser = SeqParser<'b, L, R>;
    fn init_parser(&'b self) -> Self::Parser {
        SeqParser::Left {
            left: self.left.init_parser(),
            right: unsafe { &self.right },
            // right: &self.right,
        }.into()
    }
}

pub struct DynCombinator<'a, T> {
    inner: T,
    phantom: PhantomData<&'a ()>,
}
impl<'b, T: CombinatorTrait<'b>> CombinatorTrait<'b> for DynCombinator<'b, T> where T: 'b {
    type Parser = Box<dyn ParserTrait + 'b>;
    fn init_parser(&'b self) -> Self::Parser {
        let inner = self.inner.init_parser();
        let boxed_dyn: Box<dyn ParserTrait + 'b> = Box::new(inner);
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
fn seq<'a>(left: impl CombinatorTrait<'a>, right: impl CombinatorTrait<'a>) -> Seq<'a, impl CombinatorTrait<'a>, impl CombinatorTrait<'a>> {
    Seq { left, right, phantom: PhantomData }
}
fn make_dyn<'a>(inner: impl CombinatorTrait<'a, Parser=impl ParserTrait + 'a> + 'a) -> Box<dyn CombinatorTrait<'a, Parser=Box<dyn ParserTrait + 'a>> + 'a> {
    Box::new(DynCombinator { inner, phantom: PhantomData })
}

#[test]
fn test() {
    let eat_a = eat('a');
    let eat_b = eat('b');
    let eat_ab = seq(eat_a, eat_b);
    let dyn_eat_ab = make_dyn(eat_ab);

    // let mut parser = dyn_eat_ab.init_parser();
    // assert_eq!(parser.parse('a'), Ok(false));
    // assert_eq!(parser.parse('b'), Ok(true));
    //
    // let mut parser = dyn_eat_ab.init_parser();
    // assert_eq!(parser.parse('a'), Ok(false));
    // assert_eq!(parser.parse('c'), Err("Expected b, got c".to_string()));

    // // Ensure the combinator can't be dropped before the parser
    // let combinator = seq(eat('a'), eat('b'));
    // let wrapped = combinator.init_parser();
    // let mut parser = wrapped;
    // // drop(combinator);

    // WTF
    let combinator = seq(eat('a'), eat('b'));
    let mut parser = combinator.init_parser();
    // drop(combinator);
    // parser.parse('a');
}