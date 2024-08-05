use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Opt {
    pub(crate) inner: Box<Combinator>,
    pub(crate) greedy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OptParser {
    pub(crate) inner: Option<Box<Parser>>,
    pub(crate) greedy: bool,
}

impl CombinatorTrait for Opt {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, mut parse_results) = self.inner.parse(right_data, bytes);
        if self.greedy && parse_results.succeeds_decisively() {
            (Parser::OptParser(OptParser { inner: None, greedy: self.greedy }), parse_results)
        } else {
            (Parser::OptParser(OptParser { inner: Some(Box::new(parser)), greedy: self.greedy }), parse_results)
        }
    }
}

impl ParserTrait for OptParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.as_ref().map(|p| p.get_u8set()).unwrap_or_default()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        if let Some(parser) = &mut self.inner {
            let mut parse_results = parser.parse(bytes);
            if self.greedy && parse_results.succeeds_decisively() {
                self.inner = None;
            }
            parse_results.squash();
            parse_results
        } else {
            ParseResults::empty_finished()
        }
    }
}

pub fn opt(a: impl Into<Combinator>) -> Combinator {
    profile_internal("opt", Opt { inner: Box::new(a.into()), greedy: false })
}

pub fn opt_greedy(a: impl Into<Combinator>) -> Combinator {
    profile_internal("opt_greedy", Opt { inner: Box::new(a.into()), greedy: true })
}

impl From<Opt> for Combinator {
    fn from(value: Opt) -> Self {
        Combinator::Opt(value)
    }
}