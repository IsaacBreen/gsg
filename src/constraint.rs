// src/constraint.rs
use crate::glr::parser::{GLRParser, GLRParserState, InsertWith, ParseState, ParseStateKey};
use crate::glr::table::{StateID, TerminalID};
use crate::{precompute, debug};
use crate::precompute::{LLMTokenID, Token, TokenID, Tokenizer};
use bitvec::prelude::*;
use bimap::BiBTreeMap;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use crate::trie::TrieNode;

type LLMToken = Vec<u8>;
type LLMTokenMap = BiBTreeMap<Vec<u8>, LLMTokenID>;

#[derive(Debug, Clone)]
pub struct GrammarConstraint<T: Tokenizer> {
    pub(crate) tokenizer: T,
    pub(crate) parser: GLRParser,
    pub precomputed: BTreeMap<StateID, TrieNode<TokenID, (BTreeMap<LLMTokenID, Option<StateID>>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>>,
    pub(crate) max_llm_token_id: usize,
}

#[derive(Debug, Clone)]
pub struct GrammarConstraintState<T: Tokenizer> {
    pub(crate) parent: GrammarConstraint<T>,
    pub(crate) states: Vec<(ParseState, BTreeSet<StateID>)>,
}

impl<T: Tokenizer> GrammarConstraint<T> {
    pub fn new(tokenizer: T, parser: GLRParser, llm_tokens: LLMTokenMap, eof_llm_token_id: usize, max_llm_token_id: usize) -> Self {
        let mut precomputed = precompute::precompute(&tokenizer, &llm_tokens, LLMTokenID(eof_llm_token_id), max_llm_token_id);
        precompute_add_eof(&mut precomputed, LLMTokenID(eof_llm_token_id), parser.eof_terminal_id.0, max_llm_token_id);

        Self {
            tokenizer,
            parser,
            precomputed,
            max_llm_token_id,
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

pub(crate) fn precompute_add_eof(
    precomputed: &mut BTreeMap<StateID, TrieNode<TokenID, (BTreeMap<LLMTokenID, Option<StateID>>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>>,
    eof_llm_token_id: LLMTokenID,
    eof_grammar_token_id: TokenID,
    max_llm_token_id: usize,
) {
    let mut bitset = bitvec![0; max_llm_token_id + 1];
    bitset.set(eof_llm_token_id.0, true);
    let node = precomputed.get_mut(&StateID(0)).expect("State 0 should exist");
    node.value.1.insert(eof_grammar_token_id, bitset);
}

impl<'a, T: Tokenizer> GrammarConstraintState<T> {
    pub fn get_mask(&self) -> BitVec {
        let mut result = bitvec![0; self.parent.max_llm_token_id + 1];
        debug!(2, "Getting mask");

        for (parse_state, tokenizer_state_ids) in &self.states {
            debug!(2, "Getting mask for parse state {:?}, tokenizer states {:?}", parse_state, tokenizer_state_ids);
            for tokenizer_state in tokenizer_state_ids {
                let token_sequence_map = &self.parent.precomputed[tokenizer_state];
                TrieNode::special_map(
                    // todo (performance): unnecessary clone
                    Arc::new(Mutex::new(token_sequence_map.clone())),
                    vec![parse_state.clone()],
                    // todo: it's messy that we need to access the value in dst_node here.
                    |current_parse_states, token_id, _dst_node| {
                        // todo: this is introducing redundancy... ?
                        let mut glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(current_parse_states.clone());
                        debug!(3, "Stepping");
                        glr_parse_state.step(TerminalID(*token_id));
                        debug!(3, "Done stepping");
                        glr_parse_state.active_states
                    },
                    |parse_states: Vec<Vec<ParseState>>| {
                        let all_parse_states: Vec<ParseState> = parse_states.into_iter().flatten().collect();
                        let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(all_parse_states);
                        debug!(3, "Merging active states");
                        new_glr_parse_state.merge_active_states();
                        debug!(3, "Done merging active states");
                        new_glr_parse_state.active_states
                    },
                    |(_, bitsets, maybe_clean_end_bitset), current_parse_states| {
                        let mut glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(current_parse_states.clone());
                        if glr_parse_state.is_ok() {
                            for (possible_next_grammar_token, bitset) in bitsets {
                                let mut new_glr_parse_state = glr_parse_state.clone();
                                let possible_next_grammar_token_id = TerminalID(*possible_next_grammar_token);
                                debug!(3, "Stepping for possible next grammar token {:?}", possible_next_grammar_token);
                                new_glr_parse_state.step(possible_next_grammar_token_id);
                                debug!(3, "Done stepping for possible next grammar token");

                                if new_glr_parse_state.is_ok() {
                                    result |= bitset;
                                }
                            }
                            if let Some(bitset) = maybe_clean_end_bitset {
                                result |= bitset;
                            }
                        }
                    },
                );
                debug!(2, "Done");
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
                TrieNode::special_map(
                    Arc::new(Mutex::new(self.parent.precomputed[&tokenizer_state_id].clone())),
                    vec![parse_state.clone()],
                    // todo: it's messy that we need to access the value in dst_node here.
                    |current_parse_states, token_id, _dst_node| {
                        // todo: this is introducing redundancy... ?
                        let mut glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(current_parse_states.clone());
                        glr_parse_state.step(TerminalID(*token_id));
                        glr_parse_state.active_states
                    },
                    |parse_states: Vec<Vec<ParseState>>| {
                        let all_parse_states: Vec<ParseState> = parse_states.into_iter().flatten().collect();
                        let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(all_parse_states);
                        new_glr_parse_state.merge_active_states();
                        new_glr_parse_state.active_states
                    },
                    |(llm_token_id_to_state_id, _, _), current_parse_states| {
                        let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(current_parse_states.clone());
                        if let Some(info) = llm_token_id_to_state_id.get(&llm_token_id) {
                            for active_parse_state in new_glr_parse_state.active_states {
                                new_states.insert_with(
                                    (active_parse_state.key(), BTreeSet::from([info.unwrap_or(StateID(0))])),
                                    active_parse_state,
                                    |old, new| {
                                        old.merge(new);
                                    },
                                );
                            }
                        }
                    },
                )
            }
        }
        self.states = new_states.into_iter().map(|((_key, tokenizer_state_ids), parse_state)| {
            (parse_state, tokenizer_state_ids)
        }).collect();
    }

    pub fn commit_many(&mut self, llm_token_ids: &[LLMTokenID]) {
        for &llm_token_id in llm_token_ids {
            self.commit(llm_token_id);
        }
    }
}
