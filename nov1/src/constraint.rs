// src/constraint.rs
use crate::glr::parser::{GLRParser, InsertWith, ParseState, ParseStateKey};
use crate::glr::table;
use crate::glr::table::StateID;
use crate::precompute;
use crate::precompute::{Token, TokenID, Tokenizer};
use bitvec::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

type LLMToken = Vec<u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LLMTokenID(pub usize);

// TODO: should this *really* derive `Clone`? Users probably shouldn't clone this, should they?
#[derive(Debug, Clone)]
pub struct GrammarConstraint<T: Tokenizer> {
    pub(crate) tokenizer: T,
    pub(crate) parser: GLRParser,
    pub(crate) precomputed: BTreeMap<StateID, BTreeMap<Vec<TokenID>, (BTreeMap<LLMTokenID, StateID>, BitVec)>> ,
    pub(crate) num_llm_tokens: usize,
}

#[derive(Debug, Clone)]
pub struct GrammarConstraintState<T: Tokenizer> {
    pub(crate) parent: GrammarConstraint<T>,
    pub(crate) states: Vec<(ParseState, BTreeSet<StateID>)>,
}

pub fn convert_precomputed_to_llm_token_ids<'a>(
    precomputed: BTreeMap<StateID, BTreeMap<Vec<TokenID>, BTreeMap<&'a [u8], StateID>>>,
    llm_tokens: &[LLMToken],
) -> BTreeMap<StateID, BTreeMap<Vec<TokenID>, (BTreeMap<LLMTokenID, StateID>, BitVec)>> {
    let num_llm_tokens = llm_tokens.len();
    let llm_token_to_id: BTreeMap<_, _> = llm_tokens.iter().enumerate().map(|(i, token)| (token.clone(), LLMTokenID(i))).collect();
    let mut result = BTreeMap::new();
    for (state_id, token_sequence_map) in precomputed {
        let mut new_token_sequence_map = BTreeMap::new();
        for (token_sequence, llm_token_state_map) in token_sequence_map {
            let mut bitset = BitVec::new();
            bitset.resize(num_llm_tokens, false);
            let mut new_llm_token_state_map = BTreeMap::new();
            for (llm_token, next_state_id) in llm_token_state_map {
                let llm_token_id = llm_token_to_id.get(llm_token).unwrap();
                bitset.set(llm_token_id.0, true);
                new_llm_token_state_map.insert(*llm_token_id, next_state_id);
            }
            new_token_sequence_map.insert(token_sequence, (new_llm_token_state_map, bitset));
        }
        result.insert(state_id, new_token_sequence_map);
    }
    result
}

impl<T: Tokenizer> GrammarConstraint<T> {
    pub fn new(tokenizer: T, parser: GLRParser, llm_tokens: &[LLMToken]) -> Self {
        let precomputed = precompute::precompute(&tokenizer, &llm_tokens.iter().map(|token| &token[..]).collect::<Vec<_>>());
        let precomputed = precompute::precompute_add_incomplete_token(&tokenizer, precomputed);
        let precomputed = convert_precomputed_to_llm_token_ids(precomputed, llm_tokens);
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
                    for (tokenizer_token_sequence, (llm_token_id_to_state_id, bitset)) in token_sequence_map {
                        let mut new_glr_parse_state = self.parent.parser.init_glr_parser_from_parse_state(parse_state.clone());
                        let grammar_token_id_sequence = tokenizer_token_sequence.iter().map(|t| table::TerminalID(*t)).collect::<Vec<_>>();
                        new_glr_parse_state.parse_part(&grammar_token_id_sequence);
                        if new_glr_parse_state.is_ok() {
                            result |= bitset;
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
                for (grammar_token_sequence, (llm_token_id_to_state_id, _)) in &self.parent.precomputed[&tokenizer_state_id] {
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

    pub fn performance_report(&self) -> String {
        let mut report = String::new();
        writeln!(report, "Performance Report:").unwrap();
        writeln!(report, "  Number of active states: {}", self.states.len()).unwrap();

        let mut state_id_counts = BTreeMap::new();
        for (_, tokenizer_state_ids) in &self.states {
            for state_id in tokenizer_state_ids {
                *state_id_counts.entry(state_id).or_insert(0) += 1;
            }
        }

        writeln!(report, "  Tokenizer State ID Counts:").unwrap();
        for (state_id, count) in state_id_counts {
            writeln!(report, "    {}: {}", state_id.0, count).unwrap();
        }

        let mut reachable_state_count = 0;
        let mut possible_next_token_sequence_count = 0;
        for (_, tokenizer_state_ids) in &self.states {
            for tokenizer_state in tokenizer_state_ids {
                if let Some(token_sequence_map) = self.parent.precomputed.get(tokenizer_state) {
                    possible_next_token_sequence_count += token_sequence_map.len();
                    for (_, (llm_token_id_to_state_id, _)) in token_sequence_map {
                        reachable_state_count += llm_token_id_to_state_id.len();
                    }
                }
            }
        }
        writeln!(report, "  Number of reachable states to explore: {}", reachable_state_count).unwrap();
        writeln!(report, "  Number of possible next token sequences: {}", possible_next_token_sequence_count).unwrap();

        report
    }
}