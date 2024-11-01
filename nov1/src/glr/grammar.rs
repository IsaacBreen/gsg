use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonTerminal(pub String);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Terminal(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    Production { lhs: NonTerminal(name.to_string()), rhs }
}


pub fn compute_first_sets(productions: &[Production]) -> HashMap<NonTerminal, HashSet<Terminal>> {
    let mut first_sets: HashMap<NonTerminal, HashSet<Terminal>> = HashMap::new();

    // Initialize first sets
    for production in productions {
        let lhs = &production.lhs;
        if !first_sets.contains_key(lhs) {
            first_sets.insert(lhs.clone(), HashSet::new());
        }
        if let Symbol::Terminal(t) = &production.rhs[0] {
            first_sets.get_mut(lhs).unwrap().insert(t.clone());
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
    first_sets: &HashMap<NonTerminal, HashSet<Terminal>>,
) -> HashMap<NonTerminal, HashSet<Terminal>> {
    let mut follow_sets: HashMap<NonTerminal, HashSet<Terminal>> = HashMap::new();

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

                    if i + 1 < rhs.len() {
                        let next_symbol = &rhs[i + 1];
                        match next_symbol {
                            Symbol::Terminal(t_next) => {
                                follow_sets.get_mut(nt).unwrap().insert(t_next.clone());
                            }
                            Symbol::NonTerminal(nt_next) => {
                                let first_next = &first_sets[nt_next];
                                follow_sets.get_mut(nt).unwrap().extend(first_next.clone());
                            }
                        }
                    } else {
                        // Last symbol in the production
                        let follow_lhs = follow_sets.get(lhs).unwrap().clone();
                        follow_sets.get_mut(nt).unwrap().extend(follow_lhs);
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