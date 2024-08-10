use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::fast_combinator::FastParserResult;
use crate::U8Set;
use crate::fast_combinator::FastParser;

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

impl FastParser {
    pub fn to_dfa(&self) -> DFA {
        let mut states = Vec::new();
        let mut state_map = HashMap::new();
        let start_state = self.to_dfa_helper(&mut states, &mut state_map);
        DFA::new(start_state, states)
    }

    fn to_dfa_helper(&self, states: &mut Vec<DFAState>, state_map: &mut HashMap<FastParser, usize>) -> usize {
        if let Some(&state_index) = state_map.get(self) {
            return state_index;
        }

        let state_index = states.len();
        states.push(DFAState {
            is_accepting: false,
            transitions: HashMap::new(),
        });
        state_map.insert(self.clone(), state_index);

        match self {
            FastParser::Seq(children) => {
                let mut current_state = state_index;
                for child in children {
                    let next_state = child.to_dfa_helper(states, state_map);
                    states[current_state].transitions.insert(0, next_state); // Use 0 as a dummy byte for seq transitions
                    current_state = next_state;
                }
                states[current_state].is_accepting = true;
            }
            FastParser::Choice(children) => {
                for child in children {
                    let next_state = child.to_dfa_helper(states, state_map);
                    states[state_index].transitions.insert(0, next_state); // Use 0 as a dummy byte for choice transitions
                }
            }
            FastParser::Opt(parser) => {
                let next_state = parser.to_dfa_helper(states, state_map);
                states[state_index].transitions.insert(0, next_state); // Use 0 as a dummy byte for opt transitions
                states[state_index].is_accepting = true;
            }
            FastParser::Repeat1(parser) => {
                let next_state = parser.to_dfa_helper(states, state_map);
                states[state_index].transitions.insert(0, next_state); // Use 0 as a dummy byte for repeat1 transitions
                states[next_state].transitions.insert(0, state_index); // Loop back to the beginning for repeat1
                states[next_state].is_accepting = true;
            }
            FastParser::Eps => {
                states[state_index].is_accepting = true;
            }
            FastParser::EatU8Parser(u8set) => {
                for byte in u8set.iter() {
                    let next_state = states.len();
                    states.push(DFAState {
                        is_accepting: true,
                        transitions: HashMap::new(),
                    });
                    states[state_index].transitions.insert(byte, next_state);
                }
            }
        }

        state_index
    }
}