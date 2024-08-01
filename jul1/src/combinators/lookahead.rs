use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartialLookahead {
    pub position: usize,
    pub lookahead: Vec<u8>,
    pub positive: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct LookaheadData {
    pub partial_lookaheads: Vec<PartialLookahead>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lookahead {
    pub lookahead: Vec<u8>,
    pub positive: bool,
}

impl CombinatorTrait for Lookahead {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        right_data.lookahead_data.partial_lookaheads.push(PartialLookahead {
            position: right_data.position,
            lookahead: self.lookahead.clone(),
            positive: self.positive,
        });
        (Parser::FailParser(FailParser), ParseResults {
            right_data_vec: vec![right_data],
            up_data_vec: vec![],
            done: false,
        })
    }
}

pub fn lookahead(lookahead: Vec<u8>) -> Lookahead {
    Lookahead {
        lookahead,
        positive: true,
    }
}

pub fn negative_lookahead(lookahead: Vec<u8>) -> Lookahead {
    Lookahead {
        lookahead,
        positive: false,
    }
}

impl From<Lookahead> for Combinator {
    fn from(lookahead: Lookahead) -> Self {
        Combinator::Lookahead(lookahead)
    }
}