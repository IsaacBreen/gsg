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
    pub terminal_map: BiBTreeMap<Terminal, TerminalID>,
    pub non_terminal_map: BiBTreeMap<NonTerminal, NonTerminalID>,
    pub literal_map: BTreeMap<String, String>, 
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

        writeln!(f, "  Literal Map:")?;
        for (literal, mangled_name) in &self.literal_map {
            writeln!(f, "    {:?}: {}", literal, mangled_name)?;
        }

        Ok(())
    }
}

impl Grammar {
    pub fn from_exprs(start_symbol: &str, exprs: Vec<(String, GrammarExpr)>, tokens: BTreeMap<String, Expr>) -> (Self, Regex) {
        let mut productions = Vec::new();
        let mut terminal_map = BiBTreeMap::new();
        let mut non_terminal_map = BiBTreeMap::new();
        let mut literal_map = BTreeMap::new();
        let mut next_terminal_id = 0;
        let mut next_non_terminal_id = 0;
        let mut tokenizer_exprs = Vec::new();

        for (name, expr) in &tokens {
            let terminal = Terminal(name.clone());
            terminal_map.insert(terminal.clone(), TerminalID(next_terminal_id));
            tokenizer_exprs.push((next_terminal_id, expr.clone()));
            productions.push(Production {
                lhs: NonTerminal(name.clone()),
                rhs: vec![Symbol::Terminal(terminal)],
            });
            next_terminal_id += 1;
        }

        fn convert_expr(
            expr: &GrammarExpr,
            productions: &mut Vec<Production>,
            terminal_map: &mut BiBTreeMap<Terminal, TerminalID>,
            non_terminal_map: &mut BiBTreeMap<NonTerminal, NonTerminalID>,
            next_terminal_id: &mut usize,
            next_non_terminal_id: &mut usize,
            tokenizer_exprs: &mut Vec<(usize, Expr)>,
            literal_map: &mut BTreeMap<String, String>,
            tokens: &BTreeMap<String, Expr>,
        ) -> Vec<Symbol> {
            match expr {
                GrammarExpr::Literal(literal) => {
                    let mangled_name = Grammar::mangle_literal(literal, terminal_map);
                    literal_map.insert(literal.clone(), mangled_name.clone());

                    if !terminal_map.contains_left(&Terminal(mangled_name.clone())) {
                        terminal_map.insert(Terminal(mangled_name.clone()), TerminalID(*next_terminal_id));
                        *next_terminal_id += 1;
                    }

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
                GrammarExpr::Sequence(exprs) => {
                    let mut sequence_symbols = Vec::new();
                    for e in exprs {
                        sequence_symbols.extend(convert_expr(e, productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id, tokenizer_exprs, literal_map, tokens));
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
                            rhs: convert_expr(expr, productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id, tokenizer_exprs, literal_map, tokens),
                        });
                    }
                    productions.extend(choice_productions);

                    vec![Symbol::NonTerminal(NonTerminal(new_nonterminal))]
                }
                GrammarExpr::Optional(expr) => convert_expr(&GrammarExpr::choice(vec![expr.as_ref().clone(), GrammarExpr::sequence(vec![])]), productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id, tokenizer_exprs, literal_map, tokens),
                GrammarExpr::Repeat(expr) => convert_expr(&GrammarExpr::optional(GrammarExpr::sequence(vec![expr.as_ref().clone(), GrammarExpr::repeat(expr.as_ref().clone())])), productions, terminal_map, non_terminal_map, next_terminal_id, next_non_terminal_id, tokenizer_exprs, literal_map, tokens),
            }
        }

        for (name, expr) in &exprs {
            let rhs = convert_expr(expr, &mut productions, &mut terminal_map, &mut non_terminal_map, &mut next_terminal_id, &mut next_non_terminal_id, &mut tokenizer_exprs, &mut literal_map, &tokens);
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
                literal_map,
            },
            tokenizer,
        )
    }

    fn mangle_literal(literal: &str, terminal_map: &BiBTreeMap<Terminal, TerminalID>) -> String {
        let mut mangled_name = literal.to_string();
        let mut i = 0;
        while terminal_map.contains_left(&Terminal(mangled_name.clone())) {
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

        let (grammar, tokenizer) = Grammar::from_exprs("S", exprs, tokens);
        dbg!(&grammar);
        let parser = generate_glr_parser(&grammar.productions);

        let tokenize = |input: &[u8], parser: &GLRParser, tokenizer: &Regex| -> Vec<TerminalID> {
            let mut regex_state = tokenizer.init();
            regex_state.execute(input);
            regex_state.matches.keys().copied().map(|id| TerminalID(id)).collect()
        };

        let valid_strings = [b"i".as_slice(), b"i+i", b"i*i", b"(i)", b"i+i*i", b"(i+i)*i"];
        let invalid_strings = [b"i+".as_slice(), b"i++i", b")"];

        for &input_str in &valid_strings {
            assert!(parser.parse(&tokenize(input_str, &parser, &tokenizer)).fully_matches(), "Failed to parse valid string: {:?} ({:?})", input_str, String::from_utf8_lossy(input_str));
        }

        for &input_str in &invalid_strings {
            assert!(!parser.parse(&tokenize(input_str, &parser, &tokenizer)).fully_matches(), "Incorrectly parsed invalid string: {:?}", input_str);
        }
    }
}