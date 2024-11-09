use super::items::{compute_closure, compute_goto, split_on_dot, Item};
use crate::glr::grammar::{compute_first_sets, compute_follow_sets, NonTerminal, Production, Symbol, Terminal};
use crate::glr::parser::GLRParser;
use bimap::BiBTreeMap;
use std::collections::VecDeque;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;

type Stage1Table = BTreeMap<BTreeSet<Item>, Stage1Row>;
type Stage2Table = BTreeMap<BTreeSet<Item>, Stage2Row>;
type Stage3Table = BTreeMap<BTreeSet<Item>, Stage3Row>;
type Stage4Table = BTreeMap<BTreeSet<Item>, Stage4Row>;
type Stage5Table = BTreeMap<BTreeSet<Item>, Stage5Row>;
type Stage6Table = BTreeMap<BTreeSet<Item>, Stage6Row>;
pub type Stage7Table = BTreeMap<StateID, Stage7Row>;


type Stage1Row = BTreeMap<Option<Symbol>, BTreeSet<Item>>;
#[derive(Debug)]
struct Stage2Row {
    shifts: BTreeMap<Terminal, BTreeSet<Item>>,
    gotos: BTreeMap<NonTerminal, BTreeSet<Item>>,
    reduces: BTreeSet<Item>,
}
#[derive(Debug)]
struct Stage3Row {
    shifts: BTreeMap<Terminal, BTreeSet<Item>>,
    gotos: BTreeMap<NonTerminal, BTreeSet<Item>>,
    /// Split the reduce items by lookahead.
    /// For LR(0), all possible terminals map to the entire reduce item set.
    /// But there are various cleverer and more selective ways to compute lookaheads.
    /// For simplicity, use LALR.
    reduces: BTreeMap<Terminal, BTreeSet<Item>>,
}
#[derive(Debug)]
struct Stage4Row {
    shifts: BTreeMap<Terminal, BTreeSet<Item>>,
    gotos: BTreeMap<NonTerminal, BTreeSet<Item>>,
    reduces: BTreeMap<Terminal, BTreeSet<ProductionID>>,
}
type Stage5Row = Stage4Row;
#[derive(Debug)]
struct Stage6Row {
    shifts_and_reduces: BTreeMap<Terminal, Stage6ShiftsAndReduces>,
    gotos: BTreeMap<NonTerminal, BTreeSet<Item>>,
}

#[derive(Debug)]
enum Stage6ShiftsAndReduces {
    Shift(BTreeSet<Item>),
    Reduce(ProductionID),
    Split {
        shift: Option<BTreeSet<Item>>,
        reduces: BTreeSet<ProductionID>,
    },
}

#[derive(Debug)]
pub enum Stage7ShiftsAndReduces {
    /// Map each item set to a unique ID, and do the same for terminals and nonterminals.
    Shift(StateID),
    Reduce { production_id: ProductionID, nonterminal_id: NonTerminalID, len: usize },
    Split {
        shift: Option<StateID>,
        reduces: BTreeMap<usize, BTreeMap<NonTerminalID, BTreeSet<ProductionID>>>,
    },
}

