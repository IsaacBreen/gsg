// src/tokenizer/finite_automata.rs
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::tokenizer::charmap::TrieMap;

use crate::tokenizer::frozenset::FrozenSet;
use crate::U8Set;

type GroupID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Finalizer {
    pub precedence: isize,
    pub group: GroupID,
}

#[derive(Debug, Clone)]
pub struct NFAState {
    transitions: TrieMap<Vec<usize>>,
    epsilon_transitions: Vec<usize>,
    finalizer: Option<Finalizer>,
}

#[derive(Clone)]
pub struct NFA {
    states: Vec<NFAState>,
    start_state: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DFAState {
    transitions: TrieMap<usize>,
    finalizer: Option<Finalizer>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DFA {
    states: Vec<DFAState>,
    start_state: usize,
}

#[derive(Debug)]
pub struct Regex {
    dfa: DFA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Match {
    pub position: usize,
    pub group_id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct FinalStateReport {
    pub position: usize,
    pub inner: Option<Match>,
}

#[derive(Debug)]
pub struct RegexState<'a> {
    pub regex: &'a Regex,
    pub(crate) position: usize,
    current_state: usize,
    prev_finalizer: Option<Finalizer>,
    prev_finalizer_position: usize,
    done: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    U8(u8),
    U8Class(U8Set),
    Quantifier(Box<Expr>, QuantifierType),
    Choice(Vec<Expr>),
    Seq(Vec<Expr>),
    Epsilon, // Explicit epsilon transition
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QuantifierType {
    ZeroOrMore, // *
    OneOrMore,  // +
    ZeroOrOne,  // ?
}

#[derive(Debug, Clone)]
pub struct ExprGroup {
    pub expr: Expr,
    pub precedence: isize,
}

#[derive(Debug, Clone)]
pub struct ExprGroups {
    pub groups: Vec<ExprGroup>,
}

impl From<Expr> for ExprGroup {
    fn from(expr: Expr) -> Self {
        ExprGroup {
            expr,
            precedence: 0,
        }
    }
}

pub fn eat_u8(c: u8) -> Expr {
    Expr::U8(c)
}

pub fn rep<T: Into<Expr>>(expr: T) -> Expr {
    Expr::Quantifier(Box::new(expr.into()), QuantifierType::ZeroOrMore)
}

pub fn rep1<T: Into<Expr>>(expr: T) -> Expr {
    Expr::Quantifier(Box::new(expr.into()), QuantifierType::OneOrMore)
}

pub fn opt<T: Into<Expr>>(expr: T) -> Expr {
    Expr::Quantifier(Box::new(expr.into()), QuantifierType::ZeroOrOne)
}

pub fn prec<T: Into<Expr>>(precedence: isize, expr: T) -> ExprGroup {
    ExprGroup { expr: expr.into(), precedence }
}

pub fn eps() -> Expr {
    Expr::Epsilon
}

pub fn _seq(exprs: Vec<Expr>) -> Expr {
    Expr::Seq(exprs)
}

pub fn _choice(exprs: Vec<Expr>) -> Expr {
    Expr::Choice(exprs)
}

macro_rules! choice {
    ($($expr:expr),* $(,)?) => {
        Expr::Choice(vec![$($expr.into()),*])
    };
}

macro_rules! seq {
    ($($expr:expr),* $(,)?) => {
        Expr::Seq(vec![$($expr.into()),*])
    };
}

macro_rules! groups {
    ($($expr:expr),* $(,)?) => {
        ExprGroups {
            groups: vec![$($expr.into()),*]
        }
    };
}

impl Debug for NFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Regex State NFA:\n")?;

        for (state_index, state) in self.states.iter().enumerate() {
            f.write_str(&format!("State {}:\n", state_index))?;

            for (transition_u8, next_states) in &state.transitions {
                f.write_str(&format!("  - '{}': {:?}\n", transition_u8, next_states))?;
            }

            for next_state in &state.epsilon_transitions {
                f.write_str(&format!("  - Epsilon: {}\n", next_state))?;
            }

            if let Some(finalizer) = state.finalizer {
                f.write_str(&format!("  - Finalizer: {:?}\n", finalizer))?;
            }
        }

        Ok(())
    }
}

impl Debug for DFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Regex State DFA:\n")?;

        for (state_index, state) in self.states.iter().enumerate() {
            f.write_str(&format!("State {}:\n", state_index))?;

            for (transition_u8, next_state) in &state.transitions {
                f.write_str(&format!("  - '{}': {}\n", transition_u8, next_state))?;
            }

            if let Some(finalizer) = state.finalizer {
                f.write_str(&format!("  - Finalizer: {:?}\n", finalizer))?;
            }
        }

        Ok(())
    }
}

