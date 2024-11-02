use crate::glr::parser::GLRParserState;
use crate::precompute::{StateID, Token};
use std::collections::BTreeMap;
use crate::glr::table::TerminalID;

type LLMToken = usize;

struct GrammarConstraintState<'a> {
    precomputed: BTreeMap<StateID, BTreeMap<Vec<Token>, BTreeMap<&'a [u8], usize>>>,
    // Somehow we need to associate the tokenizer state with each parser state.
    // We could maybe add a generic to the parser state.
    // Or maybe we could 'thin out' GLRParserState and its related methods so we can deal with ParseState here directly.
    // Perhaps option 2 is better.
    parser_states: GLRParserState<'a>,
}

impl GrammarConstraintState<'_> {
    fn new() -> Self {
        todo!()
    }

    fn get_mask(&self) -> Vec<bool> {
        todo!()
    }

    fn step(&mut self, llm_token: LLMToken) {
        todo!()
    }
}