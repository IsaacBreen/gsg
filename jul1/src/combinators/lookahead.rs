use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartialLookahead {
    pub parser: Box<Parser>,
    pub positive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookaheadData {
    pub lookahead_data_inner: Rc<RefCell<LookaheadDataInner>>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LookaheadDataInner {
    pub partial_lookaheads: Vec<PartialLookahead>
}

impl Default for LookaheadData {
    fn default() -> Self {
        LookaheadData { lookahead_data_inner: Rc::new(RefCell::new(LookaheadDataInner { partial_lookaheads: vec![] })) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LookaheadContext {
    pub lookahead_data_inner: Option<Rc<RefCell<LookaheadDataInner>>>,
    pub inner: Option<Box<Combinator>>
}

impl Hash for LookaheadContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.as_ref().unwrap().hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LookaheadContextParser {
    pub lookahead_data_inner: Option<Rc<RefCell<LookaheadDataInner>>>,
    pub inner: Option<Box<Parser>>
}

impl Hash for LookaheadContextParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.as_ref().unwrap().hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lookahead {
    pub combinator: Box<Combinator>,
    pub positive: bool,
}

impl LookaheadData {
    pub fn is_empty(&self) -> bool {
        self.lookahead_data_inner.borrow().partial_lookaheads.is_empty()
    }
}

impl LookaheadContextParser {
    fn cleanup(&mut self) {
        self.lookahead_data_inner = None;
        self.inner = None;
    }
}

impl CombinatorTrait for LookaheadContext {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let (inner_parser, mut parse_results) = self.inner.as_ref().unwrap().parser(right_data.clone());
        let mut parser = LookaheadContextParser {
            lookahead_data_inner: self.lookahead_data_inner.clone(),
            inner: Some(Box::new(inner_parser)),
        };
        parser.cleanup();
        (Parser::LookaheadContextParser(parser), parse_results)
    }

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (inner_parser, mut parse_results) = self.inner.as_ref().unwrap().parser_with_steps(right_data.clone(), bytes);
        let mut parser = LookaheadContextParser {
            lookahead_data_inner: self.lookahead_data_inner.clone(),
            inner: Some(Box::new(inner_parser)),
        };
        parser.cleanup();
        (Parser::LookaheadContextParser(parser), parse_results)
    }
}

impl ParserTrait for LookaheadContextParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let parse_results = self.inner.as_ref().unwrap().step(c);
        self.cleanup();
        parse_results
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        let parse_results = self.inner.as_ref().unwrap().steps(bytes);
        self.cleanup();
        parse_results
    }
}

impl CombinatorTrait for Lookahead {
    fn parser(&self, mut right_data: RightData) -> (Parser, ParseResults) {
        self.parser_with_steps(right_data, &[])
    }

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
                right_data.lookahead_data.lookahead_data_inner.partial_lookaheads.push(PartialLookahead {
                    parser: Box::new(parser),
                    positive: self.positive,
                });
            }
            (Parser::FailParser(FailParser), ParseResults {
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