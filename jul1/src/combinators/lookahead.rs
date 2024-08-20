use crate::*;

#[derive(Debug)]
pub struct PartialLookahead<'a> {
    pub parser: Box<Parser<'a>>,
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

#[derive(Debug)]
pub struct Lookahead<T: CombinatorTrait> {
    pub combinator: Box<T>,
    pub positive: bool,
    pub persist_with_partial_lookahead: bool,
}

impl<T: CombinatorTrait + 'static> CombinatorTrait for Lookahead<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(&self.combinator);
    }
    fn one_shot_parse(&self, mut right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        match self.combinator.one_shot_parse(right_data.clone(), bytes) {
            Ok(_) | Err(UnambiguousParseError::Ambiguous | UnambiguousParseError::Incomplete) => Ok(right_data),
            Err(UnambiguousParseError::Fail) => Err(UnambiguousParseError::Fail),
        }
    }
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

pub fn lookahead(combinator: impl CombinatorTrait + 'static) -> impl CombinatorTrait {
    Lookahead { combinator: Box::new(Box::new(combinator)), positive: true, persist_with_partial_lookahead: false }
}

pub fn negative_lookahead(combinator: impl CombinatorTrait + 'static) -> impl CombinatorTrait {
    Lookahead { combinator: Box::new(Box::new(combinator)), positive: false, persist_with_partial_lookahead: false }
}

// impl From<Lookahead> for Combinator {
//     fn from(lookahead: Lookahead) -> Self {
//         Combinator::Lookahead(lookahead)
//     }
// }
