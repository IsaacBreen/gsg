use std::fmt::Write;
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
    lhs: NonTerminal,
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

/// Computes the GOTO function for a set of LR(0) items.
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

/// Splits a set of LR(0) items based on the symbol after the dot.
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
type Stage6Table = HashMap<BTreeSet<Item>, Stage6Row>;
type Stage7Table = HashMap<StateID, Stage7Row>;


type Stage1Row = HashMap<Option<Symbol>, BTreeSet<Item>>;
#[derive(Debug)]
struct Stage2Row {
    shifts: HashMap<Terminal, BTreeSet<Item>>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
    reduces: BTreeSet<Item>,
}
#[derive(Debug)]
struct Stage3Row {
    shifts: HashMap<Terminal, BTreeSet<Item>>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
    /// Split the reduce items by lookahead.
    /// For LR(0), all possible terminals map to the entire reduce item set.
    /// But there are various cleverer and more selective ways to compute lookaheads.
    /// For simplicity, use LALR.
    reduces: HashMap<Terminal, BTreeSet<Item>>,
}
#[derive(Debug)]
struct Stage4Row {
    shifts: HashMap<Terminal, BTreeSet<Item>>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
    /// Each item in the reduce has a dot at the end, so throw away the dot and just store the production ID.
    reduces: HashMap<Terminal, BTreeSet<ProductionID>>,
}
#[derive(Debug)]
struct Stage5Row {
    shifts: HashMap<Terminal, BTreeSet<Item>>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
    /// The `usize` here is the length of the production, i.e. the number of items to pop off the stack during reduction.
    reduces: HashMap<Terminal, BTreeMap<usize, BTreeSet<NonTerminal>>>,
}
#[derive(Debug)]
struct Stage6Row {
    shifts_and_reduces: HashMap<Terminal, Stage6ShiftsAndReduces>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
}

#[derive(Debug)]
enum Stage6ShiftsAndReduces {
    Shift(BTreeSet<Item>),
    Reduce { nonterminal: NonTerminal, len: usize },
    Split {
        shift: Option<BTreeSet<Item>>,
        reduces: BTreeMap<usize, BTreeSet<NonTerminal>>,
    },
}

#[derive(Debug)]
enum Stage7ShiftsAndReduces {
    /// Map each item set to a unique ID, and do the same for terminals and nonterminals.
    Shift(StateID),
    Reduce { nonterminal: NonTerminalID, len: usize },
    Split {
        shift: Option<StateID>,
        reduces: BTreeMap<usize, BTreeSet<NonTerminalID>>,
    },
}

#[derive(Debug)]
struct Stage7Row {
    /// Map each item set to a unique ID, and do the same for terminals and nonterminals.
    shifts_and_reduces: HashMap<TerminalID, Stage7ShiftsAndReduces>,
    gotos: HashMap<NonTerminalID, StateID>,
}

type Stage1Result = Stage1Table;
type Stage2Result = Stage2Table;
type Stage3Result = Stage3Table;
type Stage4Result = Stage4Table;
type Stage5Result = Stage5Table;
type Stage6Result = Stage6Table;
type Stage7Result = (
    Stage7Table,
    BiMap<Terminal, TerminalID>,
    BiMap<NonTerminal, NonTerminalID>,
    BiMap<BTreeSet<Item>, StateID>,
    StateID,
);