#[derive(Debug)]
pub struct Stage7Row {
    /// Map each item set to a unique ID, and do the same for terminals and nonterminals.
    pub shifts_and_reduces: BTreeMap<TerminalID, Stage7ShiftsAndReduces>,
    pub gotos: BTreeMap<NonTerminalID, StateID>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateID(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProductionID(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonTerminalID(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TerminalID(pub usize);



type Stage1Result = Stage1Table;
type Stage2Result = Stage2Table;
type Stage3Result = Stage3Table;
type Stage4Result = Stage4Table;
type Stage5Result = Stage5Table;
type Stage6Result = Stage6Table;
type Stage7Result = (
    Stage7Table,
    BiBTreeMap<Terminal, TerminalID>,
    BiBTreeMap<NonTerminal, NonTerminalID>,
    BiBTreeMap<BTreeSet<Item>, StateID>,
    StateID,
    TerminalID,
);

fn stage_1(productions: &[Production]) -> Stage1Result {
    let initial_item = Item {
        production: productions[0].clone(),
        dot_position: 0,
    };
    let initial_closure = BTreeSet::from([initial_item]);
    let mut worklist = VecDeque::from([initial_closure.clone()]);

    let mut transitions: BTreeMap<BTreeSet<Item>, BTreeMap<Option<Symbol>, BTreeSet<Item>>> = BTreeMap::new();

    while let Some(items) = worklist.pop_front() {
        if transitions.contains_key(&items) {
            continue;
        }

        let closure = compute_closure(&items, productions);
        let splits = split_on_dot(&closure);
        let mut row = BTreeMap::new();

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
    let mut stage_2_table = BTreeMap::new();
    for (item_set, transitions) in stage_1_table {
        let mut shifts = BTreeMap::new();
        let mut gotos = BTreeMap::new();
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

    let mut stage_3_table = BTreeMap::new();

    for (item_set, row) in stage_2_table {
        let mut reduces: BTreeMap<Terminal, BTreeSet<Item>> = BTreeMap::new();

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
    let production_ids: BTreeMap<Production, ProductionID> = productions
        .iter()
        .enumerate()
        .map(|(i, p)| (p.clone(), ProductionID(i)))
        .collect();

    let mut stage_4_table = BTreeMap::new();

    for (item_set, row) in stage_3_table {
        let mut reduces = BTreeMap::new();

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
    // todo: remove this
    stage_4_table
}

fn stage_6(stage_5_table: Stage5Table) -> Stage6Result {
    let mut stage_6_table = BTreeMap::new();

    for (item_set, row) in stage_5_table {
        let mut shifts_and_reduces = BTreeMap::new();

        for (terminal, next_item_set) in row.shifts {
            shifts_and_reduces.insert(terminal, Stage6ShiftsAndReduces::Shift(next_item_set));
        }

        for (terminal, mut production_ids) in row.reduces {
            if let Some(mut existing) = shifts_and_reduces.remove(&terminal) {
                match existing {
                    Stage6ShiftsAndReduces::Shift(shift_set) => {
                        shifts_and_reduces.insert(terminal, Stage6ShiftsAndReduces::Split {
                            shift: Some(shift_set.clone()),
                            reduces: production_ids.clone(),
                        });
                    }
                    Stage6ShiftsAndReduces::Reduce(existing_production_id) => {
                        production_ids.insert(existing_production_id);
                        shifts_and_reduces.insert(terminal, Stage6ShiftsAndReduces::Split {
                            shift: None,
                            reduces: production_ids,
                        });
                    }
                    Stage6ShiftsAndReduces::Split { shift, mut reduces } => {
                        reduces.extend(production_ids.into_iter());
                        shifts_and_reduces.insert(terminal, Stage6ShiftsAndReduces::Split { shift, reduces });
                    }
                }
            } else {
                // If there's only one production ID, we can optimize by storing it directly
                if production_ids.len() == 1 {
                    shifts_and_reduces.insert(terminal, Stage6ShiftsAndReduces::Reduce(production_ids.iter().next().unwrap().clone()));
                } else {
                    shifts_and_reduces.insert(terminal, Stage6ShiftsAndReduces::Split { shift: None, reduces: production_ids });
                }
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
    let mut terminal_map = BiBTreeMap::new();
    let mut non_terminal_map = BiBTreeMap::new();
    let mut item_set_map = BiBTreeMap::new();
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

    // Add the EOF terminal
    terminals.insert(Terminal("$".to_string()));


    for t in terminals {
        terminal_map.insert(t.clone(), TerminalID(next_terminal_id));
        next_terminal_id += 1;
    }

    for nt in non_terminals {
        non_terminal_map.insert(nt.clone(), NonTerminalID(next_non_terminal_id));
        next_non_terminal_id += 1;
    }

    // Ensure all nonterminals on the LHS of productions are in the map
    for production in productions {
        if !non_terminal_map.contains_left(&production.lhs) {
            non_terminal_map.insert(production.lhs.clone(), NonTerminalID(next_non_terminal_id));
            next_non_terminal_id += 1;
        }
    }

    let mut stage_7_table = BTreeMap::new();

    for (item_set, row) in stage_6_table {
        let state_id = *item_set_map.get_by_left(&item_set).unwrap();
        let mut shifts_and_reduces = BTreeMap::new();
        let mut gotos = BTreeMap::new();

        for (terminal, action) in row.shifts_and_reduces {
            let terminal_id = *terminal_map.get_by_left(&terminal).unwrap();
            let converted_action = match action {
                Stage6ShiftsAndReduces::Shift(next_item_set) => {
                    let next_state_id = *item_set_map.get_by_left(&next_item_set).unwrap();
                    Stage7ShiftsAndReduces::Shift(next_state_id)
                }
                Stage6ShiftsAndReduces::Reduce(production_id) => {
                    let production = productions.get(production_id.0).unwrap();
                    let nonterminal_id = *non_terminal_map.get_by_left(&production.lhs).unwrap();
                    let len = production.rhs.len();
                    Stage7ShiftsAndReduces::Reduce { production_id, nonterminal_id, len }
                }
                Stage6ShiftsAndReduces::Split { shift, reduces } => {
                    let shift_state_id = shift.as_ref().map(|set| *item_set_map.get_by_left(set).unwrap());
                    let mut len_to_nt_to_production_id: BTreeMap<usize, BTreeMap<NonTerminalID, BTreeSet<ProductionID>>> = BTreeMap::new();
                    for production_id in reduces {
                        let production = productions.get(production_id.0).unwrap();
                        let nonterminal_id = *non_terminal_map.get_by_left(&production.lhs).unwrap();
                        let len = production.rhs.len();
                        len_to_nt_to_production_id.entry(len).or_default().entry(nonterminal_id).or_default().insert(production_id);
                    }
                    Stage7ShiftsAndReduces::Split { shift: shift_state_id, reduces: len_to_nt_to_production_id }
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
    let eof_terminal_id = *terminal_map.get_by_left(&Terminal("$".to_string())).unwrap();

    (stage_7_table, terminal_map, non_terminal_map, item_set_map, start_state_id, eof_terminal_id)
}

pub fn generate_glr_parser(productions: &[Production]) -> GLRParser {
    let stage_1_table = stage_1(productions);
    let stage_2_table = stage_2(stage_1_table, productions);
    let stage_3_table = stage_3(stage_2_table, productions);
    let stage_4_table = stage_4(stage_3_table, productions);
    let stage_5_table = stage_5(stage_4_table, productions);
    let stage_6_table = stage_6(stage_5_table);
    let (stage_7_table, terminal_map, non_terminal_map, item_set_map, start_state_id, eof_terminal_id) = stage_7(stage_6_table, productions);

    GLRParser::new(stage_7_table, productions.to_vec(), terminal_map, non_terminal_map, item_set_map, start_state_id, eof_terminal_id)
}