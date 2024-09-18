use crate::charmap::TrieMap;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use crate::frozenset::FrozenSet;
use crate::u8set::U8Set;

type GroupID = usize;

#[derive(Debug, Clone)]
pub struct NFAState {
    transitions: TrieMap<Vec<usize>>,
    epsilon_transitions: Vec<usize>,
    finalizers: BTreeSet<GroupID>,
}

#[derive(Clone)]
pub struct NFA {
    states: Vec<NFAState>,
    start_state: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DFAState {
    transitions: TrieMap<usize>,
    finalizers: BTreeSet<GroupID>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DFA {
    states: Vec<DFAState>,
    start_state: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Regex {
    dfa: DFA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Match {
    pub num_leftover: usize,
    pub group_id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct FinalStateReport {
    pub inner: BTreeMap<GroupID, usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegexState<'a> {
    pub regex: &'a Regex,
    current_state: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    U8Seq(Vec<u8>),
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
    pub group: GroupID,
}

#[derive(Debug, Clone)]
pub struct ExprGroups {
    pub groups: Vec<ExprGroup>,
}

impl From<Expr> for ExprGroup {
    fn from(expr: Expr) -> Self {
        ExprGroup {
            expr,
            group: 0,
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

pub fn group<T: Into<Expr>>(group: GroupID, expr: T) -> ExprGroup {
    ExprGroup { expr: expr.into(), group }
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

#[macro_export]
macro_rules! choice {
    ($($expr:expr),* $(,)?) => {
        $crate::finite_automata::Expr::Choice(vec![$($expr.into()),*])
    };
}

#[macro_export]
macro_rules! seq {
    ($($expr:expr),* $(,)?) => {
        $crate::finite_automata::Expr::Seq(vec![$($expr.into()),*])
    };
}

#[macro_export]
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

            if !state.finalizers.is_empty() {
                f.write_str(&format!("  - Finalizers: {:?}\n", state.finalizers))?;
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

            if !state.finalizers.is_empty() {
                f.write_str(&format!("  - Finalizers: {:?}\n", state.finalizers))?;
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
            finalizers: BTreeSet::new(),
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

        for ExprGroup {expr, group} in self.groups.into_iter() {
            let end_state = Expr::handle_expr(expr, &mut nfa, 0);
            nfa.states[end_state].finalizers.insert(group);
        }

        nfa
    }
}

impl Expr {
    pub fn build(self) -> Regex {
        ExprGroups { groups: vec![ExprGroup { expr: self, group: 0 }] }.build()
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
            finalizers: closure.iter().flat_map(|&state| self.states[state].finalizers.iter().cloned()).collect(),
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
                        finalizers: closure.iter().flat_map(|&state| self.states[state].finalizers.iter().cloned()).collect(),
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
    pub fn execute(&mut self, text: &[u8]) -> FinalStateReport {
        let dfa = &self.regex.dfa;
        let mut local_position = 0;
        let mut final_state_report = FinalStateReport::default();

        while local_position < text.len() {
            let state_data = &dfa.states[self.current_state];
            let next_u8 = text[local_position];
            if let Some(&next_state) = state_data.transitions.get(next_u8) {
                self.current_state = next_state;
                local_position += 1;
                // Check for finalizers in the current state
                for group_id in &dfa.states[self.current_state].finalizers {
                    final_state_report.inner.insert(*group_id, text.len() - local_position);
                }
            } else {
                // If no transition exists for the current u8, we're finished
                break;
            }
        }

        final_state_report
    }
}

impl Regex {
    pub fn init_to_state(&self, state: usize) -> RegexState {
        RegexState {
            regex: self,
            current_state: state,
        }
    }

    pub fn init(&self) -> RegexState {
        self.init_to_state(self.dfa.start_state)
    }

    pub fn init_all_states(&self) -> Vec<RegexState<'_>> {
        let mut result = vec![];
        for state in 0..self.dfa.states.len() {
            result.push(self.init_to_state(state));
        }
        result
    }

    pub fn find(&self, text: &[u8]) -> FinalStateReport {
        let mut regex_state = self.init();
        regex_state.execute(text)
    }

    pub fn matches(&self, text: &[u8]) -> FinalStateReport {
        let mut regex_state = self.init();
        regex_state.execute(text)
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

        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"b").inner, BTreeMap::new());

        assert_eq!(regex.find(b"").inner, BTreeMap::new()); // Incomplete match not allowed
        assert_eq!(regex.find(b"ab").inner, BTreeMap::from([(0, 1)])); // Prefix match allowed
        assert_eq!(regex.find(b"aa").inner, BTreeMap::from([(0, 1)])); // Prefix match allowed
    }

    #[test]
    fn test_quantifier() {
        let expr = rep(eat_u8(b'a'));
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"aaaa").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"b").inner, BTreeMap::new());
    }

    #[test]
    fn test_choice() {
        let expr = choice![eat_u8(b'a'), eat_u8(b'b')];
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"b").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"c").inner, BTreeMap::new());
    }

    #[test]
    fn test_seq() {
        let expr = seq![eat_u8(b'a'), eat_u8(b'b')];
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"a").inner, BTreeMap::new());
        assert_eq!(regex.find(b"b").inner, BTreeMap::new());
        assert_eq!(regex.find(b"ab").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"abab").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"c").inner, BTreeMap::new());
    }

    #[test]
    fn test_opt() {
        let expr = opt(eat_u8(b'a'));
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"aa").inner, BTreeMap::from([(0, 1)]));
        assert_eq!(regex.find(b"b").inner, BTreeMap::new());
    }

    #[test]
    fn test_0() {
        let expr = eat_u8(0);
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"\0").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"1").inner, BTreeMap::new());
    }

    #[test]
    fn test_epsilon() {
        let expr = eps();
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(0, 1)]));
    }

    #[test]
    fn test_u8seq() {
        let expr = Expr::U8Seq(vec![b'a', b'b']);
        dbg!(&expr);
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"ab").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"a").inner, BTreeMap::new());
        assert_eq!(regex.find(b"b").inner, BTreeMap::new());
        assert_eq!(regex.find(b"ba").inner, BTreeMap::new());
    }

    #[test]
    fn test_multiple_finalizers() {
        let expr = groups![
            group(1, eat_u8(b'a')),
            group(2, eat_u8(b'a')),
        ];
        let regex = expr.build();

        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(1, 0), (2, 0)]));
    }

    #[test]
    fn test_multiple_finalizers_with_quantifier() {
        let expr = groups![
            group(1, rep(eat_u8(b'a'))),
            group(2, eat_u8(b'a')),
        ];
        let regex = expr.build();

        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(1, 0), (2, 0)]));
        assert_eq!(regex.find(b"aa").inner, BTreeMap::from([(1, 0), (2, 1)]));
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

        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"aa").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"aaa").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"").inner, BTreeMap::new());
    }

    #[test]
    fn test_complex_choice() {
        let expr = choice![
            seq![eat_u8(b'a'), rep1(eat_u8(b'b'))],
            eat_u8(b'c'),
        ];
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"ab").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"abb").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"c").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"a").inner, BTreeMap::new());
        assert_eq!(regex.find(b"b").inner, BTreeMap::new());
        assert_eq!(regex.find(b"cc").inner, BTreeMap::from([(0, 1)]));
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

        assert_eq!(regex.find(b"bc").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"bcc").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"abcc").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"aaabccc").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"a").inner, BTreeMap::new());
        assert_eq!(regex.find(b"b").inner, BTreeMap::new());
        assert_eq!(regex.find(b"c").inner, BTreeMap::new());
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

        assert_eq!(regex.find(b"cd").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"ce").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"cde").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"aced").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"bacde").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"a").inner, BTreeMap::new());
        assert_eq!(regex.find(b"b").inner, BTreeMap::new());
        assert_eq!(regex.find(b"c").inner, BTreeMap::new());
        assert_eq!(regex.find(b"d").inner, BTreeMap::new());
    }
}

