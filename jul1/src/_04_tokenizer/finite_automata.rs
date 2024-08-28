use crate::tokenizer::charmap::TrieMap;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

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
    is_failing_state: bool, // Flag to indicate a failing state for negative lookahead
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
    forbidden_prefixes: Option<TrieMap<()>>, // Store forbidden prefixes for negative lookaheads
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
    U8Seq(Vec<u8>),
    U8Class(U8Set),
    Quantifier(Box<Expr>, QuantifierType),
    Choice(Vec<Expr>),
    Seq(Vec<Expr>),
    Epsilon, // Explicit epsilon transition
    NegativeLookahead(Box<Expr>),
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
    Expr::U8Seq(vec![c])
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
    ExprGroup {
        expr: expr.into(),
        precedence,
    }
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

// ... (Debug implementations for NFA, DFA)

impl NFAState {
    pub fn new() -> NFAState {
        NFAState {
            transitions: TrieMap::new(),
            epsilon_transitions: Vec::new(),
            finalizer: None,
            is_failing_state: false,
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
        println!("Building NFA...");
        let mut nfa = NFA {
            states: vec![NFAState::new()],
            start_state: 0,
        };

        for (group, ExprGroup { expr, precedence }) in self.groups.into_iter().enumerate() {
            let end_state = Expr::handle_expr(expr, &mut nfa, 0);
            nfa.states[end_state].finalizer = Some(Finalizer { group, precedence });
        }
        println!("Done building NFA");

        nfa
    }
}

impl Expr {
    pub fn build(self) -> Regex {
        ExprGroups {
            groups: vec![ExprGroup {
                expr: self,
                precedence: 0,
            }],
        }
        .build()
    }

    fn handle_expr(expr: Expr, nfa: &mut NFA, mut current_state: usize) -> usize {
        match expr {
            Expr::U8Seq(u8s) => {
                let mut next_state = current_state;
                for c in u8s {
                    let new_state = nfa.add_state();
                    nfa.add_transition(next_state, c, new_state);
                    next_state = new_state;
                }
                next_state
            }
            Expr::U8Class(u8s) => {
                let new_state = nfa.add_state();
                for ch in u8s.iter() {
                    nfa.add_transition(current_state, ch, new_state);
                }
                new_state
            }
            Expr::Quantifier(expr, quantifier_type) => match quantifier_type {
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
                }
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
                }
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
            }
            Expr::Seq(exprs) => {
                for expr in exprs {
                    current_state = Self::handle_expr(expr, nfa, current_state);
                }
                current_state
            }
            Expr::Epsilon => {
                let new_state = nfa.add_state();
                nfa.add_epsilon_transition(current_state, new_state);
                new_state
            }
            Expr::NegativeLookahead(expr) => {
                let lookahead_start_state = nfa.add_state();
                let lookahead_end_state = nfa.add_state();

                // Epsilon transition from current state to lookahead start state
                nfa.add_epsilon_transition(current_state, lookahead_start_state);

                // Process the negated expr
                let negated_expr_end_state = Self::handle_expr(*expr, nfa, lookahead_start_state);

                // Mark the negated_expr_end_state as a failing state for the lookahead
                nfa.states[negated_expr_end_state].is_failing_state = true;

                // Epsilon transition from lookahead start state to lookahead end state (if the negated pattern doesn't match)
                nfa.add_epsilon_transition(lookahead_start_state, lookahead_end_state);

                lookahead_end_state
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
        println!("Converting NFA to DFA...");
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
            finalizer: closure
                .iter()
                .filter_map(|&state| self.states[state].finalizer)
                .min_by_key(|finalizer| (finalizer.precedence, finalizer.group)),
            forbidden_prefixes: None,
        });

        while let Some(current_set) = worklist.pop() {
            let current_dfa_state = *dfa_state_map.get(¤t_set).unwrap();
            let mut transition_map: TrieMap<HashSet<usize>> = TrieMap::new();

            // For each state in the current DFA state, look at the NFA transitions
            for &state in current_set.iter() {
                for (transition_u8, next_states) in &self.states[state].transitions {
                    let entry = transition_map
                        .entry(transition_u8)
                        .or_insert_with(HashSet::new);
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
                        finalizer: closure
                            .iter()
                            .filter_map(|&state| self.states[state].finalizer)
                            .min_by_key(|finalizer| (-finalizer.precedence, finalizer.group)),
                        forbidden_prefixes: None,
                    });
                }