impl NFAState {
    pub fn new() -> NFAState {
        NFAState {
            transitions: TrieMap::new(),
            epsilon_transitions: Vec::new(),
            finalizer: None,
        }
    }
}

impl ExprGroups {
    pub fn build(self) -> Regex {
        Regex {
            dfa: self.build_nfa().to_dfa(),
        }
    }

    fn build_nfa(self) -> NFA {
        let mut nfa = NFA {
            states: vec![NFAState::new()],
            start_state: 0,
        };

        for (group, ExprGroup {expr, precedence}) in self.groups.into_iter().enumerate() {
            let end_state = Expr::handle_expr(expr, &mut nfa, 0);
            nfa.states[end_state].finalizer = Some(Finalizer { group, precedence });
        }

        nfa
    }
}

impl Expr {
    pub fn build(self) -> Regex {
        ExprGroups { groups: vec![ExprGroup { expr: self, precedence: 0 }] }.build()
    }

    fn handle_expr(expr: Expr, nfa: &mut NFA, mut current_state: usize) -> usize {
        match expr {
            Expr::U8((c)) => {
                let new_state = nfa.add_state();
                nfa.add_transition(current_state, c, new_state);
                new_state
            },
            Expr::U8Class(u8s) => {
                let new_state = nfa.add_state();
                for ch in u8s.iter() {
                    nfa.add_transition(current_state, ch, new_state);
                }
                new_state
            }
            Expr::Quantifier(expr, quantifier_type) => {
                match quantifier_type {
                    QuantifierType::ZeroOrMore => {
                        let loop_start_state = nfa.add_state();
                        let loop_end_state = nfa.add_state();

                        // Epsilon transition from current state to loop start state
                        nfa.add_epsilon_transition(current_state, loop_start_state);

                        // Process the expr
                        let expr_end_state = Self::handle_expr(*expr, nfa, loop_start_state);

                        // Epsilon transition from expr end state back to loop start state for repetition
                        nfa.add_epsilon_transition(expr_end_state, loop_start_state);

                        // Epsilon transition from loop start state to loop end state to allow skipping
                        nfa.add_epsilon_transition(loop_start_state, loop_end_state);

                        // The loop end state becomes the new current state
                        loop_end_state
                    },
                    QuantifierType::OneOrMore => {
                        let loop_start_state = nfa.add_state();

                        // Process the expr first to ensure at least one occurrence
                        let expr_end_state = Self::handle_expr(*expr, nfa, current_state);

                        // Epsilon transition from expr end state back to loop start state for repetition
                        nfa.add_epsilon_transition(expr_end_state, loop_start_state);

                        // Epsilon transition from loop start state back to expr start state to allow repetition
                        nfa.add_epsilon_transition(loop_start_state, current_state);

                        // The expr end state becomes the new current state
                        expr_end_state
                    },
                    QuantifierType::ZeroOrOne => {
                        let optional_end_state = nfa.add_state();

                        // Epsilon transition from current state to optional end state to allow skipping
                        nfa.add_epsilon_transition(current_state, optional_end_state);

                        // Process the expr
                        let expr_end_state = Self::handle_expr(*expr, nfa, current_state);

                        // Epsilon transition from expr end state to optional end state
                        nfa.add_epsilon_transition(expr_end_state, optional_end_state);

                        // The optional end state becomes the new current state
                        optional_end_state
                    },
                }
            },
            Expr::Choice(exprs) => {
                let choice_start_state = nfa.add_state(); // New start state for choice
                let choice_end_state = nfa.add_state(); // New end state for choice

                // Epsilon transition from the current state to the start state of the choice
                nfa.add_epsilon_transition(current_state, choice_start_state);

                for expr in exprs {
                    // For each expr, connect the start state of the choice to the start state of the expr
                    let expr_start_state = nfa.add_state();
                    nfa.add_epsilon_transition(choice_start_state, expr_start_state);

                    // Process the expr and get its end state
                    let expr_end_state = Self::handle_expr(expr, nfa, expr_start_state);

                    // Connect the end state of the expr to the end state of the choice
                    nfa.add_epsilon_transition(expr_end_state, choice_end_state);
                }

                // The end state of the choice becomes the new current state
                choice_end_state
            },
            Expr::Seq(exprs) => {
                for expr in exprs {
                    current_state = Self::handle_expr(expr, nfa, current_state);
                }
                current_state
            },
            Expr::Epsilon => {
                let new_state = nfa.add_state();
                nfa.add_epsilon_transition(current_state, new_state);
                new_state
            }
        }
    }
}

