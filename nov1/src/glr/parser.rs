use crate::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use crate::glr::items::Item;
use crate::glr::table::{NonTerminalID, ProductionID, Stage7ShiftsAndReduces, Stage7Table, StateID, TerminalID};
use crate::gss::{GSSNode, GSSTrait};

use bimap::BiMap;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseState {
    pub stack: Rc<GSSNode<StateID>>,
    pub action_stack: Option<Rc<GSSNode<Action>>>,
    pub status: ParseStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
    Shift(TerminalID),
    Reduce { production_id: ProductionID, len: usize, nonterminal_id: NonTerminalID },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseStatus {
    Active,
    Inactive(StopReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StopReason {
    ActionNotFound,
    GotoNotFound,
}


pub struct GLRParser {
    pub stage_7_table: Stage7Table,
    pub productions: Vec<Production>,
    pub terminal_map: BiMap<Terminal, TerminalID>,
    pub non_terminal_map: BiMap<NonTerminal, NonTerminalID>,
    pub item_set_map: BiMap<BTreeSet<Item>, StateID>,
    pub start_state_id: StateID,
    pub eof_terminal_id: TerminalID,
}

impl GLRParser {

    pub fn new(
        stage_7_table: Stage7Table,
        productions: Vec<Production>,
        terminal_map: BiMap<Terminal, TerminalID>,
        non_terminal_map: BiMap<NonTerminal, NonTerminalID>,
        item_set_map: BiMap<BTreeSet<Item>, StateID>,
        start_state_id: StateID,
        eof_terminal_id: TerminalID,
    ) -> Self {
        Self {
            stage_7_table,
            productions,
            terminal_map,
            non_terminal_map,
            item_set_map,
            start_state_id,
            eof_terminal_id,
        }
    }

    pub fn init_parser(&self) -> GLRParserState {
        let start_stack = GSSNode::new(self.start_state_id);

        GLRParserState {
            parser: self,
            active_states: vec![ParseState {
                stack: Rc::new(start_stack.clone()),
                action_stack: None,
                status: ParseStatus::Active,
            }],
            inactive_states: HashMap::new(),
            input_pos: 0,
        }
    }

    pub fn init_parser_from_parse_state(&self, parse_state: ParseState) -> GLRParserState {
        let mut state = self.init_parser();
        state.active_states.push(parse_state.clone());
        state
    }


    pub fn parse(&self, input: &[TerminalID]) -> GLRParserState {
        let mut state = self.init_parser();
        state.parse(input);
        state
    }
}

impl Display for GLRParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stage_7_table = &self.stage_7_table;
        let terminal_map = &self.terminal_map;
        let non_terminal_map = &self.non_terminal_map;
        let item_set_map = &self.item_set_map;

        writeln!(f, "Parse Table:")?;
        for (&state_id, row) in stage_7_table.iter().collect::<BTreeMap<_, _>>() {
            writeln!(f, "  State {}:", state_id.0)?;

            writeln!(f, "    Items:")?;
            let item_set = item_set_map.get_by_right(&state_id).unwrap();
            for item in item_set {
                write!(f, "      - {} ->", item.production.lhs.0)?;
                for (i, symbol) in item.production.rhs.iter().enumerate() {
                    if i == item.dot_position {
                        write!(f, " •")?;
                    }
                    match symbol {
                        Symbol::Terminal(terminal) => {
                            write!(f, " {:?}", terminal.0)?;
                        }
                        Symbol::NonTerminal(non_terminal) => {
                            write!(f, " {}", non_terminal.0)?;
                        }
                    }
                }
                if item.dot_position == item.production.rhs.len() {
                    write!(f, " •")?;
                }
                writeln!(f)?;
            }

            writeln!(f, "    Actions:")?;
            for (&terminal_id, action) in &row.shifts_and_reduces {
                let terminal = terminal_map.get_by_right(&terminal_id).unwrap();
                match action {
                    Stage7ShiftsAndReduces::Shift(next_state_id) => {
                        writeln!(f, "      - {:?} -> Shift {}", terminal.0, next_state_id.0)?;
                    }
                    Stage7ShiftsAndReduces::Reduce { production_id, nonterminal_id: nonterminal, len } => {
                        let nt = non_terminal_map.get_by_right(nonterminal).unwrap();
                        writeln!(f, "      - {:?} -> Reduce {} (len {})", terminal.0, nt.0, len)?;
                    }
                    Stage7ShiftsAndReduces::Split { shift, reduces } => {
                        writeln!(f, "      - {:?} -> Conflict:", terminal.0)?;
                        if let Some(shift_state) = shift {
                            writeln!(f, "        - Shift {}", shift_state.0)?;
                        }
                        for (len, nt_id_to_prod_ids) in reduces {
                            writeln!(f, "        - Reduce (len {}):", len)?;
                            for (nt_id, prod_ids) in nt_id_to_prod_ids {
                                let nt = non_terminal_map.get_by_right(nt_id).unwrap();
                                for prod_id in prod_ids {
                                    let prod = self.productions.get(prod_id.0).unwrap();
                                    writeln!(f, "          - {} -> {}", nt.0, prod.lhs.0)?;
                                }
                            }

                        }
                    }
                }
            }

            writeln!(f, "    Gotos:")?;
            for (&non_terminal_id, &next_state_id) in &row.gotos {
                let non_terminal = non_terminal_map.get_by_right(&non_terminal_id).unwrap();
                writeln!(f, "      - {:?} -> {}", non_terminal.0, next_state_id.0)?;
            }
        }

        writeln!(f, "\nTerminal Map:")?;
        for (terminal, terminal_id) in terminal_map {
            writeln!(f, "  {:?} -> {}", terminal.0, terminal_id.0)?;
        }

        writeln!(f, "\nNon-Terminal Map:")?;
        for (non_terminal, non_terminal_id) in non_terminal_map {
            writeln!(f, "  {:?} -> {}", non_terminal.0, non_terminal_id.0)?;
        }

        Ok(())
    }
}


#[derive(Clone)]
pub struct GLRParserState<'a> {
    pub parser: &'a GLRParser,
    pub active_states: Vec<ParseState>,
    pub inactive_states: HashMap<usize, Vec<ParseState>>,
    pub input_pos: usize,
}



