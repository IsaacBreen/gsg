use crate::finite_automata::{greedy_group, groups, non_greedy_group, ExprGroup, ExprGroups};
use crate::finite_automata::{Expr, Regex};
use crate::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use crate::glr::parser::{GLRParser, ParseState};
use crate::glr::table::{generate_glr_parser, NonTerminalID, StateID, TerminalID};
use crate::precompute::{precompute, precompute_add_incomplete_token, Token, Tokenizer};
use bimap::BiBTreeMap;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};
use crate::constraint::GrammarConstraintState;

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

pub fn literal(literal: &str) -> GrammarExpr {
    GrammarExpr::Literal(literal.to_string())
}

pub fn r#ref(name: &str) -> GrammarExpr {
    GrammarExpr::Ref(name.to_string())
}

pub fn sequence(exprs: Vec<GrammarExpr>) -> GrammarExpr {
    GrammarExpr::Sequence(exprs)
}

pub fn choice(exprs: Vec<GrammarExpr>) -> GrammarExpr {
    GrammarExpr::Choice(exprs)
}

pub fn optional(expr: GrammarExpr) -> GrammarExpr {
    GrammarExpr::Optional(Box::new(expr))
}

pub fn repeat(expr: GrammarExpr) -> GrammarExpr {
    GrammarExpr::Repeat(Box::new(expr))
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
            // TODO: a lot of these conversion are wrong. Look at easy_interface.rs for examples of how to do it right.
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
                GrammarExpr::Choice(exprs) => {
                    let new_nonterminal = format!("Choice{}", *next_non_terminal_id);
                    let nt = NonTerminal(new_nonterminal.clone());

                    if !non_terminal_map.contains_left(&nt) {
                        non_terminal_map.insert(nt.clone(), NonTerminalID(*next_non_terminal_id));
                        *next_non_terminal_id += 1;
                    }

                    for expr in exprs {
                        let rhs = convert_expr(expr, productions, non_terminal_map, next_non_terminal_id, literal_map, tokens);
                        productions.push(Production {
                            lhs: nt.clone(),
                            rhs,
                        });
                    }

                    vec![Symbol::NonTerminal(nt)]
                }
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

        let tokenizer_exprs_vec: Vec<ExprGroup> = tokenizer_exprs.into_iter().map(|(_, expr)| greedy_group(expr)).collect();
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

impl<T: Tokenizer> GrammarConstraintState<T> {
    pub fn new_from_grammar(tokenizer: T, grammar: Grammar, llm_tokens: &[LLMToken]) -> Self {
        // TODO: make sure the start nonterm is unique.
        let parser = generate_glr_parser(&grammar.productions, &Production { lhs: NonTerminal("S'".to_string()), rhs: vec![Symbol::NonTerminal(NonTerminal(grammar.start_symbol.0.clone()))] });
        let precomputed = precompute(&tokenizer, llm_tokens);
        let precomputed = precompute_add_incomplete_token(&tokenizer, precomputed);
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
    use super::*;
    use crate::finite_automata::eat_u8;

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
                choice(vec![
                    sequence(vec![
                        r#ref("E"),
                        r#ref("plus"),
                        r#ref("T"),
                    ]),
                    r#ref("T"),
                ]),
            ),
            (
                "T".to_string(),
                choice(vec![
                    sequence(vec![
                        r#ref("T"),
                        r#ref("star"),
                        r#ref("F"),
                    ]),
                    r#ref("F"),
                ]),
            ),
            (
                "F".to_string(),
                choice(vec![
                    sequence(vec![
                        r#ref("lparen"),
                        r#ref("E"),
                        r#ref("rparen"),
                    ]),
                    r#ref("i"),
                ]),
            ),
        ];

        let (grammar, tokenizer, _) = Grammar::from_exprs("S", exprs, tokens);
        let parser = generate_glr_parser(&grammar.productions, &Production { lhs: NonTerminal("S'".to_string()), rhs: vec![Symbol::NonTerminal(NonTerminal(grammar.start_symbol.0.clone()))] });

        let tokenize = |input: &[u8], parser: &GLRParser, tokenizer: &Regex, grammar: &Grammar| -> Vec<TerminalID> {
            let mut tokenizer_state = tokenizer.init();
            let tokenizer_matches = tokenizer_state.greedy_find_all(input, true);

            let mut result = Vec::new();
            for m in tokenizer_matches {
                let group_id = m.group_id;
                let token_name = grammar.terminal_name_to_group_id.get_by_right(&group_id).unwrap();
                let terminal_id = parser.terminal_map.get_by_left(&Terminal(token_name.clone())).unwrap();
                result.push(*terminal_id);
            }
            result
        };

        let valid_strings = [b"i".as_slice(), b"i+i", b"i*i", b"(i)", b"i+i*i", b"(i+i)*i", b"(((i))+(i)*i)+(((((i)))))"];
        let invalid_strings = [b"i+".as_slice(), b"i++i", b")"];

        for &input_str in &valid_strings {
            let tokens = tokenize(input_str, &parser, &tokenizer, &grammar);
            assert!(parser.parse(&tokens).fully_matches(), "Failed to parse valid input: {:?}, string: {:?}, tokens: {:?}", input_str, String::from_utf8_lossy(input_str), tokens);
        }

        for &input_str in &invalid_strings {
            let tokens = tokenize(input_str, &parser, &tokenizer, &grammar);
            assert!(!parser.parse(&tokens).fully_matches(), "Incorrectly parsed invalid input: {:?}, string: {:?}, tokens: {:?}", input_str, String::from_utf8_lossy(input_str), tokens);
        }

        let llm_tokens = &[b"i".as_slice(), b"+", b"*", b"(", b")", b"(i", b"+i"];
        let mut grammar_state = GrammarConstraintState::new_from_grammar(tokenizer, grammar, llm_tokens);

        grammar_state.commit_many(&[b"(i".as_slice(), b"+i", b"*", b"i"]);

        let mask = grammar_state.get_mask();
        assert_eq!(mask, BTreeSet::from([b"+".as_slice(), b"*", b")", b"+i)"]));
    }
}