impl NFA {
    pub fn add_state(&mut self) -> usize {
        let new_index = self.states.len();
        self.states.push(NFAState::new());
        new_index
    }

    pub fn add_transition(&mut self, from: usize, on_u8: u8, to: usize) {
        self.states[from]
            .transitions
            .entry(on_u8)
            .or_insert(Vec::new())
            .push(to);
    }

    pub fn add_epsilon_transition(&mut self, from: usize, to: usize) {
        self.states[from].epsilon_transitions.push(to);
    }

    pub fn to_dfa(self) -> DFA {
        let mut dfa_states: Vec<DFAState> = Vec::new();
        let mut dfa_state_map: HashMap<FrozenSet<usize>, usize> = HashMap::new();
        let mut worklist: Vec<FrozenSet<usize>> = Vec::new();

        let mut epsilon_closures = self.compute_epsilon_closures();

        // Compute the epsilon closure of the NFA start state and use it as the DFA start state
        let start_closure = epsilon_closures[self.start_state].clone();
        let start_state = FrozenSet::from(start_closure);
        worklist.push(start_state.clone());
        dfa_state_map.insert(start_state.clone(), 0);

        // Initialize the first DFA state
        let closure = epsilon_closures[self.start_state].clone();
        dfa_states.push(DFAState {
            transitions: TrieMap::new(),
            finalizer: closure.iter().filter_map(|&state| self.states[state].finalizer).min_by_key(|finalizer| (finalizer.precedence, finalizer.group)),
        });

        while let Some(current_set) = worklist.pop() {
            let current_dfa_state = *dfa_state_map.get(&current_set).unwrap();
            let mut transition_map: TrieMap<HashSet<usize>> = TrieMap::new();

            // For each state in the current DFA state, look at the NFA transitions
            for &state in current_set.iter() {
                for (transition_u8, next_states) in &self.states[state].transitions {
                    let entry = transition_map.entry(transition_u8).or_insert_with(HashSet::new);
                    for &next_state in next_states {
                        entry.insert(next_state);
                    }
                }
            }

            // For each transition, compute the epsilon closure of the resulting state set
            for (transition_u8, next_states) in &transition_map {
                let mut closure = HashSet::new();
                for &next_state in next_states {
                    closure.extend(epsilon_closures[next_state].iter().cloned());
                }
                let frozen_closure = FrozenSet::from(closure.clone());

                // If this set of states is new, add it as a new DFA state
                if !dfa_state_map.contains_key(&frozen_closure) {
                    let new_state_index = dfa_states.len();
                    dfa_state_map.insert(frozen_closure.clone(), new_state_index);
                    worklist.push(frozen_closure.clone());

                    dfa_states.push(DFAState {
                        transitions: TrieMap::new(),
                        finalizer: closure.iter().filter_map(|&state| self.states[state].finalizer).min_by_key(|finalizer| (-finalizer.precedence, finalizer.group)),
                    });
                }

                let next_dfa_state = *dfa_state_map.get(&frozen_closure).unwrap();
                dfa_states[current_dfa_state].transitions.insert(transition_u8, next_dfa_state);
            }
        }

        DFA {
            states: dfa_states,
            start_state: 0,
        }
    }

