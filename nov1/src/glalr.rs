use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use bimap::BiMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NonTerminal(String);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Terminal(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StateID(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ProductionID(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NonTerminalID(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TerminalID(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Symbol {
    Terminal(Terminal),
    NonTerminal(NonTerminal),
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
    let mut added = true;
    while added {
        added = false;
        let mut new_items = BTreeSet::new();
        for item in &closure {
            if let Some(Symbol::NonTerminal(nt)) = item.production.rhs.get(item.dot_position) {
                for prod in productions.iter().filter(|p| p.lhs == Symbol::NonTerminal(nt.clone())) {
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

fn compute_goto(items: &BTreeSet<Item>) -> BTreeSet<Item> {
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

type Stage1Table = HashMap<BTreeSet<Item>, Stage1Row>;
type Stage2Table = HashMap<BTreeSet<Item>, Stage2Row>;
type Stage3Table = HashMap<BTreeSet<Item>, Stage3Row>;
type Stage4Table = HashMap<BTreeSet<Item>, Stage4Row>;
type Stage5Table = HashMap<BTreeSet<Item>, Stage5Row>;
type Stage6Table = HashMap<StateID, Stage6Row>;

type Stage1Row = HashMap<Option<Symbol>, BTreeSet<Item>>;
struct Stage2Row {
    shifts: HashMap<Terminal, BTreeSet<Item>>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
    reduces: BTreeSet<Item>,
}
struct Stage3Row {
    shifts: HashMap<Terminal, BTreeSet<Item>>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
    /// Split the reduce items by lookahead.
    /// For LR(0), all possible terminals map to the entire reduce item set.
    /// But there are various cleverer and more selective ways to compute lookaheads.
    /// For simplicity, use LALR.
    reduces: HashMap<Terminal, BTreeSet<Item>>,
}
struct Stage4Row {
    shifts: HashMap<Terminal, BTreeSet<Item>>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
    /// Each item in the reduce has a dot at the end, so throw away the dot and just store the production ID.
    reduces: HashMap<Terminal, BTreeSet<ProductionID>>,
}
struct Stage5Row {
    shifts: HashMap<Terminal, BTreeSet<Item>>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
    /// The `usize` here is the length of the production, i.e. the number of items to pop off the stack during reduction.
    reduces: HashMap<Terminal, BTreeMap<usize, BTreeSet<NonTerminal>>>,
}
struct Stage6Row {
    /// Map each item set to a unique ID, and do the same for terminals and nonterminals.
    shifts: HashMap<TerminalID, StateID>,
    gotos: HashMap<NonTerminalID, StateID>,
    reduces: HashMap<TerminalID, BTreeMap<usize, BTreeSet<NonTerminalID>>>,
}

type Stage1Result = Stage1Table;
type Stage2Result = Stage2Table;
type Stage3Result = Stage3Table;
type Stage4Result = Stage4Table;
type Stage5Result = Stage5Table;
type Stage6Result = (
    Stage6Table,
    BiMap<Terminal, TerminalID>,
    BiMap<NonTerminal, NonTerminalID>,
    BiMap<BTreeSet<Item>, StateID>,
);

fn stage_1(productions: &[Production]) -> Stage1Result {
    let initial_item = Item {
        production: productions[0].clone(),
        dot_position: 0,
    };
    let initial_closure = compute_closure(&BTreeSet::from([initial_item]), productions);
    let mut worklist = VecDeque::from([initial_closure.clone()]);

    let mut transitions: HashMap<BTreeSet<Item>, HashMap<Option<Symbol>, BTreeSet<Item>>> = HashMap::new();

    while let Some(items) = worklist.pop_front() {
        if transitions.contains_key(&items) {
            continue;
        }

        let closure = compute_closure(&items, productions);
        let splits = split_on_dot(&closure);
        let mut row = HashMap::new();

        for (symbol, items) in splits {
            if symbol.is_none() {
                continue;
            }
            let goto_set = compute_goto(&items);
            let goto_closure = compute_closure(&goto_set, productions);
            row.insert(symbol.clone(), goto_closure.clone());
            worklist.push_back(goto_closure);
        }

        transitions.insert(items.clone(), row);
    }

    transitions
}

fn stage_2(stage_1_table: Stage1Table) -> Stage2Result {
    let mut stage_2_table = HashMap::new();
    for (item_set, transitions) in stage_1_table {
        let mut shifts = HashMap::new();
        let mut gotos = HashMap::new();
        let mut reduces = BTreeSet::new();

        for item in &item_set {
            if item.dot_position >= item.production.rhs.len() {
                // Reduce item
                reduces.insert(item.clone());
            }
        }

        for (symbol_opt, next_item_set) in &transitions {
            if let Some(symbol) = symbol_opt {
                match symbol {
                    Symbol::Terminal(t) => {
                        shifts.insert(t.clone(), next_item_set.clone());
                    }
                    Symbol::NonTerminal(nt) => {
                        gotos.insert(nt.clone(), next_item_set.clone());
                    }
                }
            }
        }

        stage_2_table.insert(
            item_set,
            Stage2Row {
                shifts,
                gotos,
                reduces,
            },
        );
    }
    stage_2_table
}

use std::collections::HashSet;

fn compute_first_sets(productions: &[Production]) -> HashMap<Symbol, HashSet<Terminal>> {
    let mut first_sets: HashMap<Symbol, HashSet<Terminal>> = HashMap::new();

    // Initialize first sets
    for production in productions {
        let lhs = &production.lhs;
        if !first_sets.contains_key(lhs) {
            first_sets.insert(lhs.clone(), HashSet::new());
        }
        for symbol in &production.rhs {
            match symbol {
                Symbol::Terminal(_) => {
                    if !first_sets.contains_key(symbol) {
                        let mut set = HashSet::new();
                        if let Symbol::Terminal(t) = symbol {
                            set.insert(t.clone());
                        }
                        first_sets.insert(symbol.clone(), set);
                    }
                }
                Symbol::NonTerminal(_) => {
                    first_sets.entry(symbol.clone()).or_default();
                }
            }
        }
    }

    let mut changed = true;

    while changed {
        changed = false;

        for production in productions {
            let lhs = &production.lhs;
            let rhs = &production.rhs;

            let old_size = first_sets.get_mut(lhs).unwrap().len();

            for symbol in rhs {
                let symbol_first_set = first_sets.get(symbol).unwrap().clone();
                first_sets.get_mut(lhs).unwrap().extend(symbol_first_set);

                // For LR(0), we can assume no epsilon productions
                break;
            }

            if first_sets.get_mut(lhs).unwrap().len() != old_size {
                changed = true;
            }
        }
    }

    first_sets
}

fn compute_follow_sets(
    productions: &[Production],
    first_sets: &HashMap<Symbol, HashSet<Terminal>>,
) -> HashMap<Symbol, HashSet<Terminal>> {
    let mut follow_sets: HashMap<Symbol, HashSet<Terminal>> = HashMap::new();

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
                if let Symbol::NonTerminal(_) = symbol {
                    let old_size = follow_sets.get_mut(symbol).unwrap().len();

                    if i + 1 < rhs.len() {
                        let next_symbol = &rhs[i + 1];
                        match next_symbol {
                            Symbol::Terminal(t) => {
                                follow_sets.get_mut(symbol).unwrap().insert(t.clone());
                            }
                            Symbol::NonTerminal(_) => {
                                let first_next = &first_sets[next_symbol];
                                follow_sets.get_mut(symbol).unwrap().extend(first_next.clone());
                            }
                        }
                    } else {
                        // Last symbol in the production
                        let follow_lhs = follow_sets.get(lhs).unwrap().clone();
                        follow_sets.get_mut(symbol).unwrap().extend(follow_lhs);
                    }

                    if follow_sets.get_mut(symbol).unwrap().len() != old_size {
                        changed = true;
                    }
                }
            }
        }
    }

    follow_sets
}

fn stage_3(stage_2_table: Stage2Table, productions: &[Production]) -> Stage3Result {
    let first_sets = compute_first_sets(productions);
    let follow_sets = compute_follow_sets(productions, &first_sets);

    let mut stage_3_table = HashMap::new();

    for (item_set, row) in stage_2_table {
        let mut reduces: HashMap<Terminal, BTreeSet<Item>> = HashMap::new();

        for item in &row.reduces {
            let lhs = &item.production.lhs;
            let lookaheads = follow_sets.get(lhs).unwrap();

            for terminal in lookaheads {
                reduces
                    .entry(terminal.clone())
                    .or_default()
                    .insert(item.clone());
            }
        }

        stage_3_table.insert(
            item_set,
            Stage3Row {
                shifts: row.shifts,
                gotos: row.gotos,
                reduces,
            },
        );
    }

    stage_3_table
}

fn stage_4(stage_3_table: Stage3Table, productions: &[Production]) -> Stage4Result {
    let production_ids: HashMap<Production, ProductionID> = productions
        .iter()
        .enumerate()
        .map(|(i, p)| (p.clone(), ProductionID(i)))
        .collect();

    let mut stage_4_table = HashMap::new();

    for (item_set, row) in stage_3_table {
        let mut reduces = HashMap::new();

        for (terminal, items) in row.reduces {
            let mut prod_ids = BTreeSet::new();
            for item in items {
                let prod_id = production_ids.get(&item.production).unwrap();
                prod_ids.insert(*prod_id);
            }
            reduces.insert(terminal.clone(), prod_ids);
        }

        stage_4_table.insert(
            item_set,
            Stage4Row {
                shifts: row.shifts,
                gotos: row.gotos,
                reduces,
            },
        );
    }

    stage_4_table
}

fn stage_5(stage_4_table: Stage4Table, productions: &[Production]) -> Stage5Result {
    let production_info: HashMap<ProductionID, (usize, NonTerminal)> = productions
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if let Symbol::NonTerminal(nt) = &p.lhs {
                Some((ProductionID(i), (p.rhs.len(), nt.clone())))
            } else {
                None
            }
        })
        .collect();

    let mut stage_5_table = HashMap::new();

    for (item_set, row) in stage_4_table {
        let mut reduces = HashMap::new();

        for (terminal, prod_ids) in row.reduces {
            let mut len_map: BTreeMap<usize, BTreeSet<NonTerminal>> = BTreeMap::new();
            for prod_id in prod_ids {
                let (len, nt) = &production_info[&prod_id];
                len_map.entry(*len).or_default().insert(nt.clone());
            }
            reduces.insert(terminal.clone(), len_map);
        }

        stage_5_table.insert(
            item_set,
            Stage5Row {
                shifts: row.shifts,
                gotos: row.gotos,
                reduces,
            },
        );
    }

    stage_5_table
}

fn stage_6(stage_5_table: Stage5Table) -> Stage6Result {
    let mut terminal_map = BiMap::new();
    let mut non_terminal_map = BiMap::new();
    let mut state_map = BiMap::new();
    let mut next_terminal_id = 0;
    let mut next_non_terminal_id = 0;
    let mut next_state_id = 0;

    // Collect all terminals, non-terminals, and states
    let mut terminals = BTreeSet::new();
    let mut non_terminals = BTreeSet::new();

    for (item_set, row) in &stage_5_table {
        state_map.insert(item_set.clone(), StateID(next_state_id));
        next_state_id += 1;

        for t in row.shifts.keys() {
            terminals.insert(t.clone());
        }

        for nt in row.gotos.keys() {
            non_terminals.insert(nt.clone());
        }

        for t in row.reduces.keys() {
            terminals.insert(t.clone());
        }
    }

    for t in terminals {
        terminal_map.insert(t.clone(), TerminalID(next_terminal_id));
        next_terminal_id += 1;
    }

    for nt in non_terminals {
        non_terminal_map.insert(nt.clone(), NonTerminalID(next_non_terminal_id));
        next_non_terminal_id += 1;
    }

    let mut stage_6_table = HashMap::new();

    for (item_set, row) in stage_5_table {
        let state_id = *state_map.get_by_left(&item_set).unwrap();

        let mut shifts = HashMap::new();
        for (t, next_item_set) in row.shifts {
            let terminal_id = *terminal_map.get_by_left(&t).unwrap();
            let next_state_id = *state_map.get_by_left(&next_item_set).unwrap();
            shifts.insert(terminal_id, next_state_id);
        }

        let mut gotos = HashMap::new();
        for (nt, next_item_set) in row.gotos {
            let non_terminal_id = *non_terminal_map.get_by_left(&nt).unwrap();
            let next_state_id = *state_map.get_by_left(&next_item_set).unwrap();
            gotos.insert(non_terminal_id, next_state_id);
        }

        let mut reduces = HashMap::new();
        for (t, len_map) in row.reduces {
            let terminal_id = *terminal_map.get_by_left(&t).unwrap();
            let mut len_nt_map = BTreeMap::new();

            for (len, nts) in len_map {
                let mut nt_ids = BTreeSet::new();
                for nt in nts {
                    let nt_id = *non_terminal_map.get_by_left(&nt).unwrap();
                    nt_ids.insert(nt_id);
                }
                len_nt_map.insert(len, nt_ids);
            }

            reduces.insert(terminal_id, len_nt_map);
        }

        stage_6_table.insert(
            state_id,
            Stage6Row {
                shifts,
                gotos,
                reduces,
            },
        );
    }

    (stage_6_table, terminal_map, non_terminal_map, state_map)
}

fn parse(
    input: &[TerminalID],
    stage_6_table: &Stage6Table,
    terminal_map: &BiMap<Terminal, TerminalID>,
    non_terminal_map: &BiMap<NonTerminal, NonTerminalID>,
) -> Vec<Vec<Symbol>> {
    let mut stack = vec![StateID(0)];
    let mut symbols_stack: Vec<Symbol> = Vec::new();
    let mut input_pos = 0;

    loop {
        let state_id = *stack.last().unwrap();
        let token = if input_pos < input.len() {
            input[input_pos]
        } else {
            // EOF TerminalID
            terminal_map.get_by_left(&Terminal("$".to_string())).cloned().unwrap()
        };

        let row = match stage_6_table.get(&state_id) {
            Some(row) => row,
            None => return vec![], // Error
        };

        if let Some(&next_state_id) = row.shifts.get(&token) {
            // Shift
            stack.push(next_state_id);
            let terminal = terminal_map.get_by_right(&token).unwrap().clone();
            symbols_stack.push(Symbol::Terminal(terminal));
            input_pos += 1;
        } else if let Some(reduces) = row.reduces.get(&token) {
            // Reduce
            let (&len, nt_ids) = reduces.iter().next().unwrap(); // Simplified: pick the first reduce action
            for _ in 0..len {
                stack.pop();
                symbols_stack.pop();
            }
            let nt_id = *nt_ids.iter().next().unwrap(); // Simplified
            let goto_state_id = match stage_6_table
                .get(stack.last().unwrap())
                .and_then(|row| row.gotos.get(&nt_id))
            {
                Some(&state) => state,
                None => return vec![], // Error
            };
            stack.push(goto_state_id);
            let non_terminal = non_terminal_map.get_by_right(&nt_id).unwrap().clone();
            symbols_stack.push(Symbol::NonTerminal(non_terminal));
        } else if stack.len() == 1 && input_pos == input.len() {
            // Accept
            return vec![symbols_stack];
        } else {
            // Error
            return vec![];
        }
    }
}

fn generate_parse_table(productions: &[Production]) -> Stage6Result {
    let stage_1_table = stage_1(productions);
    let stage_2_table = stage_2(stage_1_table);
    let stage_3_table = stage_3(stage_2_table, productions);
    let stage_4_table = stage_4(stage_3_table, productions);
    let stage_5_table = stage_5(stage_4_table, productions);
    stage_6(stage_5_table)
}

fn nt(name: &str) -> Symbol {
    Symbol::NonTerminal(NonTerminal(name.to_string()))
}

fn t(name: &str) -> Symbol {
    Symbol::Terminal(Terminal(name.to_string()))
}

fn prod(name: &str, rhs: Vec<Symbol>) -> Production {
    Production { lhs: nt(name), rhs }
}

#[cfg(test)]
mod glalr_tests {
    use super::*;

    #[test]
    fn test_parse_simple_expression() {
        let productions = vec![
            // S -> E
            prod("S", vec![nt("E")]),
            // E -> E + T
            prod("E", vec![nt("E"), t("+"), nt("T")]),
            // E -> T
            prod("E", vec![nt("T")]),
            // T -> T * F
            prod("T", vec![nt("T"), t("*"), nt("F")]),
            // T -> F
            prod("T", vec![nt("F")]),
            // F -> ( E )
            prod("F", vec![t("("), nt("E"), t(")")]),
            // F -> i
            prod("F", vec![t("i")]),
        ];

        let (parse_table, terminal_map, non_terminal_map, _) = generate_parse_table(&productions);

        let tokenize = |input: &str| -> Vec<TerminalID> {
            let mut result = Vec::new();
            for c in input.chars() {
                let terminal = Terminal(c.to_string());
                if let Some(id) = terminal_map.get_by_left(&terminal) {
                    result.push(*id);
                } else {
                    panic!("Unknown token: {}", c);
                }
            }
            result
        };

        assert!(!parse(
            &tokenize("i+i*i"),
            &parse_table,
            &terminal_map,
            &non_terminal_map
        )
        .is_empty());
        assert!(!parse(
            &tokenize("i+i"),
            &parse_table,
            &terminal_map,
            &non_terminal_map
        )
        .is_empty());
        assert!(!parse(
            &tokenize("i*i"),
            &parse_table,
            &terminal_map,
            &non_terminal_map
        )
        .is_empty());
        assert!(!parse(
            &tokenize("i"),
            &parse_table,
            &terminal_map,
            &non_terminal_map
        )
        .is_empty());
        assert!(!parse(
            &tokenize("(i+i)*i"),
            &parse_table,
            &terminal_map,
            &non_terminal_map
        )
        .is_empty());

        assert!(parse(
            &tokenize("i+"),
            &parse_table,
            &terminal_map,
            &non_terminal_map
        )
        .is_empty());
        assert!(parse(
            &tokenize("i++i"),
            &parse_table,
            &terminal_map,
            &non_terminal_map
        )
        .is_empty());
        assert!(parse(
            &tokenize(""),
            &parse_table,
            &terminal_map,
            &non_terminal_map
        )
        .is_empty());
        assert!(parse(
            &tokenize(")"),
            &parse_table,
            &terminal_map,
            &non_terminal_map
        )
        .is_empty());
    }
}