use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialLookahead {
    pub parser: Box<Parser>,
    pub positive: bool,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LookaheadData {
    pub has_omitted_partial_lookaheads: bool,
}

impl Default for LookaheadData {
    fn default() -> Self {
        // LookaheadData { partial_lookaheads: vec![PartialLookahead { parser: Box::new(Parser::FailParser(FailParser)), positive: true }] }
        LookaheadData { has_omitted_partial_lookaheads: false }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookahead {
    pub combinator: Box<Combinator>,
    pub positive: bool,
    pub persist_with_partial_lookahead: bool,
}

impl CombinatorTrait for Lookahead {
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
                    Rc::make_mut(&mut right_data.right_data_inner).fields1.lookahead_data.has_omitted_partial_lookaheads = true;
            }
            (Parser::FailParser(FailParser), ParseResults::new_single(right_data, true))
        } else {
            (Parser::FailParser(FailParser), ParseResults::empty_finished())
        }
    }
}

pub fn lookahead(combinator: impl Into<Combinator>) -> Lookahead {
    Lookahead { combinator: Box::new(combinator.into()), positive: true, persist_with_partial_lookahead: false }
}

pub fn negative_lookahead(combinator: impl Into<Combinator>) -> Lookahead {
    Lookahead { combinator: Box::new(combinator.into()), positive: false, persist_with_partial_lookahead: false }
}

impl From<Lookahead> for Combinator {
    fn from(lookahead: Lookahead) -> Self {
        Combinator::Lookahead(lookahead)
    }
}