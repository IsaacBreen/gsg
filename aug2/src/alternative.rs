type ParseResult = Result<bool, String>;

trait Combinator: BaseCombinator {
    type Parser<'a>: Parser where Self: 'a;
    fn init_parser(&self) -> Self::Parser<'_>;
}

trait BaseCombinator {
    fn init_parser_dyn(&self) -> Box<dyn Parser + '_>;
}

trait Parser {
    fn parse(&mut self, c: char) -> ParseResult;
}

struct Seq<L, R> {
    l: L,
    r: R,
}

enum SeqParser<'a, L, R> where L: Combinator + 'a, R: Combinator {
    Left {
        l: L::Parser<'a>,
        r: &'a R,
    },
    Right {
        r: R::Parser<'a>,
    },
    Done,
}

impl<L, R> Combinator for Seq<L, R> where L: Combinator, R: Combinator {
    type Parser<'a> = SeqParser<'a, L, R>
    where
        Self: 'a;

    fn init_parser(&self) -> Self::Parser<'_> {
        SeqParser::Left {
            l: self.l.init_parser(),
            r: unsafe { std::mem::transmute(&self.r) },
        }
    }
}

impl<L, R> BaseCombinator for Seq<L, R> where L: Combinator, R: Combinator {
    fn init_parser_dyn(&self) -> Box<dyn Parser + '_> {
        Box::new(self.init_parser())
    }
}

impl<'a, L, R> Parser for SeqParser<'a, L, R> where L: Combinator + 'a, R: Combinator
{
    fn parse(&mut self, c: char) -> ParseResult {
        match self {
            SeqParser::Left { l, r } => {
                let mut result = l.parse(c);
                if let Ok(true) = result {
                    result = Ok(false);
                    *self = SeqParser::Right {
                        r: r.init_parser(),
                    };
                } else {
                    *self = SeqParser::Done;
                }
                result
            }
            SeqParser::Right { r } => {
                let result = r.parse(c);
                *self = SeqParser::Done;
                result
            }
            SeqParser::Done => {
                Err("Sequence already exhausted".to_string())
            }
        }
    }
}

struct Eat {
    c: char,
}
struct EatParser {
    c: char,
}
impl Combinator for Eat {
    type Parser<'a> = EatParser;
    fn init_parser(&self) -> Self::Parser<'_> {
        EatParser { c: self.c }
    }
}
impl BaseCombinator for Eat {
    fn init_parser_dyn(&self) -> Box<dyn Parser + '_> {
        Box::new(self.init_parser())
    }
}
impl Parser for EatParser {
    fn parse(&mut self, c: char) -> ParseResult {
        if c == self.c {
            Ok(true)
        } else {
            Err(format!("Expected {}, got {}", self.c, c))
        }
    }
}

struct DynCombinator<T> {
    inner: T,
}
impl<T: Combinator> Combinator for DynCombinator<T> {
    type Parser<'a> = Box<dyn Parser + 'a> where Self: 'a;
    fn init_parser(&self) -> Self::Parser<'_> {
        let inner = self.inner.init_parser();
        let boxed_dyn: Box<dyn Parser + '_> = Box::new(inner);
        boxed_dyn
    }
}
impl<T: Combinator> BaseCombinator for DynCombinator<T> {
    fn init_parser_dyn(&self) -> Box<dyn Parser + '_> {
        Box::new(self.init_parser())
    }
}
impl Parser for Box<dyn Parser + '_> {
    fn parse(&mut self, c: char) -> ParseResult {
        (**self).parse(c)
    }
}

fn eat(c: char) -> Eat {
    Eat { c }
}
fn seq<'a>(left: impl Combinator + 'a, right: impl Combinator + 'a) -> Seq<impl Combinator + 'a, impl Combinator + 'a> {
    Seq { l: left, r: right }
}
fn make_dyn<'a>(inner: impl Combinator + 'a) -> Box<dyn BaseCombinator + 'a> {
    Box::new(DynCombinator { inner })
}

#[test]
fn test() {
    let eat_a = eat('a');
    let eat_b = eat('b');
    let eat_ab = seq(eat_a, eat_b);
    let dyn_eat_ab = make_dyn(eat_ab);

    let mut parser = dyn_eat_ab.init_parser_dyn();
    assert_eq!(parser.parse('a'), Ok(false));
    assert_eq!(parser.parse('b'), Ok(true));

    // let mut parser = dyn_eat_ab;
    // assert_eq!(parser.parse('a'), Ok(false));
    // assert_eq!(parser.parse('c'), Err("Expected b, got c".to_string()));
    //
    // // Ensure the combinator can't be dropped before the parser
    // let combinator = seq(eat('a'), eat('b'));
    // let wrapped = combinator.init_parser();
    // let mut parser = wrapped;
    // // drop(combinator);
    //
    // WTF
    let combinator = make_dyn(seq(eat('a'), eat('b')));
    let mut parser = combinator.init_parser_dyn();
    // drop(parser);
    // drop(combinator);
    parser.parse('a');
}