impl<'a> GLRParserState<'a> {
    pub fn parse(&mut self, input: &[TerminalID]) {
        self.parse_part(input);
        self.parse_eof();
    }

    pub fn parse_part(&mut self, input: &[TerminalID]) {
        for &token_id in input {
            self.step(token_id);
        }
    }

    pub fn parse_eof(&mut self) {
        self.step(self.parser.eof_terminal_id);
    }


    pub fn step(&mut self, token_id: TerminalID) {
        let mut next_active_states = Vec::new();
        let mut inactive_states = Vec::new();

        while let Some(state) = self.active_states.pop() {
            let stack = state.stack;
            let action_stack = state.action_stack;
            let state_id = *stack.peek();

            let row = self.parser.stage_7_table.get(&state_id).unwrap();

            if let Some(action) = row.shifts_and_reduces.get(&token_id) {
                match action {
                    Stage7ShiftsAndReduces::Shift(next_state_id) => {
                        let new_stack = stack.push(*next_state_id);
                        let new_actions = action_stack.push(Action::Shift(token_id));
                        next_active_states.push(ParseState {
                            stack: Rc::new(new_stack),
                            action_stack: Some(Rc::new(new_actions)),
                            status: ParseStatus::Active,
                        });

                    }
                    Stage7ShiftsAndReduces::Reduce { production_id, nonterminal_id: nonterminal, len } => {
                        let popped_stack_nodes = stack.popn(*len);

                        for stack_node in popped_stack_nodes.into_iter() {
                            let revealed_state = *stack_node.peek();
                            let goto_row = self.parser.stage_7_table.get(&revealed_state).unwrap();

                            if let Some(&goto_state) = goto_row.gotos.get(nonterminal) {
                                let new_stack = stack_node.push(goto_state);
                                let new_actions = action_stack.clone().push(Action::Reduce { production_id: *production_id, len: *len, nonterminal_id: *nonterminal });
                                self.active_states.push(ParseState {
                                    stack: Rc::new(new_stack),
                                    action_stack: Some(Rc::new(new_actions)),
                                    status: ParseStatus::Active,
                                });
                            } else {
                                inactive_states.push(ParseState {
                                    stack: stack_node,
                                    action_stack: None,
                                    status: ParseStatus::Inactive(StopReason::GotoNotFound),
                                });
                            }
                        }
                    }
                    Stage7ShiftsAndReduces::Split { shift, reduces } => {
                        if let Some(shift_state) = shift {

                            let new_stack = stack.push(*shift_state);
                            let new_actions = action_stack.clone().push(Action::Shift(token_id));

                            next_active_states.push(ParseState {
                                stack: Rc::new(new_stack),
                                action_stack: Some(Rc::new(new_actions)),
                                status: ParseStatus::Active,
                            });
                        }

                        for (len, nt_ids) in reduces {
                            for (nt_id, prod_ids) in nt_ids {
                                let popped_stack_nodes = stack.popn(*len);

                                for stack_node in popped_stack_nodes.into_iter() {
                                    let revealed_state = *stack_node.peek();
                                    let goto_row = self.parser.stage_7_table.get(&revealed_state).unwrap();
                                    if let Some(&goto_state) = goto_row.gotos.get(nt_id) {
                                        let new_stack = Rc::new(stack_node.push(goto_state));
                                        for prod_id in prod_ids {
                                            let new_actions = action_stack.clone().push(Action::Reduce { production_id: *prod_id, len: *len, nonterminal_id: *nt_id });
                                            self.active_states.push(ParseState {
                                                stack: new_stack.clone(),
                                                action_stack: Some(Rc::new(new_actions)),
                                                status: ParseStatus::Active,
                                            });
                                        }
                                    } else {
                                        inactive_states.push(ParseState {
                                            stack: stack_node.clone(),
                                            action_stack: action_stack.clone(),
                                            status: ParseStatus::Inactive(StopReason::GotoNotFound),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                inactive_states.push(ParseState {
                    stack,
                    action_stack,
                    status: ParseStatus::Inactive(StopReason::ActionNotFound),
                });
            }
        }
        self.active_states = next_active_states;
        self.inactive_states.insert(self.input_pos, inactive_states);

        if token_id != self.parser.eof_terminal_id {
            self.input_pos += 1;
        }
    }

    pub fn merge_active_states(&mut self) {
        let mut active_state_map: HashMap<ParseStateKey, ParseState> = HashMap::new();

        let mut new_active_states = Vec::new();

        for mut state in std::mem::take(&mut self.active_states) {
            let key = state.key();
            if let Some(existing) = active_state_map.get_mut(&key) {
                Rc::make_mut(&mut existing.stack).merge(state.stack.as_ref().clone());
                if let Some(existing_action_stack) = existing.action_stack.as_mut() {
                    Rc::make_mut(existing_action_stack).merge(state.action_stack.unwrap().as_ref().clone());
                }
            } else {
                active_state_map.insert(key, state.clone());
                new_active_states.push(state);
            }
        }
        self.active_states = new_active_states;
    }

    pub fn fully_matches(&self) -> bool {
        !self.fully_matching_states().is_empty()
    }

    pub fn fully_matching_states(&self) -> Vec<&ParseState> {
        if let Some(states) = self.inactive_states.get(&self.input_pos) {
            states
                .iter()
                .filter(|state| state.status == ParseStatus::Inactive(StopReason::GotoNotFound))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn is_ok(&self) -> bool {
        !self.active_states.is_empty() || self.fully_matches()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ParseStateKey {
    stack: StateID,
    action_stack: Option<Action>,
}

impl ParseState {
    pub fn key(&self) -> ParseStateKey {
        ParseStateKey {
            stack: *self.stack.peek(),
            action_stack: self.action_stack.peek().cloned(),
        }
    }
}