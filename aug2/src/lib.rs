// src/combinator.rs
use std::collections::HashMap;
use std::any::Any;
use std::fmt::Display;
use std::ops::AddAssign;
use std::rc::Rc;
use std::cell::RefCell;

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

// Removed Parser enum
type RightData = ();
type ParseResults = ();
type UnambiguousParseResults = ();
type U8Set = ();

pub trait CombinatorTrait: BaseCombinatorTrait + std::fmt::Debug {
    type Parser: ParserTrait;
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults);
}

pub trait BaseCombinatorTrait {
    fn as_any(&self) -> &dyn std::any::Any;
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {}
}

pub trait ParserTrait: std::fmt::Debug {
    fn parse(&mut self, bytes: &[u8]) -> ParseResults;
}

impl<T: CombinatorTrait + ?Sized> CombinatorTrait for Box<T> {
    type Parser = T::Parser;

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        (**self).parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait + ?Sized> BaseCombinatorTrait for Box<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        (**self).as_any()
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        (**self).apply_to_children(f);
    }
}
