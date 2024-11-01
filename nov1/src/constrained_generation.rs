use crate::glr::parser::GLRParserState;
use crate::precompute::{StateID, Token};
use std::collections::BTreeMap;

struct GrammarConstraintState<'a> {
    precomputed: BTreeMap<StateID, BTreeMap<Vec<Token>, BTreeMap<&'a [u8], usize>>>,
    parser_state: GLRParserState<'a>,
}

impl GrammarConstraintState<'_> {
    fn new() -> Self {
        todo!()
    }

    fn get_mask(&self) -> Vec<bool> {
        todo!()
    }
}