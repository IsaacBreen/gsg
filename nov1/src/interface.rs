use crate::finite_automata::{greedy_group, groups, non_greedy_group, ExprGroup, ExprGroups};
use crate::finite_automata::{Expr, Regex};
use crate::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use crate::glr::parser::{GLRParser, ParseState};
use crate::glr::table::{assign_non_terminal_ids, generate_glr_parser, generate_glr_parser_with_maps, NonTerminalID, StateID, TerminalID};
use crate::precompute::{precompute, precompute_add_incomplete_token, Token, Tokenizer};
use bimap::BiBTreeMap;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};
use crate::constraint::{GrammarConstraint, LLMTokenID, convert_precomputed_to_llm_token_ids};

type LLMToken = &'static [u8];

#[derive(Clone)]
pub struct Grammar<'a, T> {
    pub productions: Vec<Production>,
    pub start_production_id: usize,
    pub literal_map: BTreeMap<String, String>,
    pub terminal_name_to_group_id: BiBTreeMap<String, usize>,
    pub tokenizer: &'a T,
}

impl<T> Debug for Grammar<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grammar:")?;
        writeln!(f, "  Start Production ID: {}", self.start_production_id)?;
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

        writeln!(f, "  Terminal Map (name to group ID):")?;
        for (name, group_id) in &self.terminal_name_to_group_id {
            writeln!(f, "    {:?}: {}", name, group_id)?;
        }

        Ok(())
    }
}

impl<T> Grammar<'_, T> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarExpr {
    RegexExpr(Expr),
    Ref(String),
    Sequence(Vec<GrammarExpr>),
    Choice(Vec<GrammarExpr>),
    Optional(Box<GrammarExpr>),
    Repeat(Box<GrammarExpr>),
}

pub fn regex(expr: Expr) -> GrammarExpr {
    GrammarExpr::RegexExpr(expr)
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

impl<T> Grammar<'_, T> {
    pub fn glr_parser(&self) -> GLRParser {
        generate_glr_parser(&self.productions, self.start_production_id)
    }
}

