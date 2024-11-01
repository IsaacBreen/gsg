use std::collections::{BTreeSet, HashMap};
use crate::glr::grammar::{Production, Symbol};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item {
    pub production: Production,
    pub dot_position: usize,
}

pub fn compute_closure(items: &BTreeSet<Item>, productions: &[Production]) -> BTreeSet<Item> {
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
    closure
}

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

pub fn split_on_dot(items: &BTreeSet<Item>) -> HashMap<Option<Symbol>, BTreeSet<Item>> {
    let mut result: HashMap<Option<Symbol>, BTreeSet<Item>> = HashMap::new();
    for item in items {
        result
            .entry(item.production.rhs.get(item.dot_position).cloned())
            .or_default()
            .insert(item.clone());
    }
    result
}