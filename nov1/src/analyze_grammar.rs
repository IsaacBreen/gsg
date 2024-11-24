use std::collections::{BTreeMap, BTreeSet};
use crate::glr::grammar::{prod, NonTerminal, Production, Symbol};

pub fn validate(productions: &[Production]) -> Result<(), String> {
    // Ensure all nonterminals have a productions
    let mut lhs_nonterms: BTreeSet<NonTerminal> = BTreeSet::new();
    let mut rhs_nonterms: BTreeSet<NonTerminal> = BTreeSet::new();

    for prod in productions {
        lhs_nonterms.insert(prod.lhs.clone());
        for symbol in &prod.rhs {
            if let Symbol::NonTerminal(nt) = symbol {
                rhs_nonterms.insert(nt.clone());
            }
        }
    }

    let missing_nonterms: BTreeSet<_> = rhs_nonterms.difference(&lhs_nonterms).collect();
    if !missing_nonterms.is_empty() {
        let missing_nonterm_strings: BTreeSet<_> = missing_nonterms.into_iter().map(|nt| nt.0.clone()).collect();
        return Err(format!("Nonterminals missing a production: {:?}", missing_nonterm_strings));
    }

    Ok(())
}

pub fn drop_dead(productions: &[Production]) -> Vec<Production> {
    // Ensure all nonterminals have a productions
    let mut nt_reachables: BTreeMap<&NonTerminal, BTreeSet<&NonTerminal>> = BTreeMap::new();

    for prod in productions {
        let mut rhs_nonterms = BTreeSet::new();
        for symbol in &prod.rhs {
            if let Symbol::NonTerminal(nt) = symbol {
                rhs_nonterms.insert(nt);
            }
        }
        nt_reachables.insert(&prod.lhs, rhs_nonterms);
    }

    loop {
        let mut changed = false;
        for (nt, mut reachables) in nt_reachables.clone() {
            let old_len = reachables.len();
            for reachable in reachables.clone() {
                if let Some(reachable_reachables) = nt_reachables.get(reachable) {
                    reachables.extend(reachable_reachables);
                }
            }
            if reachables.len() != old_len {
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let start_prod = &productions[0];
    let mut reachable_from_start = BTreeSet::new();
    for symbol in &start_prod.rhs {
        if let Symbol::NonTerminal(nt) = symbol {
            reachable_from_start.insert(nt);
        }
    }

    let mut new_productions = vec![productions[0].clone()];
    for prod in productions.iter().skip(1) {
        if reachable_from_start.contains(&prod.lhs) {
            new_productions.push(prod.clone())
        } else {
            crate::dbgprintln2!("Removing production {:?}", prod);
        }
    }

    new_productions
}