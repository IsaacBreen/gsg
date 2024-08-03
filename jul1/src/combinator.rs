use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;
use crate::{CacheContext, CacheContextParser, Cached, CachedParser, CacheFirst, CacheFirstContext, CacheFirstContextParser, CacheFirstParser, CheckRightData, CheckRightDataParser, Choice, ChoiceParser, Deferred, EatByteStringChoice, EatByteStringChoiceParser, EatString, EatStringParser, EatU8, EatU8Parser, Eps, EpsParser, Fail, FailParser, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, ForwardRef, FrameStackOp, FrameStackOpParser, IndentCombinator, IndentCombinatorParser, Lookahead, MutateRightData, MutateRightDataParser, NegativeLookahead, NegativeLookaheadParser, ParseResults, Repeat1, Repeat1Parser, RightData, Seq, SeqParser, Symbol, SymbolParser, Tagged, TaggedParser, WithNewFrame, WithNewFrameParser};
use crate::stats::Stats;

macro_rules! define_enum {
    ($name:ident, $($variants:ident),*) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $variants($variants),
            )*
        }
    };
}

#[macro_export]
macro_rules! match_enum {
    ($expr:expr, $enum:ident, $inner:ident => $arm:expr, $($variant:ident),*) => {
        match $expr {
            $(
                $enum::$variant($inner) => $arm,
            )*
        }
    };
}

define_enum!(
    Combinator,
    Seq,
    Choice,
    EatU8,
    EatString,
    Eps,
    Fail,
    CacheContext,
    Cached,
    CacheFirstContext,
    CacheFirst,
    IndentCombinator,
    FrameStackOp,
    MutateRightData,
    Repeat1,
    Symbol,
    Tagged,
    ForwardRef,
    WithNewFrame,
    ForbidFollows,
    ForbidFollowsClear,
    ForbidFollowsCheckNot,
    EatByteStringChoice,
    CheckRightData,
    Deferred,
    Lookahead,
    NegativeLookahead
);

define_enum!(
    Parser,
    SeqParser,
    ChoiceParser,
    EatU8Parser,
    EatStringParser,
    EpsParser,
    FailParser,
    CacheContextParser,
    CachedParser,
    CacheFirstParser,
    CacheFirstContextParser,
    IndentCombinatorParser,
    FrameStackOpParser,
    MutateRightDataParser,
    Repeat1Parser,
    SymbolParser,
    TaggedParser,
    WithNewFrameParser,
    EatByteStringChoiceParser,
    CheckRightDataParser,
    NegativeLookaheadParser
);

macro_rules! match_combinator {
    ($expr:expr, $inner:ident => $arm:expr) => {
        $crate::match_enum!($expr, Combinator, $inner => $arm,
            Seq,
            Choice,
            EatU8,
            EatString,
            Eps,
            Fail,
            CacheContext,
            Cached,
            CacheFirstContext,
            CacheFirst,
            IndentCombinator,
            FrameStackOp,
            MutateRightData,
            Repeat1,
            Symbol,
            Tagged,
            ForwardRef,
            WithNewFrame,
            ForbidFollows,
            ForbidFollowsClear,
            ForbidFollowsCheckNot,
            EatByteStringChoice,
            CheckRightData,
            Deferred,
            Lookahead,
            NegativeLookahead
        )
    };
}

#[macro_export]
macro_rules! match_parser {
    ($expr:expr, $inner:ident => $arm:expr) => {
        $crate::match_enum!($expr, Parser, $inner => $arm,
            SeqParser,
            ChoiceParser,
            EatU8Parser,
            EatStringParser,
            EpsParser,
            FailParser,
            CacheContextParser,
            CachedParser,
            CacheFirstParser,
            CacheFirstContextParser,
            IndentCombinatorParser,
            FrameStackOpParser,
            MutateRightDataParser,
            Repeat1Parser,
            SymbolParser,
            TaggedParser,
            WithNewFrameParser,
            EatByteStringChoiceParser,
            CheckRightDataParser,
            NegativeLookaheadParser
        )
    };
}

pub trait CombinatorTrait {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults);
    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults);
    // fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
    //     let (mut parser, mut parse_results0) = self.parser(right_data);
    //     let parse_results1 = parser.steps(bytes);
    //     parse_results0.combine(parse_results1);
    //     (parser, parse_results0)
    // }
}

pub trait ParserTrait {
    fn step(&mut self, c: u8) -> ParseResults;
    fn steps(&mut self, bytes: &[u8]) -> ParseResults;
}

impl CombinatorTrait for Combinator {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        match_combinator!(self, inner => inner.parser(right_data))
    }

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        match_combinator!(self, inner => inner.parser_with_steps(right_data, bytes))
    }
}

impl ParserTrait for Parser {
    fn step(&mut self, c: u8) -> ParseResults {
        match_parser!(self, inner => inner.step(c))
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        match_parser!(self, inner => inner.steps(bytes))
    }
}

impl Combinator {
    pub fn type_name(&self) -> String {
        match_combinator!(self, inner => std::any::type_name_of_val(&inner)).to_string()
    }
}