fn stage_1(productions: &[Production]) -> Stage1Result {
    let initial_item = Item {
        production: productions[0].clone(),
        dot_position: 0,
    };
    let initial_closure = BTreeSet::from([initial_item]);
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

fn stage_2(stage_1_table: Stage1Table, productions: &[Production]) -> Stage2Result {
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

fn compute_first_sets(productions: &[Production]) -> HashMap<NonTerminal, HashSet<Terminal>> {
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

fn compute_follow_sets(
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
        .map(|(i, p)| (ProductionID(i), (p.rhs.len(), p.lhs.clone())))
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
    let mut stage_6_table = HashMap::new();

    for (item_set, row) in stage_5_table {
        let mut shifts_and_reduces = HashMap::new();

        for (terminal, next_item_set) in row.shifts {
            shifts_and_reduces.insert(terminal, Stage6ShiftsAndReduces::Shift(next_item_set));
        }

        for (terminal, len_map) in row.reduces {
            if shifts_and_reduces.contains_key(&terminal) {
                let existing_entry = shifts_and_reduces.remove(&terminal).unwrap();
                if let Stage6ShiftsAndReduces::Shift(shift_set) = existing_entry {
                    shifts_and_reduces.insert(terminal, Stage6ShiftsAndReduces::Split {
                        shift: Some(shift_set),
                        reduces: len_map,
                    });
                } else {
                    panic!("Unexpected entry in shifts_and_reduces");
                }
            } else {
                let (len, nts) = len_map.iter().next().unwrap();
                let nt = nts.iter().next().unwrap();
                shifts_and_reduces.insert(terminal, Stage6ShiftsAndReduces::Reduce {
                    nonterminal: nt.clone(),
                    len: *len,
                });
            }
        }

        stage_6_table.insert(
            item_set,
            Stage6Row {
                shifts_and_reduces,
                gotos: row.gotos,
            },
        );
    }

    stage_6_table
}

fn stage_7(stage_6_table: Stage6Table, productions: &[Production]) -> Stage7Result {
    let mut terminal_map = BiMap::new();
    let mut non_terminal_map = BiMap::new();
    let mut item_set_map = BiMap::new();
    let mut next_terminal_id = 0;
    let mut next_non_terminal_id = 0;
    let mut next_state_id = 0;

    // Collect all terminals, non-terminals, and states
    let mut terminals = BTreeSet::new();
    let mut non_terminals = BTreeSet::new();

    for (item_set, row) in &stage_6_table {
        item_set_map.insert(item_set.clone(), StateID(next_state_id));
        next_state_id += 1;

        for t in row.shifts_and_reduces.keys() {
            terminals.insert(t.clone());
        }

        for nt in row.gotos.keys() {
            non_terminals.insert(nt.clone());
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

    for (_, row) in &stage_6_table {
        for (_, action) in &row.shifts_and_reduces {
            match action {
                Stage6ShiftsAndReduces::Reduce { nonterminal, .. } => {
                    if !non_terminal_map.contains_left(nonterminal) {
                        non_terminal_map.insert(nonterminal.clone(), NonTerminalID(next_non_terminal_id));
                        next_non_terminal_id += 1;
                    }
                }
                Stage6ShiftsAndReduces::Split { reduces, .. } => {
                    for (_, nts) in reduces {
                        for nt in nts {
                            if !non_terminal_map.contains_left(nt) {
                                non_terminal_map.insert(nt.clone(), NonTerminalID(next_non_terminal_id));
                                next_non_terminal_id += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let mut stage_7_table = HashMap::new();

    for (item_set, row) in stage_6_table {
        let state_id = *item_set_map.get_by_left(&item_set).unwrap();
        let mut shifts_and_reduces = HashMap::new();
        let mut gotos = HashMap::new();

        for (terminal, action) in row.shifts_and_reduces {
            let terminal_id = *terminal_map.get_by_left(&terminal).unwrap();
            let converted_action = match action {
                Stage6ShiftsAndReduces::Shift(next_item_set) => {
                    let next_state_id = *item_set_map.get_by_left(&next_item_set).unwrap();
                    Stage7ShiftsAndReduces::Shift(next_state_id)
                }
                Stage6ShiftsAndReduces::Reduce { nonterminal, len } => {
                    let nonterminal_id = *non_terminal_map.get_by_left(&nonterminal).unwrap();
                    Stage7ShiftsAndReduces::Reduce { nonterminal: nonterminal_id, len }
                }
                Stage6ShiftsAndReduces::Split { shift, reduces } => {
                    let shift_state_id = shift.as_ref().map(|set| *item_set_map.get_by_left(set).unwrap());
                    let converted_reduces: BTreeMap<usize, BTreeSet<NonTerminalID>> = reduces.into_iter().map(|(len, nts)| {
                        let nt_ids = nts.into_iter().map(|nt| *non_terminal_map.get_by_left(&nt).unwrap()).collect();
                        (len, nt_ids)
                    }).collect();
                    Stage7ShiftsAndReduces::Split { shift: shift_state_id, reduces: converted_reduces }
                }
            };
            shifts_and_reduces.insert(terminal_id, converted_action);
        }

        for (nonterminal, next_item_set) in row.gotos {
            let non_terminal_id = *non_terminal_map.get_by_left(&nonterminal).unwrap();
            let next_state_id = *item_set_map.get_by_left(&next_item_set).unwrap();
            gotos.insert(non_terminal_id, next_state_id);
        }

        stage_7_table.insert(state_id, Stage7Row { shifts_and_reduces, gotos });
    }

    let start_item = Item {
        production: productions[0].clone(),
        dot_position: 0,
    };
    let start_state_id = *item_set_map.get_by_left(&BTreeSet::from([start_item])).unwrap();

    (stage_7_table, terminal_map, non_terminal_map, item_set_map, start_state_id)
}

fn generate_glr_parser(productions: &[Production]) -> GLRParser {
    let stage_1_table = stage_1(productions);
    let stage_2_table = stage_2(stage_1_table, productions);
    let stage_3_table = stage_3(stage_2_table, productions);
    let stage_4_table = stage_4(stage_3_table, productions);
    let stage_5_table = stage_5(stage_4_table, productions);
    let stage_6_table = stage_6(stage_5_table);
    let (stage_7_table, terminal_map, non_terminal_map, item_set_map, start_state_id) = stage_7(stage_6_table, productions);

    GLRParser {
        stage_7_table,
        start_state_id,
        terminal_map,
        non_terminal_map,
        item_set_map,
    }
}

fn nt(name: &str) -> Symbol {
    Symbol::NonTerminal(NonTerminal(name.to_string()))
}

fn t(name: &str) -> Symbol {
    Symbol::Terminal(Terminal(name.to_string()))
}

fn prod(name: &str, rhs: Vec<Symbol>) -> Production {
    Production { lhs: NonTerminal(name.to_string()), rhs }
}

fn print_parse_table(
    stage_7_table: &Stage7Table,
    terminal_map: &BiMap<Terminal, TerminalID>,
    non_terminal_map: &BiMap<NonTerminal, NonTerminalID>,
    item_set_map: &BiMap<BTreeSet<Item>, StateID>,
) {
    let mut output = String::new();

    writeln!(&mut output, "Parse Table:").unwrap();
    for (&state_id, row) in stage_7_table.iter().collect::<BTreeMap<_, _>>() {
        writeln!(&mut output, "  State {}:", state_id.0).unwrap();

        writeln!(&mut output, "    Items:").unwrap();
        let item_set = item_set_map.get_by_right(&state_id).unwrap();
        for item in item_set {
            write!(&mut output, "      - {} ->", item.production.lhs.0).unwrap();
            for (i, symbol) in item.production.rhs.iter().enumerate() {
                if i == item.dot_position {
                    write!(&mut output, " •").unwrap();
                }
                match symbol {
                    Symbol::Terminal(terminal) => {
                        write!(&mut output, " {:?}", terminal.0).unwrap();
                    }
                    Symbol::NonTerminal(non_terminal) => {
                        write!(&mut output, " {}", non_terminal.0).unwrap();
                    }
                }
            }
            if item.dot_position == item.production.rhs.len() {
                write!(&mut output, " •").unwrap();
            }
            writeln!(&mut output, "").unwrap();
        }

        writeln!(&mut output, "    Actions:").unwrap();
        for (&terminal_id, action) in &row.shifts_and_reduces {
            let terminal = terminal_map.get_by_right(&terminal_id).unwrap();
            match action {
                Stage7ShiftsAndReduces::Shift(next_state_id) => {
                    writeln!(&mut output, "      - {:?} -> Shift {}", terminal.0, next_state_id.0).unwrap();
                }
                Stage7ShiftsAndReduces::Reduce { nonterminal, len } => {
                    let nt = non_terminal_map.get_by_right(nonterminal).unwrap();
                    writeln!(&mut output, "      - {:?} -> Reduce {} (len {})", terminal.0, nt.0, len).unwrap();
                }
                Stage7ShiftsAndReduces::Split { shift, reduces } => {
                    writeln!(&mut output, "      - {:?} -> Conflict:", terminal.0).unwrap();
                    if let Some(shift_state) = shift {
                        writeln!(&mut output, "        - Shift {}", shift_state.0).unwrap();
                    }
                    for (len, nt_ids) in reduces {
                        writeln!(&mut output, "        - Reduce (len {}):", len).unwrap();
                        for nt_id in nt_ids {
                            let nt = non_terminal_map.get_by_right(nt_id).unwrap();
                            writeln!(&mut output, "          - {}", nt.0).unwrap();
                        }
                    }
                }
            }
        }

        writeln!(&mut output, "    Gotos:").unwrap();
        for (&non_terminal_id, &next_state_id) in &row.gotos {
            let non_terminal = non_terminal_map.get_by_right(&non_terminal_id).unwrap();
            writeln!(&mut output, "      - {:?} -> {}", non_terminal.0, next_state_id.0).unwrap();
        }
    }

    writeln!(&mut output, "\nTerminal Map:").unwrap();
    for (terminal, terminal_id) in terminal_map {
        writeln!(&mut output, "  {:?} -> {}", terminal.0, terminal_id.0).unwrap();
    }

    writeln!(&mut output, "\nNon-Terminal Map:").unwrap();
    for (non_terminal, non_terminal_id) in non_terminal_map {
        writeln!(&mut output, "  {:?} -> {}", non_terminal.0, non_terminal_id.0).unwrap();
    }

    println!("{}", output);
}

struct GLRParser {
    stage_7_table: Stage7Table,
    start_state_id: StateID,
    terminal_map: BiMap<Terminal, TerminalID>,
    non_terminal_map: BiMap<NonTerminal, NonTerminalID>,
    item_set_map: BiMap<BTreeSet<Item>, StateID>,
}

struct GLRParserState<'a> {
    parser: &'a GLRParser,
    active_states: Vec<ParseState>,
    inactive_states: HashMap<usize, Vec<ParseState>>,
    input_pos: usize,
}

struct ParseState {
    stack: Vec<StateID>,
    symbols_stack: Vec<Symbol>,
    status: ParseStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ParseStatus {
    Active,
    Inactive(StopReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum StopReason {
    ActionNotFound,
    GotoNotFound,
}

impl GLRParserState<'_> {
    fn fully_matches(&self) -> bool {
        for state in &self.inactive_states[&self.input_pos] {
            if state.status == ParseStatus::Inactive(StopReason::GotoNotFound) {
                return true;
            }
        }

        false
    }
}

impl GLRParser {
    fn parse(&self, input: &[TerminalID]) -> GLRParserState {
        let mut state = GLRParserState::new(self);
        state.parse(input);
        state
    }
}

impl GLRParserState<'_> {
    fn new(parser: &GLRParser) -> GLRParserState {
        GLRParserState {
            parser,
            active_states: vec![
                ParseState {
                    stack: vec![parser.start_state_id],
                    symbols_stack: vec![],
                    status: ParseStatus::Active,
                },
            ],
            inactive_states: HashMap::new(),
            input_pos: 0,
        }
    }

    fn parse(&mut self, input: &[TerminalID]) {
        let mut active_states = vec![ParseState {
            stack: vec![self.parser.start_state_id],
            symbols_stack: vec![],
            status: ParseStatus::Active,
        }];

        for token in input {
            let (next_active_states, new_inactive_states) = self.step(active_states, &token);
            active_states = next_active_states;
            self.inactive_states.insert(self.input_pos, new_inactive_states);
        }

        let eof_token = self.parser.terminal_map.get_by_left(&Terminal("$".to_string())).cloned().unwrap();
        let (next_active_states, new_inactive_states) = self.step(active_states, &eof_token);
        active_states = next_active_states;
        self.inactive_states.insert(self.input_pos, new_inactive_states);

    }

    fn step(&self, mut active_states: Vec<ParseState>, token: &TerminalID) -> (Vec<ParseState>, Vec<ParseState>) {
        let mut next_active_states = Vec::new();
        let mut inactive_states = Vec::new();
        while let Some(state) = active_states.pop() {
            let stack = state.stack;
            let symbols_stack = state.symbols_stack;
            let state_id = *stack.last().unwrap();

            let row = self.parser.stage_7_table.get(&state_id).unwrap();

            if let Some(action) = row.shifts_and_reduces.get(&token) {
                match action {
                    Stage7ShiftsAndReduces::Shift(next_state_id) => {
                        let mut new_stack = stack;
                        let mut new_symbols = symbols_stack;
                        new_stack.push(*next_state_id);
                        new_symbols.push(Symbol::Terminal(self.parser.terminal_map.get_by_right(&token).unwrap().clone()));
                        next_active_states.push(ParseState {
                            stack: new_stack,
                            symbols_stack: new_symbols,
                            status: ParseStatus::Active,
                        });
                    }
                    Stage7ShiftsAndReduces::Reduce { nonterminal, len } => {
                        let mut new_stack = stack;
                        let mut new_symbols = symbols_stack;
                        for _ in 0..*len {
                            new_stack.pop();
                            new_symbols.pop();
                        }
                        let revealed_state = *new_stack.last().unwrap();
                        let goto_row = self.parser.stage_7_table.get(&revealed_state).unwrap();
                        if let Some(&goto_state) = goto_row.gotos.get(nonterminal) {
                            new_stack.push(goto_state);
                            new_symbols.push(Symbol::NonTerminal(self.parser.non_terminal_map.get_by_right(nonterminal).unwrap().clone()));
                            active_states.push(ParseState {
                                stack: new_stack,
                                symbols_stack: new_symbols,
                                status: ParseStatus::Active,
                            });
                        } else {
                            inactive_states.push(ParseState {
                                stack: new_stack,
                                symbols_stack: new_symbols,
                                status: ParseStatus::Inactive(StopReason::GotoNotFound),
                            });
                        }
                    }
                    Stage7ShiftsAndReduces::Split { shift, reduces } => {
                        if let Some(shift_state) = shift {
                            let mut new_stack = stack.clone();
                            let mut new_symbols = symbols_stack.clone();
                            new_stack.push(*shift_state);
                            new_symbols.push(Symbol::Terminal(self.parser.terminal_map.get_by_right(&token).unwrap().clone()));
                            next_active_states.push(ParseState {
                                stack: new_stack,
                                symbols_stack: new_symbols,
                                status: ParseStatus::Active,
                            });
                        }

                        for (len, nt_ids) in reduces {
                            for nt_id in nt_ids {
                                let mut new_stack = stack.clone();
                                let mut new_symbols = symbols_stack.clone();
                                for _ in 0..*len {
                                    new_stack.pop();
                                    new_symbols.pop();
                                }
                                let revealed_state = *new_stack.last().unwrap();
                                let goto_row = self.parser.stage_7_table.get(&revealed_state).unwrap();
                                if let Some(&goto_state) = goto_row.gotos.get(nt_id) {
                                    new_stack.push(goto_state);
                                    new_symbols.push(Symbol::NonTerminal(self.parser.non_terminal_map.get_by_right(nt_id).unwrap().clone()));
                                    active_states.push(ParseState {
                                        stack: new_stack,
                                        symbols_stack: new_symbols,
                                        status: ParseStatus::Active,
                                    });
                                } else {
                                    inactive_states.push(ParseState {
                                        stack: new_stack,
                                        symbols_stack: new_symbols,
                                        status: ParseStatus::Inactive(StopReason::GotoNotFound),
                                    })
                                }
                            }
                        }
                    }
                }
            } else {
                inactive_states.push(ParseState {
                    stack,
                    symbols_stack,
                    status: ParseStatus::Inactive(StopReason::ActionNotFound),
                });
            }

        }

        (next_active_states, inactive_states)
    }
}

#[cfg(test)]
mod glalr_tests {
    use super::*;

    #[test]
    fn test_simple_parse_table() {
        let productions = vec![
            // S -> a
            prod("S", vec![nt("A")]),
            // A -> A a | b
            prod("A", vec![nt("A"), t("a")]),
            prod("A", vec![t("b")]),
        ];

        let parser = generate_glr_parser(&productions);

        print_parse_table(&parser.stage_7_table, &parser.terminal_map, &parser.non_terminal_map, &parser.item_set_map);

        let tokenize = |input: &str| -> Vec<TerminalID> {
            let mut result = Vec::new();
            for c in input.chars() {
                let terminal = Terminal(c.to_string());
                if let Some(id) = parser.terminal_map.get_by_left(&terminal) {
                    result.push(*id);
                } else {
                    panic!("Unknown token: {}", c);
                }
            }
            result
        };

        assert!(parser.parse(
            &tokenize("b"),
        )
        .fully_matches());
        assert!(parser.parse(
            &tokenize("ba"),
        )
        .fully_matches());
        assert!(parser.parse(
            &tokenize("baa"),
        )
        .fully_matches());

        assert!(!parser.parse(
            &tokenize("a"),
        )
        .fully_matches());

        assert!(!parser.parse(
            &tokenize("bb"),
        )
        .fully_matches());
    }

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

        let parser = generate_glr_parser(&productions);

        print_parse_table(&parser.stage_7_table, &parser.terminal_map, &parser.non_terminal_map, &parser.item_set_map);

        let tokenize = |input: &str| -> Vec<TerminalID> {
            let mut result = Vec::new();
            for c in input.chars() {
                let terminal = Terminal(c.to_string());
                if let Some(id) = parser.terminal_map.get_by_left(&terminal) {
                    result.push(*id);
                } else {
                    panic!("Unknown token: {}", c);
                }
            }
            result
        };

        assert!(parser.parse(
            &tokenize("i"),
        )
        .fully_matches());
        assert!(parser.parse(
            &tokenize("i+i*i"),
        )
        .fully_matches());
        assert!(parser.parse(
            &tokenize("i+i"),
        )
        .fully_matches());
        assert!(parser.parse(
            &tokenize("i*i"),
        )
        .fully_matches());
        assert!(parser.parse(
            &tokenize("i"),
        )
        .fully_matches());
        assert!(parser.parse(
            &tokenize("(i+i)*i"),
        )
        .fully_matches());

        assert!(!parser.parse(
            &tokenize("i+"),
        )
        .fully_matches());
        assert!(!parser.parse(
            &tokenize("i++i"),
        )
        .fully_matches());
        assert!(!parser.parse(
            &tokenize(""),
        )
        .fully_matches());
        assert!(!parser.parse(
            &tokenize(")"),
        )
        .fully_matches());
    }
}