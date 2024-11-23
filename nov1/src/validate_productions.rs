use std::collections::BTreeSet;
use crate::glr::grammar::{NonTerminal, Production, Symbol};

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