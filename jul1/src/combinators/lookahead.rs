use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LookaheadContext {
    pub inner: Box<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LookaheadContextParser {
    pub inner: Box<Parser>,
}

impl CombinatorTrait for LookaheadContext {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (inner, parse_results) = self.inner.parse(right_data, bytes);
        (Parser::LookaheadContextParser(LookaheadContextParser { inner: Box::new(inner) }), parse_results)
    }
}

impl ParserTrait for LookaheadContextParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        // Count number of partial lookaheads and number of right data
        let mut n_partial_lookaheads = 0;
        let mut n_right_data = 0;
        self.inner.map_right_data_mut(&mut |right_data| { n_partial_lookaheads += right_data.lookahead_data.partial_lookaheads.len(); n_right_data += 1; });
        // Prune partial lookaheads that are done
        self.inner.map_right_data_mut(&mut |right_data| {
            let n = right_data.lookahead_data.partial_lookaheads.len();
            right_data.lookahead_data.partial_lookaheads.retain_mut(|partial_lookahead| {
                !partial_lookahead.parser.parse(bytes).done
            });
            if n != right_data.lookahead_data.partial_lookaheads.len() {
                // println!("pruned patial lookaheads from {} to {}", n, right_data.lookahead_data.partial_lookaheads.len());
            }
        });
        // Count number of partial lookaheads again
        let mut m_partial_lookaheads = 0;
        let mut m_right_data = 0;
        self.inner.map_right_data_mut(&mut |right_data| { m_partial_lookaheads += right_data.lookahead_data.partial_lookaheads.len(); m_right_data += 1; });
        println!("lookahead_context: n_partial_lookaheads = {}, n_right_data = {}, m_partial_lookaheads = {}, m_right_data = {}", n_partial_lookaheads, n_right_data, m_partial_lookaheads, m_right_data);
        self.inner.parse(bytes)
    }
}

pub fn lookahead_context(inner: impl Into<Combinator>) -> LookaheadContext {
    LookaheadContext { inner: Box::new(inner.into()) }
}

impl From<LookaheadContext> for Combinator {
    fn from(lookahead_context: LookaheadContext) -> Self {
        Self::LookaheadContext(lookahead_context)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartialLookahead {
    pub parser: Box<Parser>,
    pub positive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LookaheadData {
    pub partial_lookaheads: Vec<PartialLookahead>,
}

impl Default for LookaheadData {
    fn default() -> Self {
        // LookaheadData { partial_lookaheads: vec![PartialLookahead { parser: Box::new(Parser::FailParser(FailParser)), positive: true }] }
        LookaheadData { partial_lookaheads: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lookahead {
    pub combinator: Box<Combinator>,
    pub positive: bool,
}

impl CombinatorTrait for Lookahead {
    fn parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, mut parse_results) = self.combinator.parse(right_data.clone(), bytes);
        let has_right_data = !parse_results.right_data_vec.is_empty();
        let succeeds = if self.positive {
            // A positive lookahead succeeds if it yields right data now or it *could* yield right data later (i.e. it's not done yet)
            has_right_data || !parse_results.done
        } else {
            // A negative lookahead succeeds if it yields no right data now
            !has_right_data
        };
        if succeeds {
            if !parse_results.done {
                // println!("Lookahead not done at position {}. Lookahead: {:?}", right_data.position, self);
                right_data.lookahead_data.partial_lookaheads.push(PartialLookahead {
                    parser: Box::new(parser),
                    positive: self.positive,
                });
            }
            (Parser::FailParser(FailParser), ParseResults {
                right_data_vec: vec![right_data],
                done: true,
            })
        } else {
            (Parser::FailParser(FailParser), ParseResults::empty_finished())
        }
    }
}

pub fn lookahead(combinator: impl Into<Combinator>) -> Lookahead {
    Lookahead { combinator: Box::new(combinator.into()), positive: true }
}

pub fn negative_lookahead(combinator: impl Into<Combinator>) -> Lookahead {
    Lookahead { combinator: Box::new(combinator.into()), positive: false }
}

impl From<Lookahead> for Combinator {
    fn from(lookahead: Lookahead) -> Self {
        Combinator::Lookahead(lookahead)
    }
}