#[cfg(test)]
mod even_more_complex_tests {
    use super::*;

    #[test]
    fn test_overlapping_u8_classes() {
        let expr = seq![
            choice![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')],
            choice![eat_u8(b'b'), eat_u8(b'c'), eat_u8(b'd')],
        ];
        let regex = expr.build();

        assert_eq!(regex.find(b"bc").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"cb").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"ab").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"cd").inner, BTreeMap::from([(0, 0)]));
    }

    #[test]
    fn test_nested_seqs_with_quantifiers() {
        let expr = seq![
            rep(seq![eat_u8(b'a'), rep1(eat_u8(b'b'))]),
            eat_u8(b'c'),
        ];
        let regex = expr.build();

        assert_eq!(regex.find(b"c").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"abc").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"abbc").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"ababbabc").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"ac").inner, BTreeMap::new());
    }

    #[test]
    fn test_choice_with_empty_option() {
        let expr = choice![eat_u8(b'a'), seq![]];
        let regex = expr.build();

        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"").inner, BTreeMap::from([(0, 0)])); // Should match the empty option
    }

    #[test]
    fn test_complex_pattern_with_overlapping_quantifiers() {
        let expr = seq![
            rep(eat_u8(b'a')),
            rep1(eat_u8(b'a')),
        ];
        let regex = expr.build();

        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"aa").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"").inner, BTreeMap::new());
        assert_eq!(regex.find(b"b").inner, BTreeMap::new());
    }

    #[test]
    fn test_matching_at_different_positions() {
        let expr: Expr = eat_u8(b'a');
        let regex = expr.build();

        assert_eq!(regex.find(b"a").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"ba").inner, BTreeMap::from([(0, 1)]));
        assert_eq!(regex.find(b"ab").inner, BTreeMap::from([(0, 1)]));
        assert_eq!(regex.find(b"bab").inner, BTreeMap::from([(0, 2)]));
        assert_eq!(regex.find(b"b").inner, BTreeMap::new());
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

        let expr = Expr::Choice(words.iter().map(|word| Expr::Seq(word.bytes().map(|c| Expr::U8Seq(vec![c])).collect())).collect());
        let regex = expr.build();
        dbg!(&regex);

        assert_eq!(regex.find(b"False").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"None").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"True").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"and").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"as").inner, BTreeMap::from([(0, 0)]));
        assert_eq!(regex.find(b"assert").inner, BTreeMap::from([(0, 0)]));
    }
}