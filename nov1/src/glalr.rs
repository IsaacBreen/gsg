use std::collections::{BTreeSet, HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Symbol {
    Terminal(char),
    NonTerminal(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Production {
    lhs: Symbol,
    rhs: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    production: Production,
    dot_position: usize,
}

fn compute_closure(items: &BTreeSet<Item>, productions: &[Production]) -> BTreeSet<Item> {
    let mut closure = items.clone();
    let mut changed = true;

    // Keep adding items until no new ones can be added
    while changed {
        changed = false;
        let current_items = closure.clone();

        for item in current_items {
            // Get the symbol after the dot
            if let Some(symbol) = item.production.rhs.get(item.dot_position) {
                // If it's a non-terminal, add all productions with that non-terminal on LHS
                if let Symbol::NonTerminal(name) = symbol {
                    for production in productions {
                        if let Symbol::NonTerminal(prod_name) = &production.lhs {
                            if prod_name == name {
                                let new_item = Item {
                                    production: production.clone(),
                                    dot_position: 0,
                                };
                                if !closure.contains(&new_item) {
                                    closure.insert(new_item);
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    closure
}

fn compute_goto(items: &BTreeSet<Item>) -> BTreeSet<Item> {
    let mut result = BTreeSet::new();
    for item in items {
        result.insert(Item {
            production: item.production.clone(),
            dot_position: item.dot_position + 1,
        });
    }
    result
}

fn split_on_dot(items: &BTreeSet<Item>) -> HashMap<Option<Symbol>, BTreeSet<Item>> {
    let mut result: HashMap<Option<Symbol>, BTreeSet<Item>> = HashMap::new();
    for item in items {
        result
            .entry(item.production.rhs.get(item.dot_position).cloned())
            .or_default()
            .insert(item.clone());
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
}

fn generate_transitions(productions: &[Production]) -> HashMap<BTreeSet<Item>, HashMap<Option<Symbol>, BTreeSet<Item>>> {
    let mut worklist = VecDeque::from([BTreeSet::from([Item {
        production: productions[0].clone(),
        dot_position: 0,
    }])]);

    let mut transitions: HashMap<BTreeSet<Item>, HashMap<Option<Symbol>, BTreeSet<Item>>> = HashMap::new();

    while let Some(items) = worklist.pop_front() {
        if transitions.contains_key(&items) {
            // Already processed
            continue;
        }

        transitions.insert(items.clone(), HashMap::new());

        let closure = compute_closure(&items, productions);

        for (maybe_symbol, items) in split_on_dot(&closure) {
            transitions.get_mut(&items).unwrap().insert(maybe_symbol, compute_goto(&items));

            worklist.push_back(items);
        }
    }

    transitions
}

fn compute_firsts(productions: &[Production]) -> HashMap<Symbol, BTreeSet<Symbol>> {
    let mut firsts: HashMap<Symbol, BTreeSet<Symbol>> = HashMap::new();
    let mut changed = true;

    // Initialize firsts sets
    for production in productions {
        firsts.entry(production.lhs.clone()).or_default();
    }

    // Keep iterating until no changes are made
    while changed {
        changed = false;

        for production in productions {
            let lhs = &production.lhs;

            // For each production A → β, add first(β) to first(A)
            if let Some(first_symbol) = production.rhs.first() {
                match first_symbol {
                    // If it's a terminal, add it directly
                    Symbol::Terminal(_) => {
                        let first_set = firsts.get_mut(lhs).unwrap();
                        if first_set.insert(first_symbol.clone()) {
                            changed = true;
                        }
                    }
                    // If it's a non-terminal, add all its firsts
                    Symbol::NonTerminal(_) => {
                        let first_symbols: Vec<_> = firsts
                            .get(first_symbol)
                            .map(|set| set.iter().cloned().collect())
                            .unwrap_or_default();

                        let first_set = firsts.get_mut(lhs).unwrap();
                        for symbol in first_symbols {
                            if first_set.insert(symbol) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }

    firsts
}

fn compute_lasts(productions: &[Production]) -> HashMap<Symbol, BTreeSet<Symbol>> {
    let mut lasts: HashMap<Symbol, BTreeSet<Symbol>> = HashMap::new();
    let mut changed = true;

    // Initialize lasts sets
    for production in productions {
        lasts.entry(production.lhs.clone()).or_default();
    }

    // Keep iterating until no changes are made
    while changed {
        changed = false;

        for production in productions {
            let lhs = &production.lhs;

            // For each production A → β, add last(β) to last(A)
            if let Some(last_symbol) = production.rhs.last() {
                match last_symbol {
                    // If it's a terminal, add it directly
                    Symbol::Terminal(_) => {
                        let last_set = lasts.get_mut(lhs).unwrap();
                        if last_set.insert(last_symbol.clone()) {
                            changed = true;
                        }
                    }
                    // If it's a non-terminal, add all its lasts
                    Symbol::NonTerminal(_) => {
                        let last_symbols: Vec<_> = lasts
                            .get(last_symbol)
                            .map(|set| set.iter().cloned().collect())
                            .unwrap_or_default();

                        let last_set = lasts.get_mut(lhs).unwrap();
                        for symbol in last_symbols {
                            if last_set.insert(symbol) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }

    lasts
}

// fn parse(input: &str, parse_table: &HashMap<BTreeSet<Item>, HashMap<Option<Symbol>, BTreeSet<(Action, BTreeSet<Item>)>>>, productions: &[Production], start_symbol: &str) -> Result<(), String> {
//     todo!()
// }

fn nt(name: &str) -> Symbol {
    Symbol::NonTerminal(name.to_string())
}

fn term(c: char) -> Symbol {
    Symbol::Terminal(c)
}

fn prod(name: &str, rhs: Vec<Symbol>) -> Production {
    Production {
        lhs: nt(name),
        rhs,
    }
}

#[cfg(test)]
mod glalr_tests {
    use super::*;

    #[test]
    fn test_parse_simple_expression() {
        let productions = vec![
            // E -> E + T
            prod("E", vec![nt("E"), term('+'), nt("T")]),
            // E -> T
            prod("E", vec![nt("T")]),
            // T -> T * F
            prod("T", vec![nt("T"), term('*'), nt("F")]),
            // T -> F
            prod("T", vec![nt("F")]),
            // F -> ( E )
            prod("F", vec![term('('), nt("E"), term(')')]),
            // F -> i
            prod("F", vec![term('i')]),
        ];

    //     let parse_table = generate_parse_table(&productions, "S");
    //
    //     assert!(parse("i+i*i", &parse_table, &productions, "S").is_ok());
    //     assert!(parse("i+i", &parse_table, &productions, "S").is_ok());
    //     assert!(parse("i*i", &parse_table, &productions, "S").is_ok());
    //     assert!(parse("i", &parse_table, &productions, "S").is_ok());
    //     assert!(parse("(i+i)*i", &parse_table, &productions, "S").is_ok());
    //
    //     assert!(parse("i+", &parse_table, &productions, "S").is_err());
    //     assert!(parse("i++i", &parse_table, &productions, "S").is_err());
    //     assert!(parse("", &parse_table, &productions, "S").is_err());
    //     assert!(parse(")", &parse_table, &productions, "S").is_err());
    }
}