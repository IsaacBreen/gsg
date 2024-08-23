struct Eat {
    bytes: Vec<u8>,
}

struct Seq {
    children: Vec<Box<dyn Combinator>>,
}

struct Choice {
    children: Vec<Box<dyn Combinator>>,
}

struct Rule {
    name: String,
}

trait Combinator {
    fn parse<'a>(&'a self, input: &[u8]) -> ParseResults;
}

trait Parser {
    fn parse<'a>(&'a self, input: &[u8]) -> ParseResults;
}

struct ParseResults {
    combinator_stacks: Vec<Vec<dyn Combinator>>,
    parser_stacks: Vec<(Box<dyn Parser>, Vec<Box<dyn Combinator>>)>,
}

struct ParseState {
    head: Option<Box<dyn Parser>>,
    tail: Vec<Box<dyn Combinator>>,
}

impl ParseState {
    fn
}

fn parse(combinator: &impl Combinator, input: &[u8]) {


}