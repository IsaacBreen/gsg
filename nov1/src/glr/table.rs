use std::collections::{BTreeMap, BTreeSet, HashMap};
use bimap::BiMap;


use std::fmt::{Display, Write};
use std::collections::VecDeque;
use crate::glr::grammar::{compute_first_sets, compute_follow_sets, NonTerminal, Production, Symbol, Terminal};
use super::items::{compute_closure, compute_goto, split_on_dot, Item};

type Stage1Table = HashMap<BTreeSet<Item>, Stage1Row>;
type Stage2Table = HashMap<BTreeSet<Item>, Stage2Row>;
type Stage3Table = HashMap<BTreeSet<Item>, Stage3Row>;
type Stage4Table = HashMap<BTreeSet<Item>, Stage4Row>;
type Stage5Table = HashMap<BTreeSet<Item>, Stage5Row>;
type Stage6Table = HashMap<BTreeSet<Item>, Stage6Row>;
pub type Stage7Table = HashMap<StateID, Stage7Row>;


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
    reduces: HashMap<Terminal, BTreeSet<Item>>,
}
#[derive(Debug)]
struct Stage4Row {
    shifts: HashMap<Terminal, BTreeSet<Item>>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
    reduces: HashMap<Terminal, BTreeSet<ProductionID>>,
}
#[derive(Debug)]
struct Stage5Row {
    shifts: HashMap<Terminal, BTreeSet<Item>>,
    gotos: HashMap<NonTerminal, BTreeSet<Item>>,
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
pub enum Stage7ShiftsAndReduces {
    Shift(StateID),
    Reduce { nonterminal: NonTerminalID, len: usize },
    Split {
        shift: Option<StateID>,
        reduces: BTreeMap<usize, BTreeSet<NonTerminalID>>,
    },
}

#[derive(Debug)]
pub struct Stage7Row {
    pub shifts_and_reduces: HashMap<TerminalID, Stage7ShiftsAndReduces>,
    pub gotos: HashMap<NonTerminalID, StateID>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateID(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductionID(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonTerminalID(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerminalID(pub usize);


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

pub fn generate_glr_parser(productions: &[Production]) -> (Stage7Table, BiMap<Terminal, TerminalID>, BiMap<NonTerminal, NonTerminalID>, BiMap<BTreeSet<Item>, StateID>, StateID) {
    let stage_1_table = stage_1(productions);
    let stage_2_table = stage_2(stage_1_table, productions);
    let stage_3_table = stage_3(stage_2_table, productions);
    let stage_4_table = stage_4(stage_3_table, productions);
    let stage_5_table = stage_5(stage_4_table, productions);
    let stage_6_table = stage_6(stage_5_table);
    stage_7(stage_6_table, productions)
}