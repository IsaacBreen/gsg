use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NegativeLookahead {
    pub(crate) inner: Box<Combinator>,
    pub(crate) lookahead: Box<Combinator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NegativeLookaheadParser {
    pub(crate) inner: Box<Parser>,
    pub(crate) lookahead: Box<Parser>,
}

impl CombinatorTrait for NegativeLookahead {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let (inner, mut inner_results) = self.inner.parser(right_data.clone());
        let (lookahead, lookahead_results) = self.lookahead.parser(right_data);
        let mut exclusion_filter = U8Set::none();
        for up_data in lookahead_results.up_data_vec.iter() {
            exclusion_filter |= up_data.u8set;
        }
        for up_data in inner_results.up_data_vec.iter_mut() {
            up_data.u8set &= exclusion_filter;
        }
        if !inner_results.right_data_vec.is_empty() && !lookahead_results.right_data_vec.is_empty() {
            inner_results.right_data_vec.clear();
        }
        (Parser::NegativeLookaheadParser(NegativeLookaheadParser {
            inner: Box::new(inner),
            lookahead: Box::new(lookahead),
        }), inner_results)
    }

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (inner, mut inner_results) = self.inner.parser_with_steps(right_data.clone(), bytes);
        let (lookahead, lookahead_results) = self.lookahead.parser_with_steps(right_data, bytes);
        let mut exclusion_filter = U8Set::none();
        for up_data in lookahead_results.up_data_vec.iter() {
            exclusion_filter |= up_data.u8set;
        }
        for up_data in inner_results.up_data_vec.iter_mut() {
            up_data.u8set &= exclusion_filter;
        }
        if !inner_results.right_data_vec.is_empty() && !lookahead_results.right_data_vec.is_empty() {
            inner_results.right_data_vec.clear();
        }
        (Parser::NegativeLookaheadParser(NegativeLookaheadParser {
            inner: Box::new(inner),
            lookahead: Box::new(lookahead),
        }), inner_results)
    }
}

impl ParserTrait for NegativeLookaheadParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut inner_results = self.inner.step(c);
        let lookahead_results = self.lookahead.step(c);
        let mut exclusion_filter = U8Set::none();
        for up_data in lookahead_results.up_data_vec.iter() {
            exclusion_filter |= up_data.u8set;
        }
        for up_data in inner_results.up_data_vec.iter_mut() {
            up_data.u8set &= exclusion_filter;
        }
        if !inner_results.right_data_vec.is_empty() && !lookahead_results.right_data_vec.is_empty() {
            inner_results.right_data_vec.clear();
        }
        inner_results
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        let mut inner_results = self.inner.steps(bytes);
        let lookahead_results = self.lookahead.steps(bytes);
        let mut exclusion_filter = U8Set::none();
        for up_data in lookahead_results.up_data_vec.iter() {
            exclusion_filter |= up_data.u8set;
        }
        for up_data in inner_results.up_data_vec.iter_mut() {
            up_data.u8set &= exclusion_filter;
        }
        if !inner_results.right_data_vec.is_empty() && !lookahead_results.right_data_vec.is_empty() {
            inner_results.right_data_vec.clear();
        }
        inner_results
    }
}

pub fn negative_lookahead_wrapper(inner: Combinator, lookahead: Combinator) -> Combinator {
    Combinator::NegativeLookahead(NegativeLookahead {
        inner: Box::new(inner),
        lookahead: Box::new(lookahead),
    })
}

impl From<NegativeLookahead> for Combinator {
    fn from(negative_lookahead: NegativeLookahead) -> Self {
        Self::NegativeLookahead(negative_lookahead)
    }
}