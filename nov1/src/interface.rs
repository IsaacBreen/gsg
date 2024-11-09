use crate::finite_automata::{groups, non_greedy_group, ExprGroup, ExprGroups};
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

type LLMToken = &'static [u8];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarExpr {
    Literal(String),
    Ref(String),
    Sequence(Vec<GrammarExpr>),
    Choice(Vec<GrammarExpr>),
    Optional(Box<GrammarExpr>),
    Repeat(Box<GrammarExpr>),
}

impl GrammarExpr {
    pub fn literal(literal: &str) -> Self {
        GrammarExpr::Literal(literal.to_string())
    }

    pub fn r#ref(name: &str) -> Self {
        GrammarExpr::Ref(name.to_string())
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
    pub literal_map: BTreeMap<String, String>,
    pub terminal_name_to_group_id: BiBTreeMap<String, usize>,
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

        writeln!(f, "  Literal Map:")?;
        for (literal, mangled_name) in &self.literal_map {
            writeln!(f, "    {:?}: {}", literal, mangled_name)?;
        }

        writeln!(f, "  Terminal Name to Group ID Map:")?;
        for (name, group_id) in &self.terminal_name_to_group_id {
            writeln!(f, "    {:?}: {}", name, group_id)?;
        }

        Ok(())
    }
}

impl Grammar {
    pub fn from_exprs(start_symbol: &str, exprs: Vec<(String, GrammarExpr)>, tokens: BTreeMap<String, Expr>) -> (Self, Regex, ExprGroups) {
        let mut productions = Vec::new();
        let mut literal_map = BTreeMap::new();
        let mut terminal_name_to_group_id = BiBTreeMap::new();
        let mut tokenizer_exprs = Vec::new();
        let mut next_terminal_id = 0;

        for (name, expr) in &tokens {
            terminal_name_to_group_id.insert(name.clone(), next_terminal_id);
            tokenizer_exprs.push((next_terminal_id, expr.clone()));
            next_terminal_id += 1;
        }

        fn convert_expr(
            expr: &GrammarExpr,
            productions: &mut Vec<Production>,
            non_terminal_map: &mut BiBTreeMap<NonTerminal, NonTerminalID>,
            next_non_terminal_id: &mut usize,
            literal_map: &mut BTreeMap<String, String>,
            tokens: &BTreeMap<String, Expr>,
        ) -> Vec<Symbol> {
            match expr {
                GrammarExpr::Literal(literal) => {
                    let mangled_name = Grammar::mangle_literal(literal, tokens);
                    literal_map.insert(literal.clone(), mangled_name.clone());
                    vec![Symbol::Terminal(Terminal(mangled_name))]
                }
                GrammarExpr::Ref(name) => {
                    if tokens.contains_key(name) {
                        vec![Symbol::Terminal(Terminal(name.clone()))]
                    } else {
                        if !non_terminal_map.contains_left(&NonTerminal(name.clone())) {
                            non_terminal_map.insert(NonTerminal(name.clone()), NonTerminalID(*next_non_terminal_id));
                            *next_non_terminal_id += 1;
                        }
                        vec![Symbol::NonTerminal(NonTerminal(name.clone()))]
                    }
                }
                GrammarExpr::Sequence(exprs) => exprs.iter().flat_map(|e| convert_expr(e, productions, non_terminal_map, next_non_terminal_id, literal_map, tokens)).collect(),
                GrammarExpr::Choice(exprs) => exprs.iter().flat_map(|e| convert_expr(e, productions, non_terminal_map, next_non_terminal_id, literal_map, tokens)).collect(),
                GrammarExpr::Optional(expr) => {
                    let mut result = convert_expr(expr, productions, non_terminal_map, next_non_terminal_id, literal_map, tokens);
                    result.push(Symbol::Terminal(Terminal("ε".to_string())));
                    result
                }
                GrammarExpr::Repeat(expr) => convert_expr(expr, productions, non_terminal_map, next_non_terminal_id, literal_map, tokens),
            }
        }

        let mut non_terminal_map = BiBTreeMap::new();
        let mut next_non_terminal_id = 0;

        for (name, expr) in &exprs {
            let rhs = convert_expr(expr, &mut productions, &mut non_terminal_map, &mut next_non_terminal_id, &mut literal_map, &tokens);
            productions.push(Production {
                lhs: NonTerminal(name.clone()),
                rhs,
            });
        }

        if !productions.iter().any(|p| p.lhs == NonTerminal(start_symbol.to_string())) {
            productions.insert(0, Production {
                lhs: NonTerminal(start_symbol.to_string()),
                rhs: vec![Symbol::NonTerminal(NonTerminal(exprs.iter().next().map(|(name, _)| name.clone()).unwrap_or_else(|| panic!("Grammar must have at least one rule")).clone()))],
            });
        }

        let tokenizer_exprs_vec: Vec<ExprGroup> = tokenizer_exprs.into_iter().map(|(_, expr)| non_greedy_group(expr)).collect();
        let tokenizer_expr_groups = groups(tokenizer_exprs_vec);
        let tokenizer = tokenizer_expr_groups.clone().build();

        (
            Self {
                productions,
                start_symbol: NonTerminal(start_symbol.to_string()),
                literal_map,
                terminal_name_to_group_id,
            },
            tokenizer,
            tokenizer_expr_groups,
        )
    }