impl Grammar<'_, Regex> {
    /// Constructs a `Grammar` and `Regex` tokenizer from a list of grammar expressions.
    /// The first non-terminal in the list is treated as the start symbol.
    pub fn from_exprs(exprs: Vec<(String, GrammarExpr)>) -> Self {
        let mut productions = Vec::new();
        let mut literal_map = BTreeMap::new();
        let mut terminal_name_to_group_id = BiBTreeMap::new();
        let mut terminal_expr_to_group_id = BiBTreeMap::new();
        let mut next_terminal_id = 0;

        // Add a start production.
        // TODO: make sure the start production name is not already taken. Use a unique name generator function.
        productions.push(Production {
            lhs: NonTerminal("start".to_string()),
            rhs: vec![Symbol::NonTerminal(NonTerminal(exprs[0].0.clone()))],
        });

        fn convert_expr(
            expr: &GrammarExpr,
            productions: &mut Vec<Production>,
            non_terminal_map: &mut BiBTreeMap<NonTerminal, NonTerminalID>,
            next_non_terminal_id: &mut usize,
            literal_map: &mut BTreeMap<String, String>,
            tokens: &mut BTreeMap<String, Expr>,
            terminal_name_to_group_id: &mut BiBTreeMap<String, usize>,
            terminal_expr_to_group_id: &mut BiBTreeMap<Expr, usize>,
            next_terminal_id: &mut usize,
        ) -> Vec<Symbol> {
            // TODO: define a function that makes us a unique name for an internal rule, with an appropriate prefix.
            //  e.g. Option0, Repeat0, etc. Make sure there's no existing rule with that name (and there won't be one later either).
            //  i.e. collect all nonterminals in the grammar upfront and pass it to convert_expr.
            match expr {
                GrammarExpr::RegexExpr(regex_expr) => {
                    // TODO: what if this is already in the map (e.g. the user happens to create a rule with name `__regex_0`?
                    //  We need to generate a unique regex name.
                    if let Some(terminal_id) = terminal_expr_to_group_id.get_by_left(&regex_expr) {
                        vec![Symbol::Terminal(Terminal(format!("__regex_{}", terminal_id)))]
                    } else {
                        // Create a unique terminal name for this regex expression
                        let terminal_id = *next_terminal_id;
                        let terminal_name = format!("__regex_{}", terminal_id);
                        terminal_name_to_group_id.insert(terminal_name.clone(), terminal_id);
                        terminal_expr_to_group_id.insert(regex_expr.clone(), terminal_id);
                        tokens.insert(terminal_name.clone(), regex_expr.clone());
                        *next_terminal_id += 1;
                        vec![Symbol::Terminal(Terminal(terminal_name))]
                    }
                }
                GrammarExpr::Ref(name) => {
                    vec![Symbol::NonTerminal(NonTerminal(name.clone()))]
                }
                GrammarExpr::Sequence(exprs) => exprs
                    .iter()
                    .flat_map(|e| {
                        convert_expr(
                            e,
                            productions,
                            non_terminal_map,
                            next_non_terminal_id,
                            literal_map,
                            tokens,
                            terminal_name_to_group_id,
                            terminal_expr_to_group_id,
                            next_terminal_id,
                        )
                    })
                    .collect(),
                GrammarExpr::Choice(exprs) => {
                    let new_nonterminal = format!("Choice{}", *next_non_terminal_id);
                    let nt = NonTerminal(new_nonterminal.clone());

                    // TODO: what if this is already in the map (e.g. the user happens to create a rule with name `Choice0`?
                    //  We need to generate a unique nonterminal name.
                    if !non_terminal_map.contains_left(&nt) {
                        non_terminal_map.insert(nt.clone(), NonTerminalID(*next_non_terminal_id));
                        *next_non_terminal_id += 1;
                    }

                    for expr in exprs {
                        let rhs = convert_expr(
                            expr,
                            productions,
                            non_terminal_map,
                            next_non_terminal_id,
                            literal_map,
                            tokens,
                            terminal_name_to_group_id,
                            terminal_expr_to_group_id,
                            next_terminal_id,
                        );
                        productions.push(Production {
                            lhs: nt.clone(),
                            rhs,
                        });
                    }

                    vec![Symbol::NonTerminal(nt)]
                }
                GrammarExpr::Optional(expr) => {
                    // TODO: name the internal rule here Option{} or something rather than Choice{}.
                    convert_expr(
                        &GrammarExpr::Choice(vec![*expr.clone(), GrammarExpr::Sequence(vec![])]),
                        productions,
                        non_terminal_map,
                        next_non_terminal_id,
                        literal_map,
                        tokens,
                        terminal_name_to_group_id,
                        terminal_expr_to_group_id,
                        next_terminal_id,
                    )
                }
                GrammarExpr::Repeat(expr) => {
                    // TODO: same as above, make sure it's unique.
                    let nonterminal_id = *next_non_terminal_id;
                    let nonterminal_name = format!("Repeat{}", nonterminal_id);
                    non_terminal_map.insert(NonTerminal(nonterminal_name.clone()), NonTerminalID(nonterminal_id));
                    *next_non_terminal_id += 1;
                    let rhs = convert_expr(
                        expr,
                        productions,
                        non_terminal_map,
                        next_non_terminal_id,
                        literal_map,
                        tokens,
                        terminal_name_to_group_id,
                        terminal_expr_to_group_id,
                        next_terminal_id,
                    );
                    productions.push(Production {
                        lhs: NonTerminal(nonterminal_name.clone()),
                        rhs,
                    });
                    vec![Symbol::NonTerminal(NonTerminal(nonterminal_name))]
                }
            }
        }

        let mut non_terminal_map = BiBTreeMap::new();
        let mut next_non_terminal_id = 0;
        let mut tokens = BTreeMap::new();

        for (name, expr) in &exprs {
            let rhs = convert_expr(
                expr,
                &mut productions,
                &mut non_terminal_map,
                &mut next_non_terminal_id,
                &mut literal_map,
                &mut tokens,
                &mut terminal_name_to_group_id,
                &mut terminal_expr_to_group_id,
                &mut next_terminal_id,
            );
            productions.push(Production {
                lhs: NonTerminal(name.clone()),
                rhs,
            });
        }

        let tokenizer_exprs_vec: Vec<ExprGroup> = tokens
            .into_iter()
            .map(|(_, expr)| greedy_group(expr))
            .collect();
        let tokenizer_expr_groups = groups(tokenizer_exprs_vec);
        let tokenizer = tokenizer_expr_groups.clone().build();

        Self {
            productions,
            start_production_id: 0,
            literal_map,
            terminal_name_to_group_id,
            tokenizer: &tokenizer,
        }
    }
}

