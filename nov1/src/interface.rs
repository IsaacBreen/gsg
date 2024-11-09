use crate::finite_automata::{groups, non_greedy_group, ExprGroup};
use crate::charmap::TrieMap;
use crate::finite_automata::{Expr, Regex, RegexState};
use crate::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use crate::glr::parser::{GLRParser, ParseState};
use crate::glr::table::{generate_glr_parser, NonTerminalID, StateID, TerminalID};
use crate::precompute::{precompute, Token, Tokenizer};
use crate::tokenizer_combinators::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};
use bimap::BiBTreeMap;
use crate::groups;

type LLMToken = &'static [u8];

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct Grammar {
    pub productions: Vec<Production>,
    pub start_symbol: NonTerminal,
    pub terminal_map: BiBTreeMap<Terminal, TerminalID>,
    pub non_terminal_map: BiBTreeMap<NonTerminal, NonTerminalID>,
}

impl Debug for Grammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grammar:")?;
        writeln!(f, "  Start Symbol: {}", self.start_symbol.0)?;
        writeln!(f, "  Productions:")?;

        for production in &self.productions {
            write!(f, "    {} -> ", production.lhs.0)?;
            for (i, symbol) in production.rhs.iter().enumerate() {
                match symbol {
                    Symbol::Terminal(terminal) => {
                        write!(f, "{}", terminal.0)?;
                    }
                    Symbol::NonTerminal(non_terminal) => {
                        write!(f, "{}", non_terminal.0)?;
                    }
                }
                if i < production.rhs.len() - 1 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "  Terminal Map:")?;
        for (terminal, terminal_id) in &self.terminal_map {
            writeln!(f, "    {:?}: {}", terminal.0, terminal_id.0)?;
        }

        writeln!(f, "  Non-Terminal Map:")?;
        for (non_terminal, non_terminal_id) in &self.non_terminal_map {
            writeln!(f, "    {}: {}", non_terminal.0, non_terminal_id.0)?;
        }

        Ok(())
    }
}