    fn epsilon_closure(&self, state: usize) -> HashSet<usize> {
        let mut closure = HashSet::new();
        let mut stack = vec![state];

        while let Some(state) = stack.pop() {
            if closure.insert(state) {
                stack.extend(self.states[state].epsilon_transitions.iter());
            }
        }

        closure
    }

    fn compute_epsilon_closures(&self) -> Vec<HashSet<usize>> {
        (0..self.states.len()).map(|state| self.epsilon_closure(state)).collect()
    }
}

impl RegexState<'_> {
    pub fn execute(&mut self, text: &[u8]) {
        let dfa = &self.regex.dfa;
        let mut local_position = 0;
        while local_position < text.len() {
            let state_data = &dfa.states[self.current_state];
            let next_u8 = text[local_position];
            if let Some(&next_state) = state_data.transitions.get(next_u8) {
                self.current_state = next_state;
                local_position += 1;
                // If the next state has a finalizer, and its precedence is greater than or equal to that of the current finalizer, replace the current finalizer
                if let Some(finalizer) = dfa.states[self.current_state].finalizer {
                    if self.prev_finalizer.is_none() || finalizer.precedence >= self.prev_finalizer.unwrap().precedence {
                        self.prev_finalizer = Some(finalizer);
                        self.prev_finalizer_position = self.position + local_position;
                    }
                }
            } else {
                // If no transition exists for the current u8, we're finished
                self.position += text.len();
                self.end();
                return;
            }
        }
        // Update the position
        self.position += text.len();
        // If there's nowhere to go at this point, we're finished
        if dfa.states[self.current_state].transitions.is_empty() {
            self.end();
        }
    }

    pub fn prev_match(&self) -> Option<Match> {
        self.prev_finalizer.map(|finalizer| Match { position: self.prev_finalizer_position, group_id: finalizer.group })
    }

    pub fn final_match(&self) -> Option<Match> {
        if self.done {
            self.prev_match()
        } else {
            None
        }
    }

    pub fn end(&mut self) {
        self.done = true;
    }

    pub fn final_state_report(&self) -> FinalStateReport {
        FinalStateReport {
            position: self.position,
            inner: self.prev_finalizer.map(|finalizer| Match { position: self.prev_finalizer_position, group_id: finalizer.group }),
        }
    }
}

impl RegexState<'_> {
    pub fn get_u8set(&self) -> U8Set {
        let dfa = &self.regex.dfa;
        let state_data = &dfa.states[self.current_state];
        // Get all possible u8s that can match next
        state_data.transitions.keys_as_u8set()
    }

    pub fn get_terminal_u8set(&self) -> U8Set {
        // Get u8s that could take the regex to a terminal state (a state with a finalizer)
        let mut u8set = U8Set::none();
        for (value, i_next_state) in &self.regex.dfa.states[self.current_state].transitions {
            if self.regex.dfa.states[*i_next_state].finalizer.is_some() {
                u8set.insert(value);
            }
        }
        u8set
    }

    pub fn get_prev_match(&self) -> Option<Match> {
        // Returns the previous match if it exists
        self.prev_finalizer.map(|finalizer| Match { position: self.prev_finalizer_position, group_id: finalizer.group })
    }

    pub fn matches(&self) -> Option<bool> {
        if self.prev_finalizer.is_some() {
            Some(true)
        } else if self.done {
            Some(false)
        } else {
            None
        }
    }

    pub fn definitely_matches(&self) -> bool {
        self.matches().unwrap_or(false)
    }

    pub fn could_match(&self) -> bool {
        self.matches().unwrap_or(true)
    }

    pub fn fully_matches(&self) -> Option<bool> {
        self.get_prev_match().map(|m| m.position == self.position)
    }

    pub fn definitely_fully_matches(&self) -> bool {
        self.fully_matches().unwrap_or(false)
    }

    pub fn could_fully_match(&self) -> bool {
        self.fully_matches().unwrap_or(true)
    }

    pub fn done(&self) -> bool {
        // Returns true if the regex has matched and cannot possibly match anymore
        self.done
    }

    pub fn failed(&self) -> bool {
        // Returns true if the regex has failed to match and cannot possibly match
        !self.could_match()
    }
}

