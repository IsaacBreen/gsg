use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Regex {
    dfa: DFA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Success {
    pub position: usize,
    pub group_id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct FindReturn {
    pub position: usize,
    pub inner: Option<Success>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexState {
    pub regex: Regex,
    find_return: Option<FindReturn>,
    pub(crate) position: usize,
    current_state: usize,
    prev_finalizer: Option<Finalizer>,
    prev_finalizer_position: usize,
    pub failed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Char(u8),
    CharClass(U8Set),
    Quantifier(Box<Expr>, QuantifierType),
    Choice(Vec<Expr>),
    Seq(Vec<Expr>),
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

impl<T> From<T> for Expr where T: ToString {
    fn from(t: T) -> Self {
        Expr::Seq(t.to_string().chars().map(|c| Expr::Char(c as u8)).collect())
    }
}

impl<T> From<T> for ExprGroup where T: ToString {
    fn from(t: T) -> Self {
        ExprGroup {
            expr: t.into(),
            precedence: 0,
        }
    }
}

pub fn char(c: u8) -> Expr {
    if c == b'\0' {
        Expr::Seq(vec![])
    } else {
        Expr::Char(c as u8)
    }
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
    Expr::Seq(vec![])
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

/// **Example:**
/// ```rust,ignore
/// groups![
///     'a',
///     'b',
///     seq!['c', 'd'],
/// ]
/// ```
///
/// **Output:**
/// ```rust,ignore
/// ExprGroups {
///     groups: vec![
///         'a'.into(),
///         'b'.into(),
///         seq!['c', 'd'].into(),
///     ]
/// }
/// ```
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

            for (transition_char, next_states) in &state.transitions {
                f.write_str(&format!("  - '{}': {:?}\n", transition_char, next_states))?;
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

            for (transition_char, next_state) in &state.transitions {
                f.write_str(&format!("  - '{}': {}\n", transition_char, next_state))?;
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
            Expr::Char((c)) => {
                let new_state = nfa.add_state();
                nfa.add_transition(current_state, c, new_state);
                new_state
            },
            Expr::CharClass(chars) => {
                let new_state = nfa.add_state();
                for ch in chars.iter() {
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
                        nfa.add_transition(current_state, b'\0', loop_start_state);

                        // Process the expr
                        let expr_end_state = Self::handle_expr(*expr, nfa, loop_start_state);

                        // Epsilon transition from expr end state back to loop start state for repetition
                        nfa.add_transition(expr_end_state, b'\0', loop_start_state);

                        // Epsilon transition from loop start state to loop end state to allow skipping
                        nfa.add_transition(loop_start_state, b'\0', loop_end_state);

                        // The loop end state becomes the new current state
                        loop_end_state
                    },
                    QuantifierType::OneOrMore => {
                        let loop_start_state = nfa.add_state();

                        // Process the expr first to ensure at least one occurrence
                        let expr_end_state = Self::handle_expr(*expr, nfa, current_state);

                        // Epsilon transition from expr end state back to loop start state for repetition
                        nfa.add_transition(expr_end_state, b'\0', loop_start_state);

                        // Epsilon transition from loop start state back to expr start state to allow repetition
                        nfa.add_transition(loop_start_state, b'\0', current_state);

                        // The expr end state becomes the new current state
                        expr_end_state
                    },
                    QuantifierType::ZeroOrOne => {
                        let optional_end_state = nfa.add_state();

                        // Epsilon transition from current state to optional end state to allow skipping
                        nfa.add_transition(current_state, b'\0', optional_end_state);

                        // Process the expr
                        let expr_end_state = Self::handle_expr(*expr, nfa, current_state);

                        // Epsilon transition from expr end state to optional end state
                        nfa.add_transition(expr_end_state, b'\0', optional_end_state);

                        // The optional end state becomes the new current state
                        optional_end_state
                    },
                }
            },
            Expr::Choice(exprs) => {
                let choice_start_state = nfa.add_state(); // New start state for choice
                let choice_end_state = nfa.add_state(); // New end state for choice

                // Epsilon transition from the current state to the start state of the choice
                nfa.add_transition(current_state, b'\0', choice_start_state);

                for expr in exprs {
                    // For each expr, connect the start state of the choice to the start state of the expr
                    let expr_start_state = nfa.add_state();
                    nfa.add_transition(choice_start_state, b'\0', expr_start_state);

                    // Process the expr and get its end state
                    let expr_end_state = Self::handle_expr(expr, nfa, expr_start_state);

                    // Connect the end state of the expr to the end state of the choice
                    nfa.add_transition(expr_end_state, b'\0', choice_end_state);
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
        }
    }
}

impl NFA {
    pub fn add_state(&mut self) -> usize {
        let new_index = self.states.len();
        self.states.push(NFAState::new());
        new_index
    }

    pub fn add_transition(&mut self, from: usize, on_char: u8, to: usize) {
        self.states[from]
            .transitions
            .entry(on_char)
            .or_insert(Vec::new())
            .push(to);
    }

    pub fn to_dfa(self) -> DFA {
        let mut dfa_states: Vec<DFAState> = Vec::new();
        let mut dfa_state_map: HashMap<FrozenSet<usize>, usize> = HashMap::new();
        let mut worklist: Vec<FrozenSet<usize>> = Vec::new();

        let mut epsilon_closures = self.compute_epsilon_closures();

        // Compute the epsilon closure of the NFA start state and use it as the DFA start state
        // let start_closure = self.epsilon_closure(&HashSet::from([self.start_state]));
        let start_closure = epsilon_closures[self.start_state].clone();
        let start_state = FrozenSet::from(start_closure);
        worklist.push(start_state.clone());
        dfa_state_map.insert(start_state.clone(), 0);

        // Initialize the first DFA state
        // let closure = self.epsilon_closure(&HashSet::from([self.start_state]));
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
                for (transition_char, next_states) in &self.states[state].transitions {
                    if transition_char == b'\0' { continue; } // Skip epsilon transitions here

                    let entry = transition_map.entry(transition_char).or_insert_with(HashSet::new);
                    for &next_state in next_states {
                        entry.insert(next_state);
                    }
                }
            }

            // For each transition, compute the epsilon closure of the resulting state set
            for (transition_char, next_states) in &transition_map {
                // let closure = self.epsilon_closure(&next_states);
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
                        // Finalise on the first group
                        finalizer: closure.iter().filter_map(|&state| self.states[state].finalizer).min_by_key(|finalizer| (-finalizer.precedence, finalizer.group)),
                    });
                }

                let next_dfa_state = *dfa_state_map.get(&frozen_closure).unwrap();
                dfa_states[current_dfa_state].transitions.insert(transition_char, next_dfa_state);
            }
        }

        DFA {
            states: dfa_states,
            start_state: 0,
        }
    }

    // fn epsilon_closure(&self, states: &HashSet<usize>) -> HashSet<usize> {
    //     let mut closure = states.clone();
    //     let mut stack: Vec<usize> = states.iter().copied().collect();
    //
    //     while let Some(state) = stack.pop() {
    //         if let Some(next_states) = self.states[state].transitions.get('\0') {
    //             for &next_state in next_states {
    //                 if closure.insert(next_state) {
    //                     stack.push(next_state);
    //                 }
    //             }
    //         }
    //     }
    //
    //     closure
    // }

    fn epsilon_closure(&self, state: usize) -> HashSet<usize> {
        let mut closure = HashSet::new();
        let mut stack = vec![state];

        while let Some(state) = stack.pop() {
            if closure.insert(state) {
                if let Some(next_states) = self.states[state].transitions.get(b'\0') {
                    stack.extend(next_states);
                }
            }
        }

        closure
    }

    fn compute_epsilon_closures(&self) -> Vec<HashSet<usize>> {
        (0..self.states.len()).map(|state| self.epsilon_closure(state)).collect()
    }
}