impl Grammar {
    pub fn from_exprs(start_symbol: &str, exprs: Vec<(String, GrammarExpr)>) -> (Self, Regex) {
        let mut productions = Vec::new();
        let mut terminal_map = BiBTreeMap::new();
        let mut non_terminal_map = BiBTreeMap::new();
        let mut next_terminal_id = 0;
        let mut next_non_terminal_id = 0;
        let mut tokenizer_exprs = Vec::new();

        fn convert_expr(
            expr: &GrammarExpr,
            productions: &mut Vec<Production>,
            terminal_map: &mut BiBTreeMap<Terminal, TerminalID>,
            non_terminal_map: &mut BiBTreeMap<NonTerminal, NonTerminalID>,
            next_terminal_id: &mut usize,
            next_non_terminal_id: &mut usize,
            tokenizer_exprs: &mut Vec<(usize, Expr)>,
        ) -> Vec<Symbol> {
            match expr {
                GrammarExpr::Terminal(expr) => {
                    tokenizer_exprs.push((*next_terminal_id, expr.clone()));
                    let regex = expr.clone().build();
                    let mut regex_state = regex.init();
                    let u8set = regex_state.get_u8set();
                    let mut symbols = Vec::new();
                    for byte in u8set.iter() {
                        let terminal_str = String::from_utf8_lossy(std::slice::from_ref(&byte)).to_string();
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
                        sequence_symbols.extend(convert_expr(e, productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id, tokenizer_exprs));
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
                            rhs: convert_expr(expr, productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id, tokenizer_exprs),
                        });
                    }
                    productions.extend(choice_productions);

                    vec![Symbol::NonTerminal(NonTerminal(new_nonterminal))]
                }
                GrammarExpr::Optional(expr) => convert_expr(&GrammarExpr::choice(vec![expr.as_ref().clone(), GrammarExpr::sequence(vec![])]), productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id, tokenizer_exprs),
                GrammarExpr::Repeat(expr) => convert_expr(&GrammarExpr::optional(GrammarExpr::sequence(vec![expr.as_ref().clone(), GrammarExpr::repeat(expr.as_ref().clone())])), productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id, tokenizer_exprs),
            }
        }

        for (name, expr) in &exprs {
            let rhs = convert_expr(expr, &mut productions, &mut terminal_map, &mut non_terminal_map, &mut next_terminal_id, &mut next_non_terminal_id, &mut tokenizer_exprs);
            productions.push(Production {
                lhs: NonTerminal(name.clone()),
                rhs,
            });
        }

        productions.insert(0, Production {
            lhs: NonTerminal(start_symbol.to_string()),
            rhs: vec![Symbol::NonTerminal(NonTerminal(exprs.iter().next().map(|(name, _)| name.clone()).expect("Grammar must have at least one rule").clone()))],
        });

        if !non_terminal_map.contains_left(&NonTerminal(start_symbol.to_string())) {
            non_terminal_map.insert(NonTerminal(start_symbol.to_string()), NonTerminalID(next_non_terminal_id));
        }

        let tokenizer_exprs_vec: Vec<ExprGroup> = tokenizer_exprs.into_iter().map(|(_, expr)| non_greedy_group(expr)).collect();
        let tokenizer = groups(tokenizer_exprs_vec).build();

        (
            Self {
                productions,
                start_symbol: NonTerminal(start_symbol.to_string()),
                terminal_map: terminal_map.clone(),
                non_terminal_map,
            },
            tokenizer,
        )
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
        let states = vec![(parser.init_parse_state(), BTreeSet::from([StateID(tokenizer.initial_state_id())]))];
        Self {
            tokenizer,
            parser,
            precomputed,
            states,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::finite_automata::eat_u8;
    use crate::groups;
    use super::*;

    #[test]
    fn test_glr_parser_with_grammar_from_exprs() {
        let exprs: Vec<(String, GrammarExpr)> = vec![
            (
                "E".to_string(),
                GrammarExpr::choice(vec![
                    GrammarExpr::sequence(vec![
                        GrammarExpr::nonterminal("E"),
                        GrammarExpr::terminal(eat_u8(b'+')),
                        GrammarExpr::nonterminal("T"),
                    ]),
                    GrammarExpr::nonterminal("T"),
                ]),
            ),
            (
                "T".to_string(),
                GrammarExpr::choice(vec![
                    GrammarExpr::sequence(vec![
                        GrammarExpr::nonterminal("T"),
                        GrammarExpr::terminal(eat_u8(b'*')),
                        GrammarExpr::nonterminal("F"),
                    ]),
                    GrammarExpr::nonterminal("F"),
                ]),
            ),
            (
                "F".to_string(),
                GrammarExpr::choice(vec![
                    GrammarExpr::sequence(vec![
                        GrammarExpr::terminal(eat_u8(b'(')),
                        GrammarExpr::nonterminal("E"),
                        GrammarExpr::terminal(eat_u8(b')')),
                    ]),
                    GrammarExpr::terminal(eat_u8(b'i')),
                ]),
            ),
        ];

        let (grammar, tokenizer) = Grammar::from_exprs("S", exprs);
        dbg!(&grammar);
        let parser = generate_glr_parser(&grammar.productions);

        let tokenize = |input: &[u8], parser: &GLRParser, tokenizer: &Regex| -> Vec<TerminalID> {
            let mut regex_state = tokenizer.init();
            regex_state.execute(input);
            regex_state.matches.keys().copied().map(TerminalID).collect()
        };

        let valid_strings = [b"i".as_slice(), b"i+i", b"i*i", b"(i)", b"i+i*i", b"(i+i)*i"];
        let invalid_strings = [b"i+".as_slice(), b"i++i", b")"];

        for &input_str in &valid_strings {
            assert!(parser.parse(&tokenize(input_str, &parser, &tokenizer)).fully_matches(), "Failed to parse valid string: {:?}", input_str);
        }

        for &input_str in &invalid_strings {
            assert!(!parser.parse(&tokenize(input_str, &parser, &tokenizer)).fully_matches(), "Incorrectly parsed invalid string: {:?}", input_str);
        }
    }
}