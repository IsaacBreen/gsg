use crate::glr::grammar::{Production, Symbol};
use std::collections::{BTreeMap, BTreeSet};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Item {
    pub production: Production,
    pub dot_position: usize,
}

pub fn compute_closure(items: &BTreeSet<Item>, productions: &[Production]) -> BTreeSet<Item> {
    crate::dbgprintln!("Computing closure");
    let mut closure = items.clone();
    let mut added = true;
    while added {
        added = false;
        let mut new_items = BTreeSet::new();
        for item in &closure {
            if let Some(Symbol::NonTerminal(nt)) = item.production.rhs.get(item.dot_position) {
                for prod in productions.iter().filter(|p| p.lhs == nt.clone()) {
                    let new_item = Item {
                        production: prod.clone(),
                        dot_position: 0,
                    };
                    if !closure.contains(&new_item) {
                        new_items.insert(new_item);
                        added = true;
                    }
                }
            }
        }
        closure.extend(new_items);
    }
    crate::dbgprintln!("Done computing closure");
    closure
}

/// Computes the GOTO function for a set of LR(0) items.
pub fn compute_goto(items: &BTreeSet<Item>) -> BTreeSet<Item> {
    let mut result = BTreeSet::new();
    for item in items {
        if item.dot_position < item.production.rhs.len() {
            result.insert(Item {
                production: item.production.clone(),
                dot_position: item.dot_position + 1,
            });
        }
    }
    result
}

/// Splits a set of LR(0) items based on the symbol after the dot.
pub fn split_on_dot(items: &BTreeSet<Item>) -> BTreeMap<Option<Symbol>, BTreeSet<Item>> {
    let mut result: BTreeMap<Option<Symbol>, BTreeSet<Item>> = BTreeMap::new();
    for item in items {
        result
            .entry(item.production.rhs.get(item.dot_position).cloned())
            .or_default()
            .insert(item.clone());
    }
    result
}
// src/glr/items.rs