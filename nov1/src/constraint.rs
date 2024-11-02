use crate::glr::parser::ParseState;
use crate::precompute;
use crate::precompute::{Token};
use std::collections::{BTreeMap, HashMap, HashSet};

type LLMToken = &'static [u8];

struct GrammarConstraintState {
    precomputed: BTreeMap<precompute::StateID, BTreeMap<Vec<Token>, BTreeMap<LLMToken, precompute::StateID>>>,
    states: Vec<(ParseState, HashSet<precompute::StateID>)>,
}

impl GrammarConstraintState {
    fn new() -> Self {
        todo!()
    }
    fn get_mask(&self) -> HashSet<LLMToken> {
        let mut result = HashSet::new();
        for (parse_state, tokenizer_state_ids) in &self.states {
            for tokenizer_state_id in tokenizer_state_ids {
                for (grammar_token_sequence, llm_token_to_state_id) in &self.precomputed[&tokenizer_state_id] {
                    let new_parse_state = parse_state.clone();
                    new_parse_state.parse_part(grammar_token_sequence);
                    if new_parse_state.is_ok() {
                        for &llm_token in llm_token_to_state_id.keys() {
                            result.insert(llm_token);
                        }
                    }
                }
            }
        }
        result
    }

    fn commit(&mut self, llm_token: LLMToken) {
        let mut new_states = Vec::new();
        for (parse_state, tokenizer_state_ids) in &self.states {
            for tokenizer_state_id in tokenizer_state_ids {
                for (grammar_token_sequence, llm_token_to_state_id) in &self.precomputed[&tokenizer_state_id] {
                    if let Some(&next_tokenizer_state_id) = llm_token_to_state_id.get(llm_token) {
                        let new_parse_state = parse_state.clone();
                        new_parse_state.parse_part(grammar_token_sequence);
                        if new_parse_state.is_ok() {
                            new_states.push((new_parse_state, HashSet::from([next_tokenizer_state_id])));
                        }
                    }
                }
            }
        }
        self.states = new_states;
    }
}