    fn mangle_literal(literal: &str, tokens: &BTreeMap<String, Expr>) -> String {
        let mut mangled_name = literal.to_string();
        let mut i = 0;
        while tokens.contains_key(&mangled_name) {
            mangled_name = format!("{}__literal_{}", literal, i);
            i += 1;
        }
        mangled_name
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
        let tokens = BTreeMap::from([
            ("plus".to_string(), eat_u8(b'+')),
            ("star".to_string(), eat_u8(b'*')),
            ("lparen".to_string(), eat_u8(b'(')),
            ("rparen".to_string(), eat_u8(b')')),
            ("i".to_string(), eat_u8(b'i')),
        ]);

        let exprs: Vec<(String, GrammarExpr)> = vec![
            (
                "E".to_string(),
                GrammarExpr::choice(vec![
                    GrammarExpr::sequence(vec![
                        GrammarExpr::r#ref("E"),
                        GrammarExpr::r#ref("plus"),
                        GrammarExpr::r#ref("T"),
                    ]),
                    GrammarExpr::r#ref("T"),
                ]),
            ),
            (
                "T".to_string(),
                GrammarExpr::choice(vec![
                    GrammarExpr::sequence(vec![
                        GrammarExpr::r#ref("T"),
                        GrammarExpr::r#ref("star"),
                        GrammarExpr::r#ref("F"),
                    ]),
                    GrammarExpr::r#ref("F"),
                ]),
            ),
            (
                "F".to_string(),
                GrammarExpr::choice(vec![
                    GrammarExpr::sequence(vec![
                        GrammarExpr::r#ref("lparen"),
                        GrammarExpr::r#ref("E"),
                        GrammarExpr::r#ref("rparen"),
                    ]),
                    GrammarExpr::r#ref("i"),
                ]),
            ),
        ];

        let (grammar, tokenizer, _) = Grammar::from_exprs("S", exprs, tokens);
        let parser = generate_glr_parser(&grammar.productions);

        let tokenize = |input: &[u8], parser: &GLRParser, tokenizer: &Regex, grammar: &Grammar| -> Vec<TerminalID> {
            let mut regex_state = tokenizer.init();
            regex_state.execute(input);

            let mut result = Vec::new();
            for group_id in regex_state.matches.keys() {
                if let Some(token_name) = grammar.terminal_name_to_group_id.get_by_right(group_id) {
                    if let Some(&terminal_id) = parser.terminal_map.get_by_left(&Terminal(token_name.clone())) {
                        result.push(terminal_id);
                    } else {
                        panic!("Token name '{}' not found in terminal map", token_name);
                    }
                }
            }
            result
        };

        let valid_strings = [b"i".as_slice(), b"i+i", b"i*i", b"(i)", b"i+i*i", b"(i+i)*i"];
        let invalid_strings = [b"i+".as_slice(), b"i++i", b")"];

        for &input_str in &valid_strings {
            assert!(parser.parse(&tokenize(input_str, &parser, &tokenizer, &grammar)).fully_matches(), "Failed to parse valid string: {:?} ({:?})", input_str, String::from_utf8_lossy(input_str));
        }

        for &input_str in &invalid_strings {
            assert!(!parser.parse(&tokenize(input_str, &parser, &tokenizer, &grammar)).fully_matches(), "Incorrectly parsed invalid string: {:?}", input_str);
        }
    }
}