// Parser::SeqParser(SeqParser { children, position }) => todo!(),
// Parser::ChoiceParser(ChoiceParser { parsers, greedy }) => todo!(),
// Parser::EatU8Parser(EatU8Parser { u8set, right_data }) => todo!(),
// Parser::EatStringParser(EatStringParser { string, index, right_data }) => todo!(),
// Parser::EpsParser(EpsParser {}) => todo!(),
// Parser::FailParser(FailParser {}) => todo!(),
// Parser::CacheContextParser(CacheContextParser { inner, cache_data_inner }) => todo!(),
// Parser::CachedParser(CachedParser { entry }) => todo!(),
// Parser::CacheFirstContextParser(CacheFirstContextParser { inner, cache_first_data_inner }) => todo!(),
// Parser::FrameStackOpParser(FrameStackOpParser { op_type, frame_stack, values, a }) => todo!(),
// Parser::MutateRightDataParser(MutateRightDataParser { run }) => todo!(),
// Parser::Repeat1Parser(Repeat1Parser { a, a_parsers, right_data, position, greedy }) => todo!(),
// Parser::SymbolParser(SymbolParser { inner, symbol_value }) => todo!(),
// Parser::TaggedParser(TaggedParser { inner, tag }) => todo!(),
// Parser::WithNewFrameParser(WithNewFrameParser { a }) => todo!(),
// Parser::EatByteStringChoiceParser(EatByteStringChoiceParser { current_node, right_data }) => todo!(),
// Parser::CheckRightDataParser(CheckRightDataParser { run }) => todo!(),
// Parser::NegativeLookaheadParser(NegativeLookaheadParser { inner, lookahead }) => todo!(),
// Parser::IndentCombinatorParser(inner) => todo!(),
// Parser::CacheFirstParser(inner) => todo!(),

impl Parser {
    pub fn get_right_data_mut(&mut self) -> Vec<&mut RightData> {
        match self {
            Parser::SeqParser(SeqParser { children, .. }) => {
                let mut results = Vec::new();
                for (_, parsers) in children {
                    for p in parsers {
                        results.append(&mut p.get_right_data_mut());
                    }
                }
                results
            }
            Parser::ChoiceParser(ChoiceParser { parsers, .. }) => {
                let mut results = Vec::new();
                for p in parsers {
                    results.append(&mut p.get_right_data_mut());
                }
                results
            }
            Parser::EatU8Parser(EatU8Parser { right_data: Some(right_data), .. }) |
            Parser::EatStringParser(EatStringParser { right_data: Some(right_data), .. }) |
            Parser::EatByteStringChoiceParser(EatByteStringChoiceParser { right_data, .. }) => {
                vec![right_data]
            }
            Parser::EpsParser(EpsParser {}) |
            Parser::FailParser(FailParser {}) => {
                vec![]
            },
            Parser::CacheContextParser(CacheContextParser { inner, cache_data_inner }) => {
                let mut results = inner.get_right_data_mut();
                for entry in cache_data_inner.borrow().entries.iter() {
                    let mut entry = entry.borrow_mut();
                    if let Some(parser) = entry.parser.as_mut() {
                        results.append(&mut parser.get_right_data_mut());
                    }
                }
                results
            }
            Parser::CachedParser(CachedParser { entry }) => {
                vec![]
            }
            Parser::CacheFirstContextParser(CacheFirstContextParser { inner, cache_first_data_inner }) => {
                let mut results = inner.get_right_data_mut();
                for (key, parse_results) in cache_first_data_inner.borrow().entries.iter_mut() {
                    for right_data in &mut parse_results.right_data_vec {
                        results.push(right_data);
                    }
                }
                results
            }
            Parser::FrameStackOpParser(FrameStackOpParser { a: inner, .. }) |
            Parser::SymbolParser(SymbolParser { inner, .. }) |
            Parser::TaggedParser(TaggedParser { inner, .. }) |
            Parser::WithNewFrameParser(WithNewFrameParser { a: Some(inner), .. }) => {
                inner.get_right_data_mut()
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::DentParser(parser)) => {
                parser.get_right_data_mut()
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::IndentParser(Some(right_data))) => {
                vec![right_data]
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::IndentParser(None)) |
            Parser::IndentCombinatorParser(IndentCombinatorParser::Done) => {
                vec![]
            }

            Parser::CacheFirstParser(CacheFirstParser::Uninitialized { key }) => {
                vec![]
            }
            Parser::CacheFirstParser(CacheFirstParser::Initialized { parser }) => {
                parser.get_right_data_mut()
            }
            Parser::NegativeLookaheadParser(NegativeLookaheadParser { inner, lookahead }) => {
                inner.get_right_data_mut()
            }
            Parser::MutateRightDataParser(MutateRightDataParser { run }) => {
                vec![]
            }
            Parser::Repeat1Parser(Repeat1Parser { a, a_parsers, right_data, position, greedy }) => {
                let mut results = vec![right_data];
                for a_parser in a_parsers {
                    results.append(&mut a_parser.get_right_data_mut());
                }
                results
            }
            Parser::CheckRightDataParser(CheckRightDataParser { run }) => {
                vec![]
            }
            Parser::EatU8Parser(EatU8Parser { right_data: None, .. }) |
            Parser::EatStringParser(EatStringParser { string: _, index: _, right_data: None }) |
            Parser::WithNewFrameParser(WithNewFrameParser { a: None }) => {
                vec![]
            }
        }
    }
}