impl Regex {
    pub fn init(&self) -> RegexState {
        RegexState {
            regex: self,
            position: 0,
            current_state: 0,
            prev_finalizer: self.dfa.states[self.dfa.start_state].finalizer,
            prev_finalizer_position: 0,
            done: false,
        }
    }

    pub fn find(&self, text: &[u8]) -> Option<Match> {
        let mut regex_state = self.init();
        regex_state.execute(text);
        regex_state.prev_match()
    }

    pub fn matches(&self, text: &[u8]) -> Option<bool> {
        let mut regex_state = self.init();
        regex_state.execute(text);
        regex_state.matches()
    }

    pub fn definitely_matches(&self, text: &[u8]) -> bool {
        self.matches(text).unwrap_or(false)
    }

    pub fn could_match(&self, text: &[u8]) -> bool {
        self.matches(text).unwrap_or(true)
    }

    pub fn fully_matches(&self, text: &[u8]) -> Option<bool> {
        self.find(text).map(|m| m.position == text.len())
    }

    pub fn definitely_fully_matches(&self, text: &[u8]) -> bool {
        self.fully_matches(text).unwrap_or(false)
    }

    pub fn could_fully_match(&self, text: &[u8]) -> bool {
        self.fully_matches(text).unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let expr: Expr = eat_u8(b'a');
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b"a"));
        assert!(!regex.could_match(b"b"));

        assert!(!regex.definitely_matches(b"")); // Incomplete match not allowed
        assert!(regex.could_match(b"")); // Incomplete match allowed
        assert!(regex.definitely_matches(b"ab")); // Prefix match allowed
        assert!(regex.definitely_matches(b"aa")); // Prefix match allowed
    }

    #[test]
    fn test_quantifier() {
        let expr = rep(eat_u8(b'a'));
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b""));
        assert!(regex.definitely_fully_matches(b"a"));
        assert!(regex.definitely_fully_matches(b"aaaa"));
        assert!(regex.definitely_matches(b"b"));

        let mut state = regex.init();
        state.execute(b"aa");
        assert_eq!(state.get_prev_match(), Some(Match { position: 2, group_id: 0 }));
        assert!(!state.done()); // Could match more 'a's
    }

    #[test]
    fn test_choice() {
        let expr = choice![eat_u8(b'a'), eat_u8(b'b')];
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b"a"));
        assert!(regex.definitely_fully_matches(b"b"));
        assert!(!regex.could_match(b"c"));
    }

    #[test]
    fn test_seq() {
        let expr = seq![eat_u8(b'a'), eat_u8(b'b')];
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.could_match(b"a"));
        assert!(!regex.definitely_matches(b"a"));
        assert!(!regex.could_match(b"b"));
        assert!(regex.definitely_matches(b"ab"));
        assert!(regex.definitely_matches(b"abab"));
        assert!(!regex.could_match(b"c"));
    }

    #[test]
    fn test_opt() {
        let expr = opt(eat_u8(b'a'));
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b"")); // Optional 'a' can be absent
        assert!(regex.definitely_fully_matches(b"a")); // Optional 'a' can be present
        assert!(!regex.could_fully_match(b"aa")); // Should not match more than one 'a'
        assert!(regex.could_match(b"b")); // Can still match the empty string in "b"
    }

    #[test]
    fn test_0() {
        let expr = eat_u8(0);
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b"\0"));
        assert!(!regex.could_match(b"1"));
    }

    #[test]
    fn test_epsilon() {
        let expr = eps();
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b""));
        assert!(regex.definitely_matches(b"a")); // Epsilon matches the empty string at the beginning
        assert!(!regex.definitely_fully_matches(b"a"));
    }
}

