// src/constraint.rs
use crate::glr::parser::{GLRParser, InsertWith, ParseState, ParseStateKey};
use crate::glr::table;
use crate::glr::table::StateID;
use crate::precompute;
use crate::precompute::{Token, TokenID, Tokenizer};
use std::collections::{BTreeMap, BTreeSet};

type LLMToken = &'static [u8];
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LLMTokenID(pub usize);

pub struct GrammarConstraint<T: Tokenizer> {
    pub(crate) tokenizer: T,
    pub(crate) parser: GLRParser,
    pub(crate) precomputed: BTreeMap<StateID, BTreeMap<Vec<TokenID>, BTreeMap<LLMTokenID, StateID>>>,
    pub(crate) llm_token_to_id: BTreeMap<LLMToken, LLMTokenID>,
    pub(crate) llm_token_id_to_token: BTreeMap<LLMTokenID, LLMToken>,
}

pub struct GrammarConstraintState<'a, T: Tokenizer> {
    parent: &'a GrammarConstraint<T>,
    pub(crate) states: Vec<(ParseState, BTreeSet<StateID>)>,
}

pub fn convert_precomputed_to_llm_token_ids<'a>(
    precomputed: BTreeMap<StateID, BTreeMap<Vec<TokenID>, BTreeMap<&'a [u8], StateID>>>,
    llm_token_to_id: &BTreeMap<LLMToken, LLMTokenID>
) -> BTreeMap<StateID, BTreeMap<Vec<TokenID>, BTreeMap<LLMTokenID, StateID>>> {
    let mut result = BTreeMap::new();
    for (state_id, token_sequence_map) in precomputed {
        let mut new_token_sequence_map = BTreeMap::new();
        for (token_sequence, llm_token_state_map) in token_sequence_map {
            let mut new_llm_token_state_map = BTreeMap::new();
            for (llm_token, next_state_id) in llm_token_state_map {
                let llm_token_id = llm_token_to_id.get(llm_token).unwrap();
                new_llm_token_state_map.insert(*llm_token_id, next_state_id);
            }
            new_token_sequence_map.insert(token_sequence, new_llm_token_state_map);
        }
        result.insert(state_id, new_token_sequence_map);
    }
    result
}

impl<T: Tokenizer> GrammarConstraint<T> {
    pub fn new(tokenizer: T, parser: GLRParser, llm_tokens: &[LLMToken]) -> Self {
        let mut llm_token_to_id = BTreeMap::new();
        let mut llm_token_id_to_token = BTreeMap::new();
        for (i, &token) in llm_tokens.iter().enumerate() {
            let id = LLMTokenID(i);
            llm_token_to_id.insert(token, id);
            llm_token_id_to_token.insert(id, token);
        }

        let precomputed = precompute::precompute(&tokenizer, llm_tokens);
        let precomputed = precompute::precompute_add_incomplete_token(&tokenizer, precomputed);
        let precomputed = convert_precomputed_to_llm_token_ids(precomputed, &llm_token_to_id);

        let states = vec![(parser.init_parse_state(), BTreeSet::from([StateID(tokenizer.initial_state_id())]))];
        Self {
            tokenizer,
            parser,
            precomputed,
            llm_token_to_id,
            llm_token_id_to_token,
        }
    }

    pub fn init(&self) -> GrammarConstraintState<T> {
        GrammarConstraintState {
            parent: self,
            states: vec![(self.parser.init_parse_state(), BTreeSet::from([StateID(self.tokenizer.initial_state_id())]))],
        }
    }
}

impl<'a, T: Tokenizer> GrammarConstraintState<'a, T> {
    pub fn get_mask(&self) -> BTreeSet<LLMTokenID> {
        let mut result = BTreeSet::new();
        for (parse_state, tokenizer_state_ids) in &self.states {
            for tokenizer_state in tokenizer_state_ids {
                for (tokenizer_token_sequence, llm_token_to_state_id) in &self.parent.precomputed[&tokenizer_state] {
                    let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_state(parse_state.clone());
                    let grammar_token_id_sequence = tokenizer_token_sequence.iter().map(|t| table::TerminalID(*t)).collect::<Vec<_>>();
                    new_glr_parse_state.parse_part(&grammar_token_id_sequence);
                    if new_glr_parse_state.is_ok() {
                        result.extend(llm_token_to_state_id.keys().cloned());
                    }
                }
            }
        }
        result
    }

    pub fn commit(&mut self, llm_token_id: LLMTokenID) {
        let llm_token = self.parent.llm_token_id_to_token.get(&llm_token_id).unwrap();
        let mut new_states: BTreeMap<(ParseStateKey, BTreeSet<StateID>), ParseState> = BTreeMap::new();
        for (parse_state, tokenizer_state_ids) in &self.states {
            for tokenizer_state_id in tokenizer_state_ids {
                for (grammar_token_sequence, llm_token_id_to_state_id) in &self.parent.precomputed[&tokenizer_state_id] {
                    if let Some(&next_tokenizer_state_id) = llm_token_id_to_state_id.get(&llm_token_id) {
                        let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_state(parse_state.clone());
                        let mut grammar_token_id_sequence = grammar_token_sequence.iter().map(|t| table::TerminalID(*t)).collect::<Vec<_>>();
                        // omit the incomplete tail token
                        grammar_token_id_sequence.pop();

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

    pub fn commit_many(&mut self, llm_token_ids: &[LLMTokenID]) {
        for &llm_token_id in llm_token_ids {
            self.commit(llm_token_id);
        }
    }
}