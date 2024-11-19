use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonTerminal(pub String);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Terminal(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Production {
    pub lhs: NonTerminal,
    pub rhs: Vec<Symbol>,
}

pub fn nt(name: &str) -> Symbol {
    Symbol::NonTerminal(NonTerminal(name.to_string()))
}

pub fn t(name: &str) -> Symbol {
    Symbol::Terminal(Terminal(name.to_string()))
}

pub fn prod(name: &str, rhs: Vec<Symbol>) -> Production {
    Production {
        lhs: NonTerminal(name.to_string()),
        rhs,
    }
}

pub fn compute_epsilon_nonterminals(productions: &[Production]) -> BTreeSet<NonTerminal> {
    let mut epsilon_nonterminals: BTreeSet<NonTerminal> = BTreeSet::new();
    let mut changed = true;

    while changed {
        changed = false;
        for production in productions {
            if production.rhs.is_empty() && !epsilon_nonterminals.contains(&production.lhs) {
                epsilon_nonterminals.insert(production.lhs.clone());
                changed = true;
            } else if production.rhs.iter().all(|symbol| {
                if let Symbol::NonTerminal(nt) = symbol {
                    epsilon_nonterminals.contains(nt)
                } else {
                    false
                }
            }) && !epsilon_nonterminals.contains(&production.lhs)
            {
                epsilon_nonterminals.insert(production.lhs.clone());
                changed = true;
            }
        }
    }

    epsilon_nonterminals
}

pub fn compute_first_sets(productions: &[Production]) -> BTreeMap<NonTerminal, BTreeSet<Terminal>> {
    let mut first_sets: BTreeMap<NonTerminal, BTreeSet<Terminal>> = BTreeMap::new();

    // Initialize first sets
    for production in productions {
        let lhs = &production.lhs;
        if !first_sets.contains_key(lhs) {
            first_sets.insert(lhs.clone(), BTreeSet::new());
        }
        if let Some(Symbol::NonTerminal(nt)) = &production.rhs.get(0) {
            first_sets.get_mut(lhs).unwrap().insert(Terminal(nt.0.clone()));
        }
    }

    let mut changed = true;

    while changed {
        changed = false;

        for production in productions {
            let lhs = &production.lhs;
            let rhs = &production.rhs;

            let old_size = first_sets.get_mut(lhs).unwrap().len();

            let first_rhs = &rhs[0];

            if let Symbol::NonTerminal(nt) = first_rhs {
                let first_nt = first_sets[nt].clone();
                first_sets.get_mut(lhs).unwrap().extend(first_nt);
            }

            if first_sets.get_mut(lhs).unwrap().len() != old_size {
                changed = true;
            }
        }
    }

    first_sets
}

pub fn compute_follow_sets(
    productions: &[Production],
) -> BTreeMap<NonTerminal, BTreeSet<Terminal>> {
    let first_sets = compute_first_sets(productions);
    let epsilon_nonterminals = compute_epsilon_nonterminals(productions);
    let mut follow_sets: BTreeMap<NonTerminal, BTreeSet<Terminal>> = BTreeMap::new();

    // Initialize follow sets
    for production in productions {
        let lhs = &production.lhs;
        follow_sets.entry(lhs.clone()).or_default();
    }

    // Add EOF marker to the start symbol
    if let Some(start_symbol) = productions.get(0) {
        follow_sets
            .get_mut(&start_symbol.lhs)
            .unwrap()
            .insert(Terminal("$".to_string()));
    }

    let mut changed = true;

    while changed {
        changed = false;

        for production in productions {
            let lhs = &production.lhs;
            let rhs = &production.rhs;

            for (i, symbol) in rhs.iter().enumerate() {
                if let Symbol::NonTerminal(nt) = symbol {
                    let old_size = follow_sets.get_mut(nt).unwrap().len();

                    let mut j = i + 1;
                    loop {
                        if j < rhs.len() {
                            let next_symbol = &rhs[j];
                            match next_symbol {
                                Symbol::Terminal(t_next) => {
                                    follow_sets.get_mut(nt).unwrap().insert(t_next.clone());
                                    break;
                                }
                                Symbol::NonTerminal(nt_next) => {
                                    let first_next = &first_sets[nt_next];
                                    follow_sets.get_mut(nt).unwrap().extend(first_next.clone());
                                    if epsilon_nonterminals.contains(nt_next) {
                                        j += 1;
                                    } else {
                                        break;
                                    }
                                }
                            }
                        } else {
                            // Last symbol in the production
                            let follow_lhs = follow_sets.get(lhs).unwrap().clone();
                            follow_sets.get_mut(nt).unwrap().extend(follow_lhs);
                            break;
                        }
                    }

                    if follow_sets.get_mut(nt).unwrap().len() != old_size {
                        changed = true;
                    }
                }
            }
        }
    }

    follow_sets
}
// src/glr/grammar.rs