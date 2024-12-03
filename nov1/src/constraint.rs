// src/constraint.rs
use crate::glr::parser::{GLRParser, GLRParserState, InsertWith, ParseState, ParseStateKey};
use crate::glr::table;
use crate::glr::table::{StateID, TerminalID};
use crate::{dbgprintln2, precompute};
use crate::precompute::{LLMTokenID, Token, TokenID, Tokenizer, TokenizerStateInfoForLLMToken};
use bitvec::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use crate::trie::TrieNode;

type LLMToken = Vec<u8>;

// TODO: should this *really* derive `Clone`? Users probably shouldn't clone this, should they?
#[derive(Debug, Clone)]
pub struct GrammarConstraint<T: Tokenizer> {
    pub(crate) tokenizer: T,
    pub(crate) parser: GLRParser,
    pub(crate) precomputed: BTreeMap<StateID, TrieNode<TokenID, (BTreeMap<LLMTokenID, TokenizerStateInfoForLLMToken>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>>,
    pub(crate) num_llm_tokens: usize,
}

#[derive(Debug, Clone)]
pub struct GrammarConstraintState<T: Tokenizer> {
    pub(crate) parent: GrammarConstraint<T>,
    pub(crate) states: Vec<(ParseState, BTreeSet<StateID>)>,
}

impl<T: Tokenizer> GrammarConstraint<T> {
    pub fn new(tokenizer: T, parser: GLRParser, llm_tokens: &[LLMToken]) -> Self {
        let num_llm_tokens = llm_tokens.len() + 1;
        let mut precomputed = precompute::precompute(&tokenizer, &llm_tokens.iter().map(|token| &token[..]).collect::<Vec<_>>(), LLMTokenID(num_llm_tokens));
        precompute_add_eof(&mut precomputed, LLMTokenID(llm_tokens.len()), parser.eof_terminal_id.0, num_llm_tokens);

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
        // crate::dbgprintln2!("precomputed.len(): {}", self.precomputed.len());
        // crate::dbgprintln2!("precomputed:");
        // for (tokenizer_state, root) in &self.precomputed {
        //     crate::dbgprintln2!("Tokenizer state: {}", tokenizer_state.0);
        //     for node in TrieNode::all_nodes(Arc::new(Mutex::new(root.clone()))) {
        //         crate::dbgprintln2!("Node address: {:p}, value: {:?}", Arc::as_ptr(&node), node.lock().unwrap().value);
        //         // crate::dbgprintln2!("Node address: {:p}, value: {:?}", Arc::as_ptr(&node), "node.lock().unwrap().value");
        //         // print edge values and destination addresses
        //         for (edge, dest) in node.lock().unwrap().children() {
        //             crate::dbgprintln2!("    Edge value: {:?}, destination address: {:p}", edge, Arc::as_ptr(&dest));
        //         }
        //     }
        // }
        GrammarConstraintState {
            parent: self,
            states: vec![(parser_initial_state, BTreeSet::from([tokenizer_initial_state_id]))],
        }
    }
}

pub fn precompute_add_eof(
    precomputed: &mut BTreeMap<StateID, TrieNode<TokenID, (BTreeMap<LLMTokenID, TokenizerStateInfoForLLMToken>, BTreeMap<TokenID, BitVec>, Option<BitVec>)>>,
    eof_llm_token_id: LLMTokenID,
    eof_grammar_token_id: TokenID,
    num_llm_tokens: usize,
) {
    let mut bitset = BitVec::new();
    bitset.resize(num_llm_tokens, false);
    bitset.set(eof_llm_token_id.0, true);
    let node = precomputed.get_mut(&StateID(0)).unwrap();
    assert!(!node.value.1.contains_key(&eof_grammar_token_id));
    node.value.1.insert(eof_grammar_token_id, bitset);
}

impl<'a, T: Tokenizer> GrammarConstraintState<T> {
    pub fn get_mask(&self) -> BitVec {
        let mut result = BitVec::new();
        result.resize(self.parent.num_llm_tokens, false);
        dbgprintln2!("Getting mask");
        for (parse_state, tokenizer_state_ids) in &self.states {
            dbgprintln2!("Getting mask for parse state {:?}", parse_state);
            for tokenizer_state in tokenizer_state_ids {
                let token_sequence_map = &self.parent.precomputed[tokenizer_state];
                TrieNode::special_map(
                    // todo (performance): unnecessary clone
                    Arc::new(Mutex::new(token_sequence_map.clone())),
                    vec![parse_state.clone()],
                    // todo: it's messy that we need to access the value in dst_node here.
                    |current_parse_states, token_id, dst_node| {
                        // todo: this is introducing redundancy... ?
                        let mut glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(current_parse_states.clone());
                        // println!("token_id: {:?}", token_id);
                        glr_parse_state.step(TerminalID(*token_id));
                        glr_parse_state.active_states
                    },
                    |mut parse_states: Vec<Vec<ParseState>>| {
                        let mut all_parse_states = parse_states.pop().unwrap();
                        for mut other_parse_state in parse_states {
                            all_parse_states.append(&mut other_parse_state)
                        }
                        let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(all_parse_states);
                        new_glr_parse_state.merge_active_states();
                        new_glr_parse_state.active_states
                    },
                    |(_, bitsets, maybe_clean_end_bitset), current_parse_states| {
                        let mut glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(current_parse_states.clone());
                        if glr_parse_state.is_ok() {
                            // dbg!(&bitsets);
                            for (possible_next_grammar_token, bitset) in bitsets {
                                let mut new_glr_parse_state = glr_parse_state.clone();
                                let possible_next_grammar_token_id = table::TerminalID(*possible_next_grammar_token);
                                new_glr_parse_state.step(possible_next_grammar_token_id);
                                // panic!();
                                // todo: remove this
                                println!("possible_next_grammar_token: {:?}", possible_next_grammar_token);
                                // result |= bitset;
                                if new_glr_parse_state.is_ok() {
                                    // dbg!(&bitset);
                                    result |= bitset;
                                }
                            }
                            if let Some(bitset) = maybe_clean_end_bitset {
                                dbg!(&maybe_clean_end_bitset);
                                result |= bitset;
                            }
                        }
                    },
                );
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
                    |current_parse_states, token_id, dst_node| {
                        // todo: this is introducing redundancy... ?
                        let mut glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(current_parse_states.clone());
                        glr_parse_state.step(TerminalID(*token_id));
                        glr_parse_state.active_states
                    },
                    |mut parse_states: Vec<Vec<ParseState>>| {
                        let mut all_parse_states = parse_states.pop().unwrap();
                        for mut other_parse_state in parse_states {
                            all_parse_states.append(&mut other_parse_state)
                        }
                        let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(all_parse_states);
                        new_glr_parse_state.merge_active_states();
                        new_glr_parse_state.active_states
                    },
                    |(llm_token_id_to_state_id, _, _), current_parse_states| {
                        let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_states(current_parse_states.clone());
                        if let Some(info) = llm_token_id_to_state_id.get(&llm_token_id) {
                            for active_parse_state in new_glr_parse_state.active_states {
                                new_states.insert_with(
                                    (active_parse_state.key(), BTreeSet::from([info.dirty_end_state.unwrap_or(StateID(0))])),
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