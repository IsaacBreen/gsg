// src/interface.rs
use crate::charmap::TrieMap;
use crate::finite_automata::{Expr, Regex, RegexState};
use crate::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use crate::glr::parser::{Action, GLRParser, GLRParserState, ParseState, ParseStatus, StopReason};
use crate::glr::table::{generate_glr_parser, NonTerminalID, ProductionID, StateID, TerminalID};
use crate::precompute::{precompute, Token, Tokenizer};
use crate::tokenizer_combinators::*;
use crate::u8set::U8Set;
use bimap::BiMap;
use std::collections::{BTreeMap, BTreeSet, HashMap};

type LLMToken = &'static [u8];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GrammarExpr {
    Terminal(Expr),
    NonTerminal(String),
    Sequence(Vec<GrammarExpr>),
    Choice(Vec<GrammarExpr>),
    Optional(Box<GrammarExpr>),
    Repeat(Box<GrammarExpr>),
}

impl GrammarExpr {
    pub fn terminal(expr: Expr) -> Self {
        GrammarExpr::Terminal(expr)
    }

    pub fn nonterminal(name: &str) -> Self {
        GrammarExpr::NonTerminal(name.to_string())
    }

    pub fn sequence(exprs: Vec<GrammarExpr>) -> Self {
        GrammarExpr::Sequence(exprs)
    }

    pub fn choice(exprs: Vec<GrammarExpr>) -> Self {
        GrammarExpr::Choice(exprs)
    }

    pub fn optional(expr: GrammarExpr) -> Self {
        GrammarExpr::Optional(Box::new(expr))
    }

    pub fn repeat(expr: GrammarExpr) -> Self {
        GrammarExpr::Repeat(Box::new(expr))
    }
}

pub struct Grammar {
    pub productions: Vec<Production>,
    pub start_symbol: NonTerminal,
    pub terminal_map: BiMap<Terminal, TerminalID>,
    pub non_terminal_map: BiMap<NonTerminal, NonTerminalID>,
}

impl Grammar {
    pub fn from_exprs(start_symbol: &str, exprs: HashMap<String, GrammarExpr>) -> Self {
        let mut productions = Vec::new();
        let mut terminal_map = BiMap::new();
        let mut non_terminal_map = BiMap::new();
        let mut next_terminal_id = 0;
        let mut next_non_terminal_id = 0;

        fn convert_expr(
            expr: &GrammarExpr,
            productions: &mut Vec<Production>,
            terminal_map: &mut BiMap<Terminal, TerminalID>,
            non_terminal_map: &mut BiMap<NonTerminal, NonTerminalID>,
            next_terminal_id: &mut usize,
            next_non_terminal_id: &mut usize,
        ) -> Vec<Symbol> {
            match expr {
                GrammarExpr::Terminal(expr) => {
                    let regex = expr.clone().build();
                    let mut regex_state = regex.init();
                    let u8set = regex_state.get_u8set();
                    let mut symbols = Vec::new();

                    for byte in u8set.iter() {
                        let terminal_str = (byte as char).to_string();
                        let terminal = Terminal(terminal_str.clone());
                        if !terminal_map.contains_left(&terminal) {
                            terminal_map.insert(terminal.clone(), TerminalID(*next_terminal_id));
                            *next_terminal_id += 1;
                        }
                        symbols.push(Symbol::Terminal(terminal));
                    }
                    if symbols.is_empty() {
                        let epsilon_terminal = Terminal("ε".to_string()); 
                        if !terminal_map.contains_left(&epsilon_terminal) {
                            terminal_map.insert(epsilon_terminal.clone(), TerminalID(*next_terminal_id));
                            *next_terminal_id += 1;
                        }
                        symbols.push(Symbol::Terminal(epsilon_terminal));
                    }

                    symbols
                }
                GrammarExpr::NonTerminal(name) => {
                    if !non_terminal_map.contains_left(&NonTerminal(name.clone())) {
                        non_terminal_map.insert(NonTerminal(name.clone()), NonTerminalID(*next_non_terminal_id));
                        *next_non_terminal_id += 1;
                    }
                    vec![Symbol::NonTerminal(NonTerminal(name.clone()))]
                }
                GrammarExpr::Sequence(exprs) => {
                    let mut sequence_symbols = Vec::new();
                    for e in exprs {
                        sequence_symbols.extend(convert_expr(e, productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id));
                    }
                    sequence_symbols
                }
                GrammarExpr::Choice(exprs) => {
                    let new_nonterminal = format!("Choice{}", *next_non_terminal_id);
                    if !non_terminal_map.contains_left(&NonTerminal(new_nonterminal.clone())) {
                        non_terminal_map.insert(NonTerminal(new_nonterminal.clone()), NonTerminalID(*next_non_terminal_id));
                        *next_non_terminal_id += 1;
                    }

                    let mut choice_productions = Vec::new();
                    for expr in exprs {
                        choice_productions.push(Production {
                            lhs: NonTerminal(new_nonterminal.clone()),
                            rhs: convert_expr(expr, productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id),
                        });
                    }
                    productions.extend(choice_productions);

                    vec![Symbol::NonTerminal(NonTerminal(new_nonterminal))]
                }
                GrammarExpr::Optional(expr) => convert_expr(&GrammarExpr::choice(vec![expr.as_ref().clone(), GrammarExpr::sequence(vec![])]), productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id),
                GrammarExpr::Repeat(expr) => convert_expr(&GrammarExpr::optional(GrammarExpr::sequence(vec![expr.as_ref().clone(), GrammarExpr::repeat(expr.as_ref().clone())])), productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id),
            }
        }

        for (name, expr) in &exprs {
            let rhs = convert_expr(expr, &mut productions, &mut terminal_map, &mut non_terminal_map, &mut next_terminal_id, &mut next_non_terminal_id);
            productions.push(Production {
                lhs: NonTerminal(name.clone()),
                rhs,
            });
        }

        // Add the start production
        productions.insert(0, Production {
            lhs: NonTerminal(start_symbol.to_string()),
            rhs: vec![Symbol::NonTerminal(NonTerminal(exprs.keys().next().expect("Grammar must have at least one rule").clone()))],
        });

        if !non_terminal_map.contains_left(&NonTerminal(start_symbol.to_string())) {
            non_terminal_map.insert(NonTerminal(start_symbol.to_string()), NonTerminalID(next_non_terminal_id));
        }

        Self {
            productions,
            start_symbol: NonTerminal(start_symbol.to_string()),
            terminal_map,
            non_terminal_map,
        }
    }
}

pub struct GrammarConstraintState<T: Tokenizer> {
    pub tokenizer: T,
    pub parser: GLRParser,
    pub precomputed: BTreeMap<StateID, BTreeMap<Vec<Token>, BTreeMap<LLMToken, StateID>>>,
    pub states: Vec<(ParseState, BTreeSet<StateID>)>,
}

impl<T: Tokenizer> GrammarConstraintState<T> {
    pub fn new_from_grammar(tokenizer: T, grammar: Grammar, llm_tokens: &[LLMToken]) -> Self {
        let parser = generate_glr_parser(&grammar.productions);
        let precomputed = precompute(&tokenizer, llm_tokens);
        // Convert `precompute::StateID` into `glr::table::StateID`
        let states = vec![(parser.init_parse_state(), BTreeSet::from([StateID(tokenizer.initial_state_id())]))];
        Self {
            tokenizer,
            parser,
            precomputed,
            states,
        }
    }
    // ... other methods for GrammarConstraintState ...
}