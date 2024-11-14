use crate::glr::parser::{GLRParser, InsertWith, ParseState, ParseStateKey};
use crate::glr::table;
use crate::glr::table::StateID;
use crate::precompute;
use crate::precompute::{Token, TokenID, Tokenizer};
use std::collections::{BTreeMap, BTreeSet};

type LLMToken = &'static [u8];

pub struct GrammarConstraintState<T: Tokenizer> {
    pub(crate) tokenizer: T,
    pub(crate) parser: GLRParser,
    pub(crate) precomputed: BTreeMap<StateID, BTreeMap<Vec<TokenID>, BTreeMap<LLMToken, StateID>>>,
    pub(crate) states: Vec<(ParseState, BTreeSet<StateID>)>,
}

impl<T: Tokenizer> GrammarConstraintState<T> {
    pub fn new(tokenizer: T, parser: GLRParser, llm_tokens: &[LLMToken]) -> Self {
        let precomputed = precompute::precompute(&tokenizer, llm_tokens);
        let precomputed = precompute::precompute_add_incomplete_token(&tokenizer, precomputed);
        let states = vec![(parser.init_parse_state(), BTreeSet::from([StateID(tokenizer.initial_state_id())]))];
        Self {
            tokenizer,
            parser,
            precomputed,
            states,
        }
    }

    pub fn get_mask(&self) -> BTreeSet<LLMToken> {
        let mut result = BTreeSet::new();
        for (parse_state, tokenizer_state_ids) in &self.states {
            for tokenizer_state in tokenizer_state_ids {
                for (grammar_token_sequence, llm_token_to_state_id) in &self.precomputed[&tokenizer_state] {
                    let mut new_glr_parse_state = self.parser.init_glr_parser_from_parse_state(parse_state.clone());
                    let grammar_token_id_sequence = grammar_token_sequence.iter().map(|t| table::TerminalID(*t)).collect::<Vec<_>>();
                    new_glr_parse_state.parse_part(&grammar_token_id_sequence);
                    if new_glr_parse_state.is_ok() {
                        // todo: fix this. this is wrong.
                        let mut any_next_tokens_are_valid = false;
                        for possible_next_grammar_token in self.tokenizer.tokens_accessible_from_state(tokenizer_state.0) {
                            let mut new_new_glr_parse_state = new_glr_parse_state.clone();
                            let possible_next_grammar_token_id = table::TerminalID(possible_next_grammar_token);
                            new_new_glr_parse_state.parse_part(&[possible_next_grammar_token_id]);
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

    pub fn commit(&mut self, llm_token: LLMToken) {
        let mut new_states: BTreeMap<(ParseStateKey, BTreeSet<StateID>), ParseState> = BTreeMap::new();
        for (parse_state, tokenizer_state_ids) in &self.states {
            for tokenizer_state_id in tokenizer_state_ids {
                for (grammar_token_sequence, llm_token_to_state_id) in &self.precomputed[&tokenizer_state_id] {
                    if let Some(&next_tokenizer_state_id) = llm_token_to_state_id.get(llm_token) {
                        let mut new_glr_parse_state = self.parser.init_glr_parser_from_parse_state(parse_state.clone());
                        let grammar_token_id_sequence = grammar_token_sequence.iter().map(|t| table::TerminalID(*t)).collect::<Vec<_>>();
                        new_glr_parse_state.parse_part(&grammar_token_id_sequence);
                        for active_parse_state in new_glr_parse_state.active_states {
                            new_states.insert_with(
                                (active_parse_state.key(), BTreeSet::from([next_tokenizer_state_id])),
                                active_parse_state,
                                |old, new| {
                                    old.merge(new);
                                }
                            );
                        }
                    }
                }
            }
        }
        self.states = new_states.into_iter().map(|((key, tokenizer_state_ids), parse_state)| {
            (parse_state, tokenizer_state_ids)
        }).collect();
    }

    pub fn commit_many(&mut self, llm_tokens: &[LLMToken]) {
        for llm_token in llm_tokens {
            self.commit(llm_token);
        }
    }
}