use crate::*;

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
        LookaheadData { partial_lookaheads: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lookahead {
    pub combinator: Box<Combinator>,
    pub positive: bool,
}

impl CombinatorTrait for Lookahead {
    fn parser_with_steps(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (parser, mut parse_results) = self.combinator.parser_with_steps(right_data.clone(), bytes);
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

impl ParserTrait for Lookahead {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        panic!("Lookahead parser should not be stepped")
    }

    fn next_u8set(&self, bytes: &[u8]) -> U8Set {
        panic!("Lookahead parser should not be asked for next_u8set")
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
}```

jul1/src/combinators/repeat1.rs