#[cfg(test)]
mod complex_tests {
    use super::*;

    #[test]
    fn test_nested_quantifiers() {
        let expr = rep1(rep(eat_u8(b'a')));
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b"a"));
        assert!(regex.definitely_fully_matches(b"aa"));
        assert!(regex.definitely_fully_matches(b"aaa"));
        assert!(regex.definitely_fully_matches(b""));
    }

    #[test]
    fn test_complex_choice() {
        let expr = choice![
            seq![eat_u8(b'a'), rep1(eat_u8(b'b'))],
            eat_u8(b'c'),
        ];
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b"ab"));
        assert!(regex.definitely_fully_matches(b"abb"));
        assert!(regex.definitely_fully_matches(b"c"));
        assert!(regex.could_match(b"a"));
        assert!(!regex.definitely_matches(b"a"));
        assert!(!regex.could_match(b"b"));
        assert!(regex.definitely_matches(b"cc"));
        assert!(!regex.definitely_fully_matches(b"cc"));
    }

    #[test]
    fn test_complex_seq_with_quantifiers() {
        let expr = seq![
            rep(eat_u8(b'a')),
            eat_u8(b'b'),
            rep1(eat_u8(b'c')),
        ];
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b"bc"));
        assert!(regex.definitely_fully_matches(b"bcc"));
        assert!(regex.definitely_fully_matches(b"abcc"));
        assert!(regex.definitely_fully_matches(b"aaabccc"));
        assert!(regex.could_match(b"a"));
        assert!(regex.could_match(b"b"));
        assert!(!regex.could_match(b"c"));
    }

    #[test]
    fn test_complex_pattern() {
        let expr = seq![
            rep(choice![eat_u8(b'a'), eat_u8(b'b')]),
            eat_u8(b'c'),
            rep1(choice![eat_u8(b'd'), eat_u8(b'e')]),
        ];
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b"cd"));
        assert!(regex.definitely_fully_matches(b"ce"));
        assert!(regex.definitely_fully_matches(b"cde"));
        assert!(regex.definitely_fully_matches(b"aced"));
        assert!(regex.definitely_fully_matches(b"bacde"));
        assert!(regex.could_fully_match(b"a"));
        assert!(!regex.definitely_matches(b"a"));
        assert!(!regex.definitely_matches(b"b"));
        assert!(regex.could_match(b"c"));
        assert!(!regex.definitely_matches(b"c"));
        assert!(!regex.could_match(b"d"));
    }
}

#[cfg(test)]
mod even_more_complex_tests {
    use crate::eat;
    use super::*;

