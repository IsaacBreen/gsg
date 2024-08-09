use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LookaheadContext<'a> {
    pub inner: Box<Combinator<'a>>,
    pub persist_with_partial_lookahead: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LookaheadContextParser<'a> {
    pub inner: Box<Parser<'a>>,
    pub persist_with_partial_lookahead: bool,
}

impl CombinatorTrait<'_> for LookaheadContext<'_> {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (inner, parse_results) = self.inner.parse(right_data, bytes);
        (Parser::LookaheadContextParser(LookaheadContextParser { inner: Box::new(inner), persist_with_partial_lookahead: self.persist_with_partial_lookahead }), parse_results)
    }
}

impl ParserTrait for LookaheadContextParser<'_> {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.inner.parse(bytes)
    }
}

pub fn lookahead_context<'a>(inner: impl Into<Combinator<'a>>) -> LookaheadContext<'a> {
    LookaheadContext { inner: Box::new(inner.into()), persist_with_partial_lookahead: false }
}

impl From<LookaheadContext<'_>> for Combinator<'_> {
    fn from(lookahead_context: LookaheadContext) -> Self {
        Self::LookaheadContext(lookahead_context)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartialLookahead<'a> {
    pub parser: Box<Parser<'a>>,
    pub positive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LookaheadData {
    pub has_omitted_partial_lookaheads: bool,
}

impl Default for LookaheadData {
    fn default() -> Self {
        // LookaheadData { partial_lookaheads: vec![PartialLookahead { parser: Box::new(Parser::FailParser(FailParser)), positive: true }] }
        LookaheadData { has_omitted_partial_lookaheads: false }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lookahead<'a> {
    pub combinator: Box<Combinator<'a>>,
    pub positive: bool,
    pub persist_with_partial_lookahead: bool,
}

impl CombinatorTrait<'_> for Lookahead<'_> {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, mut parse_results) = self.combinator.parse(right_data.clone(), bytes);
        let has_right_data = !parse_results.right_data_vec.is_empty();
        let succeeds = if self.positive {
            // A positive lookahead succeeds if it yields right data now or it *could* yield right data later (i.e. it's not done yet)
            has_right_data || !parse_results.done()
        } else {
            // A negative lookahead succeeds if it yields no right data now
            !has_right_data
        };
        if succeeds {
            if !parse_results.done() {
                    Rc::make_mut(&mut right_data.right_data_inner).lookahead_data.has_omitted_partial_lookaheads = true;
            }
            (Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
        } else {
            (Parser::FailParser(FailParser), ParseResults::empty_finished())
        }
    }
}

pub fn lookahead<'a>(combinator: impl Into<Combinator<'a>>) -> Lookahead<'a> {
    Lookahead { combinator: Box::new(combinator.into()), positive: true, persist_with_partial_lookahead: false }
}

pub fn negative_lookahead<'a>(combinator: impl Into<Combinator<'a>>) -> Lookahead<'a> {
    Lookahead { combinator: Box::new(combinator.into()), positive: false, persist_with_partial_lookahead: false }
}

impl From<Lookahead<'_>> for Combinator<'_> {
    fn from(lookahead: Lookahead) -> Self {
        Combinator::Lookahead(lookahead)
    }
}