impl RegexState {
    pub fn execute(&mut self, text: &[u8]) {
        let dfa = &self.regex.dfa;
        let mut local_position = 0;
        while local_position < text.len() {
            let state_data = &dfa.states[self.current_state];
            let next_char = text[local_position];
            if let Some(&next_state) = state_data.transitions.get(next_char) {
                self.current_state = next_state;
                self.position += 1;
                local_position += 1;
                // If the next state has a finalizer, and its precedence is greater than or equal to that of the current finalizer, replace the current finalizer
                if let Some(finalizer) = dfa.states[self.current_state].finalizer {
                    if self.prev_finalizer.is_none() || finalizer.precedence >= self.prev_finalizer.unwrap().precedence {
                        self.prev_finalizer = Some(finalizer);
                        self.prev_finalizer_position = self.position;
                    }
                }
            } else {
                // If no transition exists for the current character, we're finished
                self.end();
                return;
            }
        }
        // If there's nowhere to go at this point, we're finished
        if dfa.states[self.current_state].transitions.is_empty() {
            self.end();
        }
    }

    pub fn end(&mut self) {
        self.find_return = Some(FindReturn {
            position: self.position,
            inner: self.prev_finalizer.map(|finalizer| Success { position: self.prev_finalizer_position, group_id: finalizer.group }),
        });
    }
}