    #[test]
    fn test_overlapping_u8_classes() {
        let expr = seq![
            choice![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')],
            choice![eat_u8(b'b'), eat_u8(b'c'), eat_u8(b'd')],
        ];
        let regex = expr.build();

        assert!(regex.definitely_fully_matches(b"bc"));
        assert!(regex.definitely_fully_matches(b"cb"));
        assert!(regex.definitely_fully_matches(b"ab"));
        assert!(regex.definitely_fully_matches(b"cd"));
    }

    #[test]
    fn test_nested_seqs_with_quantifiers() {
        let expr = seq![
            rep(seq![eat_u8(b'a'), rep1(eat_u8(b'b'))]),
            eat_u8(b'c'),
        ];
        let regex = expr.build();

        assert!(regex.definitely_fully_matches(b"c"));
        assert!(regex.definitely_fully_matches(b"abc"));
        assert!(regex.definitely_fully_matches(b"abbc"));
        assert!(regex.definitely_fully_matches(b"ababbabc"));
        assert!(!regex.could_match(b"ac"));
    }

    #[test]
    fn test_choice_with_empty_option() {
        let expr = choice![eat_u8(b'a'), seq![]];
        let regex = expr.build();

        assert!(regex.definitely_fully_matches(b"a"));
        assert!(regex.definitely_fully_matches(b"")); // Should match the empty option
    }

    #[test]
    fn test_complex_pattern_with_overlapping_quantifiers() {
        let expr = seq![
            rep(eat_u8(b'a')),
            rep1(eat_u8(b'a')),
        ];
        let regex = expr.build();

        assert!(regex.definitely_fully_matches(b"a"));
        assert!(regex.definitely_fully_matches(b"aa"));
        assert!(regex.could_match(b""));
        assert!(regex.could_fully_match(b""));
        assert!(!regex.could_match(b"b"));
    }

    #[test]
    fn test_matching_at_different_positions() {
        let expr: Expr = eat_u8(b'a');
        let regex = expr.build();

        assert!(regex.definitely_fully_matches(b"a"));
        assert!(!regex.could_match(b"ba"));
        assert!(regex.definitely_matches(b"ab"));
        assert!(!regex.definitely_fully_matches(b"ab"));
        assert!(!regex.could_match(b"bab"));
        assert!(!regex.could_match(b"b"));
    }

    #[test]
    fn test_explicit_precedence() {
        let expr = groups![
            eat_u8(b'a'),
            prec(2, eat_u8(b'a')),
            prec(3, eat_u8(b'a')),
        ];
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"a"), Some(Match { position: 1, group_id: 2 }));
    }

    #[test]
    fn test_precedence_with_quantifiers() {
        // This test checks if the engine correctly applies precedence when quantifiers are involved
        let expr = groups![
            prec(1, rep(eat_u8(b'a'))), // 'a' repeated, lower precedence
            prec(2, eat_u8(b'a')),      // single 'a', higher precedence
        ];
        let regex = expr.build();

        // Expect the single 'a' to match due to higher precedence
        assert_eq!(regex.find(b"aaa"), Some(Match { position: 1, group_id: 1 }));
    }

    #[test]
    fn test_precedence_with_choice() {
        // This test checks if the engine correctly applies precedence when choices are involved
        let expr = groups![
            prec(1, choice![eat_u8(b'a'), eat_u8(b'b')]), // choice between 'a' and 'b', lower precedence
            prec(2, eat_u8(b'a')),                        // single 'a', higher precedence
        ];
        let regex = expr.build();

        // Expect the single 'a' to match due to higher precedence, even though 'a' is also part of a choice
        assert_eq!(regex.find(b"a"), Some(Match { position: 1, group_id: 1 }));
    }

    #[test]
    fn test_precedence_with_overlap() {
        // This test checks if the engine correctly applies precedence when patterns overlap
        let expr = groups![
            prec(1, seq![eat_u8(b'a'), eat_u8(b'b')]), // sequence 'ab', lower precedence
            prec(2, eat_u8(b'a')),                     // single 'a', higher precedence
        ];
        let regex = expr.build();

        // Expect the single 'a' to match due to higher precedence, even though 'ab' is also a valid match
        assert_eq!(regex.find(b"ab"), Some(Match { position: 1, group_id: 1 }));
    }

    #[test]
    fn test_lots_of_words() {
        let words = [
            "False",
            "None",
            "True",
            "and",
            "as",
            "assert",
            "async",
            "await",
            "break",
            "class",
            "continue",
            "def",
            "del",
            "elif",
            "else",
            "except",
            "finally",
            "for",
            "from",
            "global",
            "if",
            "import",
            "in",
            "is",
            "lambda",
            "nonlocal",
            "not",
            "or",
            "pass",
            "raise",
            "return",
            "try",
            "while",
            "with",
            "yield",
        ];

        let expr = Expr::Choice(words.iter().map(|word| Expr::Seq(word.bytes().map(|c| Expr::U8(c)).collect())).collect());
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.definitely_fully_matches(b"False"));
        assert!(regex.definitely_fully_matches(b"None"));
        assert!(regex.definitely_fully_matches(b"True"));
        assert!(regex.definitely_fully_matches(b"and"));
        assert!(regex.definitely_fully_matches(b"as"));
        assert!(regex.definitely_fully_matches(b"assert"));
    }
}