impl<T: Tokenizer> GrammarConstraint<'_, T> {
    pub fn from_grammar(grammar: Grammar<T>, llm_tokens: &[LLMToken]) -> Self {
        let terminal_map = grammar.terminal_name_to_group_id.iter().map(|(name, group_id)| { (Terminal(name.clone()), TerminalID(*group_id)) }).collect();
        let non_terminal_map = assign_non_terminal_ids(&grammar.productions);
        let parser = generate_glr_parser_with_maps(&grammar.productions, grammar.start_production_id, terminal_map, non_terminal_map);

        let mut llm_token_to_id = BTreeMap::new();
        let mut llm_token_id_to_token = BTreeMap::new();
        for (i, &token) in llm_tokens.iter().enumerate() {
            let id = LLMTokenID(i);
            llm_token_to_id.insert(token, id);
            llm_token_id_to_token.insert(id, token);
        }

        let precomputed = precompute(grammar.tokenizer, llm_tokens);
        let precomputed = precompute_add_incomplete_token(grammar.tokenizer, precomputed);
        let precomputed = convert_precomputed_to_llm_token_ids(precomputed, &llm_token_to_id);

        Self {
            tokenizer: grammar.tokenizer,
            parser,
            precomputed,
            llm_token_to_id,
            llm_token_id_to_token,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_automata::eat_u8;
    use crate::glr::table::generate_glr_parser;

    #[test]
    fn test_grammar_from_exprs() {
        let exprs = vec![
            (
                "E".to_string(),
                choice(vec![
                    sequence(vec![
                        r#ref("E"),
                        regex(eat_u8(b'+')),
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
                        regex(eat_u8(b'*')),
                        r#ref("F"),
                    ]),
                    r#ref("F"),
                ]),
            ),
            (
                "F".to_string(),
                choice(vec![
                    sequence(vec![
                        regex(eat_u8(b'(')),
                        r#ref("E"),
                        regex(eat_u8(b')')),
                    ]),
                    regex(eat_u8(b'i')),
                ]),
            ),
        ];

        let grammar = Grammar::from_exprs(exprs.clone());
        dbg!(&grammar);

        let parser = grammar.glr_parser();
        dbg!(&parser);

        let llm_tokens = &[b"i".as_slice(), b"+", b"*", b"(", b")", b"(i", b"+i"];
        let grammar_constraint = GrammarConstraint::from_grammar(grammar, llm_tokens);
        let mut grammar_constraint_state = grammar_constraint.init();

        #[macro_export]
        macro_rules! llm_tokens {
            ($grammar_state:expr, $($token:expr),* $(,)?) => {
                vec![
                    $(
                        *$grammar_state.llm_token_to_id.get($token.as_slice()).unwrap(),
                    )*
                ]
            };
        }

        // Get the mask.
        // The valid LLM tokens initially are ["i", "(", "(i"].
        let mask = grammar_constraint_state.get_mask();
        let expected_mask: BTreeSet<_> = llm_tokens!(grammar_constraint, b"i", b"(", b"(i").into_iter().collect();
        assert_eq!(mask, expected_mask);

        // Simulate generating from a LLM with the grammar constraint.
        // We may have some 'prefill' we want to pass to the parser before we generate the first new LLM token.
        // Let's say the prefill is "(i+i*i".
        // This would be best tokenized as ["(i", "+", "i", "*", "i"].
        //
        // Take note of the ambiguity in the LLM tokens; we could the prefill as ["(", "i", "+", "i", "*", "i"],
        // i.e. break the "(i" token into "(" and "i". But that's a waste of a token.
        // A good LLM tokenizer would greedily emit the longest possible token at each step.
        let prefill = llm_tokens!(grammar_constraint, b"(i", b"+i", b"*", b"i");
        grammar_constraint_state.commit_many(&prefill);

        // Get the mask.
        // The valid LLM tokens right now are ["+", "*", ")", "+i)"].
        let mask = grammar_constraint_state.get_mask();
        let expected_mask: BTreeSet<_> = llm_tokens!(grammar_constraint, b"+", b"*", b")", b"+i").into_iter().collect();
        assert_eq!(mask, expected_mask);
    }
}