use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::fast_combinator::FastParserResult;
use crate::U8Set;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DFAState {
    pub(crate) is_accepting: bool,
    pub(crate) transitions: HashMap<u8, usize>, // byte -> next state index
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DFA {
    states: Vec<DFAState>,
    start_state: usize,
}

impl Hash for DFA {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start_state.hash(state);
        self.states.len().hash(state);
    }
}

impl DFA {
    pub fn new(start_state: usize, states: Vec<DFAState>) -> Self {
        DFA { states, start_state }
    }

    pub fn parse(&self, bytes: &[u8]) -> FastParserResult {
        let mut current_state = self.start_state;
        let mut bytes_consumed = 0;

        for &byte in bytes {
            if let Some(next_state) = self.states[current_state].transitions.get(&byte) {
                current_state = *next_state;
                bytes_consumed += 1;
            } else {
                return if self.states[current_state].is_accepting {
                    FastParserResult::Success(bytes_consumed)
                } else {
                    FastParserResult::Failure
                };
            }
        }

        if self.states[current_state].is_accepting {
            FastParserResult::Success(bytes_consumed)
        } else {
            FastParserResult::Incomplete
        }
    }
}
