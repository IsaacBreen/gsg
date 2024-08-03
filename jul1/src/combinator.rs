use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;
use crate::{CacheContext, CacheContextParser, Cached, CachedParser, CacheFirst, CacheFirstContext, CacheFirstContextParser, CacheFirstParser, CheckRightData, CheckRightDataParser, Choice, ChoiceParser, Deferred, EatByteStringChoice, EatByteStringChoiceParser, EatString, EatStringParser, EatU8, EatU8Parser, Eps, EpsParser, Fail, FailParser, ForbidFollows, ForbidFollowsCheckNot, ForbidFollowsClear, ForwardRef, IndentCombinator, IndentCombinatorParser, Lookahead, MutateRightData, MutateRightDataParser, ExcludeBytestrings, ExcludeBytestringsParser, ParseResults, Repeat1, Repeat1Parser, RightData, Seq, SeqParser, Symbol, SymbolParser, Tagged, TaggedParser, U8Set};
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
    MutateRightData,
    Repeat1,
    Symbol,
    Tagged,
    ForwardRef,
    ForbidFollows,
    ForbidFollowsClear,
    ForbidFollowsCheckNot,
    EatByteStringChoice,
    CheckRightData,
    Deferred,
    Lookahead,
    ExcludeBytestrings
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
    MutateRightDataParser,
    Repeat1Parser,
    SymbolParser,
    TaggedParser,
    EatByteStringChoiceParser,
    CheckRightDataParser,
    ExcludeBytestringsParser
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
            MutateRightData,
            Repeat1,
            Symbol,
            Tagged,
            ForwardRef,
            ForbidFollows,
            ForbidFollowsClear,
            ForbidFollowsCheckNot,
            EatByteStringChoice,
            CheckRightData,
            Deferred,
            Lookahead,
            ExcludeBytestrings
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
            EatByteStringChoiceParser,
            EpsParser,
            FailParser,
            CacheContextParser,
            CachedParser,
            CacheFirstParser,
            CacheFirstContextParser,
            IndentCombinatorParser,
            MutateRightDataParser,
            Repeat1Parser,
            SymbolParser,
            TaggedParser,
            CheckRightDataParser,
            ExcludeBytestringsParser
        )
    };
}

pub trait CombinatorTrait {
    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults);
}

pub trait ParserTrait {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults;
    fn next_u8set(&self, bytes: &[u8]) -> U8Set;
}

impl CombinatorTrait for Combinator {
    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        match_combinator!(self, inner => inner.parser_with_steps(right_data, bytes))
    }
}

impl ParserTrait for Parser {
    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        match_parser!(self, inner => inner.steps(bytes))
    }

    fn next_u8set(&self, bytes: &[u8]) -> U8Set {
        match_parser!(self, inner => inner.next_u8set(bytes))
    }
}

impl Combinator {
    pub fn type_name(&self) -> String {
        match_combinator!(self, inner => std::any::type_name_of_val(&inner)).to_string()
    }
}

impl Parser {
    pub fn map_right_data_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut RightData),
    {
        match self {
            Parser::SeqParser(SeqParser { children, .. }) => {
                for (_, parsers) in children {
                    for p in parsers {
                        p.map_right_data_mut(&mut f);
                    }
                }
            }
            Parser::ChoiceParser(ChoiceParser { parsers, .. }) => {
                for p in parsers {
                    p.map_right_data_mut(&mut f);
                }
            }
            Parser::EatU8Parser(EatU8Parser { right_data: Some(right_data), .. }) |
            Parser::EatStringParser(EatStringParser { right_data: Some(right_data), .. }) |
            Parser::EatByteStringChoiceParser(EatByteStringChoiceParser { right_data, .. }) => {
                f(right_data);
            }
            Parser::EpsParser(EpsParser {}) |
            Parser::FailParser(FailParser {}) => {}
            Parser::CacheContextParser(CacheContextParser { inner, cache_data_inner }) => {
                inner.map_right_data_mut(&mut f);
                for entry in cache_data_inner.borrow().entries.iter() {
                    let mut entry = entry.borrow_mut();
                    if let Some(parser) = entry.parser.as_mut() {
                        parser.map_right_data_mut(&mut f);
                    }
                }
            }
            Parser::CachedParser(CachedParser { entry }) => {}
            Parser::CacheFirstContextParser(CacheFirstContextParser { inner, cache_first_data_inner }) => {
                inner.map_right_data_mut(&mut f);
                for (_, parse_results) in cache_first_data_inner.borrow_mut().entries.iter_mut() {
                    for right_data in &mut parse_results.right_data_vec {
                        f(right_data);
                    }
                }
            }
            Parser::SymbolParser(SymbolParser { inner, .. }) |
            Parser::TaggedParser(TaggedParser { inner, .. }) => {
                inner.map_right_data_mut(&mut f);
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::DentParser(parser)) => {
                parser.map_right_data_mut(&mut f);
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::IndentParser(Some(right_data))) => {
                f(right_data);
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::IndentParser(None)) |
            Parser::IndentCombinatorParser(IndentCombinatorParser::Done) => {}
            Parser::CacheFirstParser(CacheFirstParser::Uninitialized { key }) => {}
            Parser::CacheFirstParser(CacheFirstParser::Initialized { parser }) => {
                parser.map_right_data_mut(&mut f);
            }
            Parser::ExcludeBytestringsParser(ExcludeBytestringsParser { inner, .. }) => {
                inner.map_right_data_mut(&mut f);
            }
            Parser::MutateRightDataParser(MutateRightDataParser { run }) => {}
            Parser::Repeat1Parser(Repeat1Parser { a_parsers, .. }) => {
                for a_parser in a_parsers {
                    a_parser.map_right_data_mut(&mut f);
                }
            }
            Parser::CheckRightDataParser(CheckRightDataParser { run }) => {}
            Parser::EatU8Parser(EatU8Parser { right_data: None, .. }) |
            Parser::EatStringParser(EatStringParser { .. }) => {}
        }
    }
}

pub trait CombinatorTraitExt: CombinatorTrait {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        self.parser_with_steps(right_data, &[])
    }
}

pub trait ParserTraitExt: ParserTrait {
    fn step(&mut self, c: u8) -> ParseResults {
        self.steps(&[c])
    }
}

impl<T: CombinatorTrait> CombinatorTraitExt for T {}
impl<T: ParserTrait> ParserTraitExt for T {}```

jul1/src/combinators/symbol.rs
