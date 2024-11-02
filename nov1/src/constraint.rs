use crate::glr::parser::{GLRParser, ParseState};
use crate::precompute;
use crate::precompute::{Token, Tokenizer};
use std::collections::{BTreeMap, HashMap, HashSet};
use crate::glr::table;

type LLMToken = &'static [u8];

struct GrammarConstraintState<T: Tokenizer> {
    tokenizer: T,
    parser: GLRParser,
    precomputed: BTreeMap<precompute::StateID, BTreeMap<Vec<Token>, BTreeMap<LLMToken, precompute::StateID>>>,
    states: Vec<(ParseState, HashSet<precompute::StateID>)>,
}

impl<T: Tokenizer> GrammarConstraintState<T> {
    fn new() -> Self {
        todo!()
    }
    fn get_mask(&self) -> HashSet<LLMToken> {
        let mut result = HashSet::new();
        for (parse_state, tokenizer_state_ids) in &self.states {
            for tokenizer_state_id in tokenizer_state_ids {
                for (grammar_token_sequence, llm_token_to_state_id) in &self.precomputed[&tokenizer_state_id] {
                    let mut new_glr_parse_state = self.parser.init_parser_from_parse_state(parse_state.clone());
                    let grammar_token_id_sequence = grammar_token_sequence.iter().map(|t| table::TerminalID(t.id)).collect::<Vec<_>>();
                    new_glr_parse_state.partial_parse(&grammar_token_id_sequence);
                    if new_glr_parse_state.is_ok() {
                        let mut any_next_tokens_are_valid = false;
                        for possible_next_grammar_token in self.tokenizer.tokens_accessible_from_state(*tokenizer_state_id) {
                            let mut new_new_glr_parse_state = new_glr_parse_state.clone();
                            let possible_next_grammar_token_id = table::TerminalID(possible_next_grammar_token);
                            new_new_glr_parse_state.partial_parse(&[possible_next_grammar_token_id]);
                            if new_new_glr_parse_state.is_ok() {
                                any_next_tokens_are_valid = true;
                                break;
                            }
                        }
                        if any_next_tokens_are_valid {
                            for &llm_token in llm_token_to_state_id.keys() {
                                result.insert(llm_token);
                            }
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
                        let mut new_glr_parse_state = self.parser.init_parser_from_parse_state(parse_state.clone());
                        let grammar_token_id_sequence = grammar_token_sequence.iter().map(|t| table::TerminalID(t.id)).collect::<Vec<_>>();
                        new_glr_parse_state.partial_parse(&grammar_token_id_sequence);
                        for active_parse_state in new_glr_parse_state.active_states {
                            new_states.push((active_parse_state, HashSet::from([next_tokenizer_state_id])));
                        }
                    }
                }
            }
        }
        self.states = new_states;
    }

    fn commit_many(&mut self, llm_tokens: &[LLMToken]) {
        for llm_token in llm_tokens {
            self.commit(llm_token);
        }
    }
}