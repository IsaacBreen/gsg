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

fn closure(items: &BTreeSet<Item>, productions: &[Production]) -> BTreeSet<Item> {
    let mut closure_set = items.clone();
    let mut changed = true;

    while changed {
        changed = false;
        let mut new_items = BTreeSet::new();

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

fn goto(items: &BTreeSet<Item>, symbol: &Symbol, productions: &[Production]) -> BTreeSet<Item> {
    let mut goto_set = BTreeSet::new();

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
}

fn generate_parse_table(productions: &[Production], start_symbol: &str) -> HashMap<BTreeSet<Item>, HashMap<Symbol, BTreeSet<(Action, BTreeSet<Item>)>>> {
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

    let initial_state = closure(&BTreeSet::from([initial_item]), productions);
    states.push(initial_state);

    let mut i = 0;
    while i < states.len() {
        let current_state = states[i].clone();
        let mut transitions = HashMap::new();

        for item in &current_state { // Corrected: item is now in scope
            if item.dot_position < item.production.rhs.len() {
                let symbol = &item.production.rhs[item.dot_position];
                let next_state = goto(&current_state, symbol, productions);
                if !next_state.is_empty() {
                    let index = states.iter().position(|s| *s == next_state);
                    let index = match index {
                        Some(i) => i,
                        None => {
                            states.push(next_state.clone());
                            states.len() - 1
                        }
                    };

                    let action = if item.dot_position == item.production.rhs.len() {
                        if item.production.lhs == Symbol::NonTerminal(start_symbol.to_string()) {
                            Action::Accept
                        } else {
                            let production_index = productions.iter().position(|p| *p == item.production).unwrap();
                            Action::Reduce(production_index)
                        }
                    } else if let Symbol::Terminal(_) = symbol {
                        Action::Shift(index)
                    } else {
                        panic!("Invalid action");
                    };

                    transitions.entry(symbol.clone()).or_insert(BTreeSet::new()).insert((action, states[index].clone()));
                }
            }
        }
        parse_table.insert(current_state.clone(), transitions);
        i += 1;
    }

    parse_table
}

fn parse(input: &str, parse_table: &HashMap<BTreeSet<Item>, HashMap<Symbol, BTreeSet<(Action, BTreeSet<Item>)>>>, productions: &[Production], start_symbol: &str) -> Result<(), String> {
    let mut stack: VecDeque<(BTreeSet<Item>, Option<char>)> = VecDeque::new();
    let mut input_queue: VecDeque<char> = input.chars().collect();
    input_queue.push_back('$');

    let start_state = parse_table.keys().find(|state| state.iter().any(|item| item.production.lhs == Symbol::NonTerminal(start_symbol.to_string()) && item.dot_position == 0)).unwrap().clone();
    stack.push_back((start_state, None));

    while let Some((current_state, _)) = stack.back() {
        let lookahead = input_queue.front().unwrap().clone();

        if let Some(transitions) = parse_table.get(current_state) {
            if let Some(actions) = transitions.get(&Symbol::Terminal(lookahead)) {
                for (action, next_state) in actions {
                    match action {
                        Action::Shift(_) => {
                            stack.push_back((next_state.clone(), Some(lookahead)));
                            input_queue.pop_front();
                        }
                        Action::Reduce(production_index) => {
                            let production = &productions[*production_index];
                            for _ in 0..production.rhs.len() {
                                stack.pop_back();
                            }
                            let (top_state, _) = stack.back().unwrap();
                            let next_transitions = parse_table.get(top_state).unwrap();
                            if let Some(next_actions) = next_transitions.get(&production.lhs) {
                                for (next_action, next_next_state) in next_actions {
                                    if let Action::Shift(_) = next_action {
                                        stack.push_back((next_next_state.clone(), None));
                                    }
                                }
                            }
                        }
                        Action::Accept => {
                            return Ok(());
                        }
                    }
                }
            } else {
                return Err(format!("Parse error: Unexpected input '{}'", lookahead));
            }
        } else {
            return Err("Parse error: Invalid state".to_string());
        }
    }

    Err("Parse error: Incomplete parse".to_string())
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

    let result = parse("i+i*i", &parse_table, &productions, "S");
    match result {
        Ok(_) => println!("Parse successful!"),
        Err(msg) => println!("Parse error: {}", msg),
    }
}

#[cfg(test)]
mod glalr_tests {
    use super::*;

    #[test]
    fn test_parse_simple_expression() {
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

        assert!(parse("i+i*i", &parse_table, &productions, "S").is_ok());
        assert!(parse("i+i", &parse_table, &productions, "S").is_ok());
        assert!(parse("i*i", &parse_table, &productions, "S").is_ok());
        assert!(parse("i", &parse_table, &productions, "S").is_ok());
        assert!(parse("(i+i)*i", &parse_table, &productions, "S").is_ok());

        assert!(parse("i+", &parse_table, &productions, "S").is_err());
        assert!(parse("i++i", &parse_table, &productions, "S").is_err());
        assert!(parse("", &parse_table, &productions, "S").is_err());
        assert!(parse(")", &parse_table, &productions, "S").is_err());
    }
}