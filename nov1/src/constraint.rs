// src/constraint.rs
use crate::glr::parser::{GLRParser, InsertWith, ParseState, ParseStateKey};
use crate::glr::table;
use crate::glr::table::StateID;
use crate::precompute;
use crate::precompute::{Token, TokenID, Tokenizer};
use bitvec::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use crate::trie::TrieNode;

type LLMToken = Vec<u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LLMTokenID(pub usize);

// TODO: should this *really* derive `Clone`? Users probably shouldn't clone this, should they?
#[derive(Debug, Clone)]
pub struct GrammarConstraint<T: Tokenizer> {
    pub(crate) tokenizer: T,
    pub(crate) parser: GLRParser,
    pub(crate) precomputed: BTreeMap<StateID, TrieNode<TokenID, (BTreeMap<LLMTokenID, StateID>, BTreeMap<TokenID, BitVec>)>>,
    pub(crate) num_llm_tokens: usize,
}

#[derive(Debug, Clone)]
pub struct GrammarConstraintState<T: Tokenizer> {
    pub(crate) parent: GrammarConstraint<T>,
    pub(crate) states: Vec<(ParseState, BTreeSet<StateID>)>,
}

pub fn convert_precomputed_to_llm_token_ids<'a>(
    tokenizer: &impl Tokenizer,
    precomputed: BTreeMap<StateID, TrieNode<TokenID, BTreeMap<&'a [u8], StateID>>>,
    llm_tokens: &[LLMToken],
// ) -> BTreeMap<StateID, BTreeMap<Vec<TokenID>, (BTreeMap<LLMTokenID, StateID>, BitVec)>> {
) -> BTreeMap<StateID, TrieNode<TokenID, (BTreeMap<LLMTokenID, StateID>, BTreeMap<TokenID, BitVec>)>> {
    let num_llm_tokens = llm_tokens.len();
    let llm_token_to_id: BTreeMap<_, _> = llm_tokens.iter().enumerate().map(|(i, token)| (token.clone(), LLMTokenID(i))).collect();
    let mut result = BTreeMap::new();
    for (state_id, token_sequence_map) in precomputed {
        let mut new_token_sequence_map_arc = token_sequence_map.map_t(|llm_token_to_state_id| {
            let mut new_llm_token_state_map = BTreeMap::new();
            let mut bitsets: BTreeMap<TokenID, BitVec> = BTreeMap::new();
            for (llm_token, next_state_id) in llm_token_to_state_id {
                let llm_token_id = llm_token_to_id.get(llm_token).unwrap();
                new_llm_token_state_map.insert(*llm_token_id, next_state_id);
                for possible_next_token_id in tokenizer.tokens_accessible_from_state(next_state_id.0) {
                    bitsets.entry(possible_next_token_id).or_insert_with(|| {
                        let mut bitset = BitVec::new();
                        bitset.resize(num_llm_tokens, false);
                        bitset
                    }).set(llm_token_id.0, true);
                }
            }
            (new_llm_token_state_map, bitsets)
        });
        let new_token_sequence_map = new_token_sequence_map_arc.lock().unwrap().clone();
        result.insert(state_id, new_token_sequence_map);
    }

    result

}

impl<T: Tokenizer> GrammarConstraint<T> {
    pub fn new(tokenizer: T, parser: GLRParser, llm_tokens: &[LLMToken]) -> Self {
        let precomputed = precompute::precompute(&tokenizer, &llm_tokens.iter().map(|token| &token[..]).collect::<Vec<_>>());
        let precomputed = precompute::precompute_add_incomplete_token(&tokenizer, precomputed);
        let precomputed = convert_precomputed_to_llm_token_ids(&tokenizer, precomputed, llm_tokens);
        let num_llm_tokens = llm_tokens.len();

        Self {
            tokenizer,
            parser,
            precomputed,
            num_llm_tokens,
        }
    }

    pub fn init(self) -> GrammarConstraintState<T> {
        let parser_initial_state = self.parser.init_parse_state();
        let tokenizer_initial_state_id = StateID(self.tokenizer.initial_state_id());
        GrammarConstraintState {
            parent: self,
            states: vec![(parser_initial_state, BTreeSet::from([tokenizer_initial_state_id]))],
        }
    }
}

impl<'a, T: Tokenizer> GrammarConstraintState<T> {
    pub fn get_mask(&self) -> BitVec {
        let mut result = BitVec::new();
        result.resize(self.parent.num_llm_tokens, false);
        for (parse_state, tokenizer_state_ids) in &self.states {
            for tokenizer_state in tokenizer_state_ids {
                if let Some(token_sequence_map) = self.parent.precomputed.get(tokenizer_state) {
                    for (tokenizer_token_sequence, (_, bitsets)) in token_sequence_map.flatten(|(x, _)| !x.is_empty()) {
                        let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_state(parse_state.clone());
                        let grammar_token_id_sequence = tokenizer_token_sequence.iter().map(|t| table::TerminalID(*t)).collect::<Vec<_>>();
                        new_glr_parse_state.parse_part(&grammar_token_id_sequence);
                        if new_glr_parse_state.is_ok() {
                            for (possible_next_grammar_token, bitset) in bitsets {
                                let mut new_new_glr_parse_state = new_glr_parse_state.clone();
                                let possible_next_grammar_token_id = table::TerminalID(possible_next_grammar_token);
                                new_new_glr_parse_state.step(possible_next_grammar_token_id);
                                if new_new_glr_parse_state.is_ok() {
                                    result |= bitset;
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }

    pub fn commit(&mut self, llm_token_id: LLMTokenID) {
        let mut new_states: BTreeMap<(ParseStateKey, BTreeSet<StateID>), ParseState> = BTreeMap::new();
        for (parse_state, tokenizer_state_ids) in &self.states {
            for tokenizer_state_id in tokenizer_state_ids {
                // todo: should be able to do the below loop more efficiently by optimising the precomputed
                //  stuff for earlier llm token lookup
                for (grammar_token_sequence, (llm_token_id_to_state_id, _)) in &self.parent.precomputed[&tokenizer_state_id].flatten(|(x, _)| !x.is_empty()) {
                    if let Some(&next_tokenizer_state_id) = llm_token_id_to_state_id.get(&llm_token_id) {
                        let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_state(parse_state.clone());
                        let mut grammar_token_id_sequence = grammar_token_sequence.iter().map(|t| table::TerminalID(*t)).collect::<Vec<_>>();
                        // // omit the incomplete tail token
                        // grammar_token_id_sequence.pop();

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