                let next_dfa_state = *dfa_state_map.get(&frozen_closure).unwrap();
                dfa_states[current_dfa_state]
                    .transitions
                    .insert(transition_u8, next_dfa_state);
            }

            // Calculate forbidden prefixes for this DFA state
            dfa_states[current_dfa_state].forbidden_prefixes =
                self.calculate_forbidden_prefixes(&dfa_state_map, current_dfa_state, ¤t_set);
        }
        println!("Done converting NFA to DFA");

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
        (0..self.states.len())
            .map(|state| self.epsilon_closure(state))
            .collect()
    }

    fn calculate_forbidden_prefixes(
        &self,
        dfa_state_map: &HashMap<FrozenSet<usize>, usize>,
        dfa_state: &DFAState,
        nfa_states: &FrozenSet<usize>,
    ) -> Option<TrieMap<()>> {
        // Placeholder for calculating forbidden prefixes based on negative lookaheads

        // This is a simplified example, assuming a maximum prefix length of 2
        let mut forbidden_prefixes: TrieMap<()> = TrieMap::new();

        for &nfa_state_index in nfa_states {
            let nfa_state = &self.states[nfa_state_index];

            // Check for negative lookahead transitions
            for &next_nfa_state_index in &nfa_state.epsilon_transitions {
                let next_nfa_state = &self.states[next_nfa_state_index];
                if next_nfa_state.is_failing_state {
                    // Found a negative lookahead, extract prefixes
                    for (u8_1, next_states_1) in &next_nfa_state.transitions {
                        forbidden_prefixes.insert(*u8_1, ()); // Add single-character prefix

                        for &next_nfa_state_index_2 in next_states_1 {
                            let next_nfa_state_2 = &self.states[next_nfa_state_index_2];
                            for (u8_2, _) in &next_nfa_state_2.transitions {
                                forbidden_prefixes.insert(vec![*u8_1, *u8_2], ()); // Add two-character prefix
                            }
                        }
                    }
                }
            }
        }

        if forbidden_prefixes.is_empty() {
            None
        } else {
            Some(forbidden_prefixes)
        }
    }
}

impl RegexState<'_> {
    pub fn execute(&mut self, text: &[u8]) {
        if self.done {
            self.position += text.len();
            return;
        }
        let dfa = &self.regex.dfa;
        let mut local_position = 0;
        while local_position < text.len() {
            let state_data = &dfa.states[self.current_state];

            // Check for forbidden prefixes
            if let Some(forbidden_prefixes) = &state_data.forbidden_prefixes {
                if let Some(_) = forbidden_prefixes.get(&text[local_position..]) {
                    // Forbidden prefix found, fail the match
                    self.position += text.len();
                    self.end();
                    return;
                }
            }

            let next_u8 = text[local_position];
            if let Some(&next_state) = state_data.transitions.get(next_u8) {
                self.current_state = next_state;
                local_position += 1;
                // If the next state has a finalizer, and its precedence is greater than or equal to that of the current finalizer, replace the current finalizer
                if let Some(finalizer) = dfa.states[self.current_state].finalizer {
                    if self.prev_finalizer.is_none()
                        || finalizer.precedence >= self.prev_finalizer.unwrap().precedence
                    {
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
        self.prev_finalizer
            .map(|finalizer| Match {
                position: self.prev_finalizer_position,
                group_id: finalizer.group,
            })
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
            inner: self.prev_finalizer.map(|finalizer| Match {
                position: self.prev_finalizer_position,
                group_id: finalizer.group,
            }),
        }
    }

    // ... (Other methods in RegexState)
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
        self.prev_finalizer.map(|finalizer| Match {
            position: self.prev_finalizer_position,
            group_id: finalizer.group,
        })
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

    pub fn fully_matches_here(&self) -> bool {
        self.definitely_fully_matches()
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

impl RegexState<'_> {
    /// Finds all matches in the input `bytes` and returns them as a vector of `Match`.
    ///
    /// This method steps through the input bytes, character by character,
    /// and checks if the regex matches at each position. If a match is found,
    /// a `Match` object is created and added to the results vector.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The input bytes to search for matches.
    ///
    /// # Returns
    ///
    /// A vector of `Match` objects, each representing a match found in the input.
    pub fn find_matches(&mut self, bytes: &[u8]) -> Vec<Match> {
        let mut matches = Vec::new(); // Use Vec::new() for clarity
        let mut local_position = 0;
        let dfa = &self.regex.dfa;

        while local_position < bytes.len() {
            let state_data = &dfa.states[self.current_state];

            // Check for forbidden prefixes
            if let Some(forbidden_prefixes) = &state_data.forbidden_prefixes {
                if let Some(_) = forbidden_prefixes.get(&bytes[local_position..]) {
                    // Forbidden prefix found, fail the match and move to the next position
                    local_position += 1;
                    self.current_state = dfa.start_state; // Reset to the initial state
                    continue;
                }
            }

            let next_u8 = bytes[local_position];

            if let Some(&next_state) = state_data.transitions.get(next_u8) {
                self.current_state = next_state;
                local_position += 1;

                if let Some(finalizer) = dfa.states[self.current_state].finalizer {
                    matches.push(Match {
                        position: self.position + local_position,
                        group_id: finalizer.group,
                    });
                }
            } else {
                break;
            }
        }

        self.position += bytes.len();

        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_lookahead() {
        let expr = seq![eat_u8(b'a'), Expr::NegativeLookahead(Box::new(eat_u8(b'b'))), eat_u8(b'c')];
        dbg!(&expr);
        let regex = expr.build();
        dbg!(®ex);

        assert!(regex.definitely_fully_matches(b"ac"));
        assert!(!regex.could_match(b"abc"));
        assert!(!regex.could_match(b"ab"));
        assert!(!regex.could_match(b"bc"));
    }

    // ... (Other tests)
}

// ... (Other modules and code)