impl RegexState {
    pub fn get_possible_next_chars(&self) -> U8Set {
        let mut possible_chars = U8Set::new();
        let current_dfa_state = &self.regex.dfa.states[self.current_state];

        // Iterate through all transitions from the current state
        for (char, &next_state) in &current_dfa_state.transitions {
            possible_chars.insert(char);

            // If the next state has a finalizer, add all characters that can be reached from it
            if self.regex.dfa.states[next_state].finalizer.is_some() {
                self.add_reachable_chars(next_state, &mut possible_chars);
            }
        }

        possible_chars
    }

    fn add_reachable_chars(&self, state: usize, char_set: &mut U8Set) {
        let mut visited = HashSet::new();
        let mut stack = vec![state];

        while let Some(current_state) = stack.pop() {
            if visited.insert(current_state) {
                for (char, &next_state) in &self.regex.dfa.states[current_state].transitions {
                    char_set.insert(char);
                    stack.push(next_state);
                }
            }
        }
    }

    pub fn get_prev_match(&self) -> Option<Success> {
        // Returns the previous match if it exists
        self.prev_finalizer.map(|finalizer| Success { position: self.prev_finalizer_position, group_id: finalizer.group })
    }

    pub fn done(&self) -> bool {
        // Returns true if the regex has matched and cannot possibly match any more
        todo!()
    }
}

impl Regex {
    pub fn init(&self) -> RegexState {
        RegexState {
            regex: self.clone(),
            find_return: Default::default(),
            position: 0,
            current_state: 0,
            prev_finalizer: self.dfa.states[self.dfa.start_state].finalizer,
            prev_finalizer_position: 0,
            failed: false,
        }

    }

    pub fn find(&self, text: &[u8]) -> FindReturn {
        let mut regex_state = self.init();
        regex_state.execute(text);
        regex_state.end();
        if let Some(find_return) = regex_state.find_return {
            find_return
        } else {
            FindReturn {
                position: regex_state.position,
                inner: None,
            }
        }
    }

    pub fn is_match(&self, text: &[u8]) -> bool {
        self.find(text).inner.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let expr: Expr = 'a'.into();
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.is_match(b"a"));
        assert!(!regex.is_match(b"b"));
    }

    #[test]
    fn test_quantifier() {
        let expr = rep('a');
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.is_match(b""));
        assert!(regex.is_match(b"a"));
        assert!(regex.is_match(b"aaaa"));
        assert!(regex.is_match(b"b"));

        let mut state = regex.init();
        state.execute(b"aa");
        assert_eq!(state.get_prev_match(), Some(Success { position: 2, group_id: 0 }));
        assert!(!state.done()); // Could match more 'a's
    }

    #[test]
    fn test_choice() {
        let expr = choice!['a', 'b'];
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.is_match(b"a"));
        assert!(regex.is_match(b"b"));
        assert!(!regex.is_match(b"c"));
    }

    #[test]
    fn test_seq() {
        let expr = seq!['a', 'b'];
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert!(!regex.is_match(b"a"));
        assert!(!regex.is_match(b"b"));
        assert!(regex.is_match(b"ab"));
        assert!(regex.is_match(b"abab"));
        assert!(!regex.is_match(b"c"));
    }
}

#[cfg(test)]
mod complex_tests {
    use super::*;

    #[test]
    fn test_nested_quantifiers() {
        let expr = rep1(rep('a'));
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.is_match(b"a"));
        assert!(regex.is_match(b"aa"));
        assert!(regex.is_match(b"aaa"));
        assert!(regex.is_match(b""));
    }

    #[test]
    fn test_complex_choice() {
        let expr = choice![
            seq!['a', rep1('b')],
            'c',
        ];
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.is_match(b"ab"));
        assert!(regex.is_match(b"abb"));
        assert!(regex.is_match(b"c"));
        assert!(!regex.is_match(b"a"));
        assert!(!regex.is_match(b"b"));
        assert!(regex.is_match(b"cc"));
    }

    #[test]
    fn test_complex_seq_with_quantifiers() {
        let expr = seq![
            rep('a'),
            'b',
            rep1('c'),
        ];
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.is_match(b"bc"));
        assert!(regex.is_match(b"bcc"));
        assert!(regex.is_match(b"abcc"));
        assert!(regex.is_match(b"aaabccc"));
        assert!(!regex.is_match(b"a"));
        assert!(!regex.is_match(b"b"));
        assert!(!regex.is_match(b"c"));
    }

    #[test]
    fn test_complex_pattern() {
        let expr = seq![
            rep(choice!['a', 'b']),
            'c',
            rep1(choice!['d', 'e']),
        ];
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.is_match(b"cd"));
        assert!(regex.is_match(b"ce"));
        assert!(regex.is_match(b"cde"));
        assert!(regex.is_match(b"aced"));
        assert!(regex.is_match(b"bacde"));
        assert!(!regex.is_match(b"a"));
        assert!(!regex.is_match(b"b"));
        assert!(!regex.is_match(b"c"));
    }
}

