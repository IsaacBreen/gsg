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

    fn parser_with_steps(&self, mut right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        // Perform lookahead on the available bytes. Append the rest of the bytes as a partial lookahead.
        let n = std::cmp::min(bytes.len(), self.lookahead.len());
        let byte_slice = &bytes[..n];
        let lookahead_slice = &self.lookahead[..n];
        if byte_slice == lookahead_slice && self.positive || byte_slice != lookahead_slice && !self.positive {
            let lookahead_rest = bytes[n..].to_vec();
            if !lookahead_rest.is_empty() {
                let partial_lookahead = PartialLookahead {
                    position: right_data.position,
                    lookahead: lookahead_rest,
                    positive: self.positive,
                };
                right_data.lookahead_data.partial_lookaheads.push(partial_lookahead);
            }
            (Parser::FailParser(FailParser), ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
                done: false,
            })
        } else {
            (Parser::FailParser(FailParser), ParseResults {
                right_data_vec: vec![],
                up_data_vec: vec![],
                done: true,
            })
        }
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