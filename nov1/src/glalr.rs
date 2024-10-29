use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Symbol {
    Terminal(char),
    NonTerminal(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Production {
    lhs: Symbol,
    rhs: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Item {
    production: Production,
    dot_position: usize,
}

fn closure(items: &HashSet<Item>, productions: &[Production]) -> HashSet<Item> {
    let mut closure_set = items.clone();
    let mut changed = true;

    while changed {
        changed = false;
        let mut new_items = HashSet::new();

        for item in &closure_set {
            if item.dot_position < item.production.rhs.len() {
                if let Symbol::NonTerminal(nt) = &item.production.rhs[item.dot_position] {
                    for production in productions {
                        if production.lhs == Symbol::NonTerminal(nt.clone()) {
                            new_items.insert(Item {
                                production: production.clone(),
                                dot_position: 0,
                            });
                        }
                    }
                }
            }
        }

        if !new_items.is_subset(&closure_set) {
            changed = true;
            closure_set.extend(new_items);
        }
    }

    closure_set
}

fn goto(items: &HashSet<Item>, symbol: &Symbol, productions: &[Production]) -> HashSet<Item> {
    let mut goto_set = HashSet::new();

    for item in items {
        if item.dot_position < item.production.rhs.len()
            && &item.production.rhs[item.dot_position] == symbol
        {
            goto_set.insert(Item {
                production: item.production.clone(),
                dot_position: item.dot_position + 1,
            });
        }
    }

    closure(&goto_set, productions)
}

fn generate_parse_table(productions: &[Production], start_symbol: &str) -> HashMap<HashSet<Item>, HashMap<Symbol, HashSet<HashSet<Item>>>> {
    let mut parse_table = HashMap::new();
    let mut states = Vec::new();

    let start_production = productions
        .iter()
        .find(|p| p.lhs == Symbol::NonTerminal(start_symbol.to_string()))
        .unwrap()
        .clone();

    let initial_item = Item {
        production: start_production,
        dot_position: 0,
    };

    let initial_state = closure(&HashSet::from([initial_item]), productions);
    states.push(initial_state);

    let mut i = 0;
    while i < states.len() {
        let current_state = &states[i];
        let mut transitions = HashMap::new();

        for symbol in current_state.iter().flat_map(|item| {
            if item.dot_position < item.production.rhs.len() {
                Some(&item.production.rhs[item.dot_position])
            } else {
                None
            }
        }) {
            let next_state = goto(current_state, symbol, productions);
            if !next_state.is_empty() {
                let index = states.iter().position(|s| *s == next_state);
                let index = match index {
                    Some(i) => i,
                    None => {
                        states.push(next_state.clone());
                        states.len() - 1
                    }
                };

                transitions.entry(symbol.clone()).or_insert(HashSet::new()).insert(states[index].clone());
            }
        }

        parse_table.insert(current_state.clone(), transitions);
        i += 1;
    }

    parse_table
}


fn main() {
    let productions = vec![
        Production { lhs: Symbol::NonTerminal("S".to_string()), rhs: vec![Symbol::NonTerminal("E".to_string())] },
        Production { lhs: Symbol::NonTerminal("E".to_string()), rhs: vec![Symbol::NonTerminal("E".to_string()), Symbol::Terminal('+'), Symbol::NonTerminal("T".to_string())] },
        Production { lhs: Symbol::NonTerminal("E".to_string()), rhs: vec![Symbol::NonTerminal("T".to_string())] },
        Production { lhs: Symbol::NonTerminal("T".to_string()), rhs: vec![Symbol::NonTerminal("T".to_string()), Symbol::Terminal('*'), Symbol::NonTerminal("F".to_string())] },
        Production { lhs: Symbol::NonTerminal("T".to_string()), rhs: vec![Symbol::NonTerminal("F".to_string())] },
        Production { lhs: Symbol::NonTerminal("F".to_string()), rhs: vec![Symbol::Terminal('('), Symbol::NonTerminal("E".to_string()), Symbol::Terminal(')')] },
        Production { lhs: Symbol::NonTerminal("F".to_string()), rhs: vec![Symbol::Terminal('i')] },
    ];

    let parse_table = generate_parse_table(&productions, "S");

    println!("{:#?}", parse_table);
}