#[cfg(test)]
mod even_more_complex_tests {
    use super::*;

    #[test]
    fn test_overlapping_character_classes() {
        let expr = seq![
            choice!['a', 'b', 'c'],
            choice!['b', 'c', 'd'],
        ];
        let regex = expr.build();

        assert!(regex.is_match(b"bc"));
        assert!(regex.is_match(b"cb"));
        assert!(regex.is_match(b"ab"));
        assert!(regex.is_match(b"cd"));
    }

    #[test]
    fn test_nested_seqs_with_quantifiers() {
        let expr = seq![
            rep(seq!['a', rep1('b')]),
            'c',
        ];
        let regex = expr.build();

        assert!(regex.is_match(b"c"));
        assert!(regex.is_match(b"abc"));
        assert!(regex.is_match(b"abbc"));
        assert!(regex.is_match(b"ababbabc"));
        assert!(!regex.is_match(b"ac"));
    }

    #[test]
    fn test_choice_with_empty_option() {
        let expr = choice!['a', seq![]];
        let regex = expr.build();

        assert!(regex.is_match(b"a"));
        assert!(regex.is_match(b"")); // Should match the empty option

        assert!(regex.is_match(b"ab")); // Partial match allowed
    }

    #[test]
    fn test_complex_pattern_with_overlapping_quantifiers() {
        let expr = seq![
            rep('a'),
            rep1('a'),
        ];
        let regex = expr.build();

        assert!(regex.is_match(b"a"));
        assert!(regex.is_match(b"aa"));
        assert!(!regex.is_match(b""));
        assert!(!regex.is_match(b"b"));
    }

    #[test]
    fn test_matching_at_different_positions() {
        let expr: Expr = 'a'.into();
        let regex = expr.build();

        assert!(regex.is_match(b"a"));
        assert!(!regex.is_match(b"ba"));
        assert!(regex.is_match(b"ab"));
        assert!(!regex.is_match(b"bab"));
        assert!(!regex.is_match(b"b"));
    }

    #[test]
    fn test_explicit_precedence() {
        let expr = groups![
            'a',
            prec(2, 'a'),
            prec(3, 'a'),
        ];
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"a"), FindReturn { position: 1, inner: Some(Success { position: 1, group_id: 2 }) });
    }

    #[test]
    fn test_precedence_with_quantifiers() {
        // This test checks if the engine correctly applies precedence when quantifiers are involved
        let expr = groups![
            prec(1, rep('a')), // 'a' repeated, lower precedence
            prec(2, 'a'),      // single 'a', higher precedence
        ];
        let regex = expr.build();

        // Expect the single 'a' to match due to higher precedence
        assert_eq!(regex.find(b"aaa"), FindReturn { position: 3, inner: Some(Success { position: 1, group_id: 1 }) });
    }

    #[test]
    fn test_precedence_with_choice() {
        // This test checks if the engine correctly applies precedence when choices are involved
        let expr = groups![
            prec(1, choice!['a', 'b']), // choice between 'a' and 'b', lower precedence
            prec(2, 'a'),               // single 'a', higher precedence
        ];
        let regex = expr.build();

        // Expect the single 'a' to match due to higher precedence, even though 'a' is also part of a choice
        assert_eq!(regex.find(b"a"), FindReturn { position: 1, inner: Some(Success { position: 1, group_id: 1 }) });
    }

    #[test]
    fn test_precedence_with_overlap() {
        // This test checks if the engine correctly applies precedence when patterns overlap
        let expr = groups![
            prec(1, seq!['a', 'b']), // sequence 'ab', lower precedence
            prec(2, 'a'),            // single 'a', higher precedence
        ];
        let regex = expr.build();

        // Expect the single 'a' to match due to higher precedence, even though 'ab' is also a valid match
        assert_eq!(regex.find(b"ab"), FindReturn { position: 2, inner: Some(Success { position: 1, group_id: 1 }) });
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

        let expr = Expr::Choice(words.iter().map(|word| Expr::Seq(word.bytes().map(|c| Expr::Char(c)).collect())).collect());
        let regex = expr.build();
        dbg!(&regex);

        assert!(regex.is_match(b"False"));
        assert!(regex.is_match(b"None"));
        assert!(regex.is_match(b"True"));
        assert!(regex.is_match(b"and"));
        assert!(regex.is_match(b"as"));
        assert!(regex.is_match(b"assert"));
    }
}
