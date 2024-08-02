use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartialLookahead {
    pub parser: Box<Parser>,
    pub positive: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct LookaheadData {
    pub partial_lookaheads: Vec<PartialLookahead>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lookahead {
    pub combinator: Box<Combinator>,
    pub positive: bool,
}

impl CombinatorTrait for Lookahead {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        self.parser_with_steps(right_data, &[])
    }

    fn parser_with_steps(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (mut parser, mut parse_results) = self.combinator.parser_with_steps(right_data.clone(), bytes);
        let has_right_data = !parse_results.right_data_vec.is_empty();
        if has_right_data && self.positive || !has_right_data && !self.positive {
            (parser, ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
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