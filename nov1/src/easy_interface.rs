use crate::finite_automata::{groups, non_greedy_group, ExprGroup, ExprGroups};
use crate::finite_automata::{Expr, Regex};
use crate::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use crate::glr::parser::{GLRParser, ParseState};
use crate::glr::table::{generate_glr_parser, NonTerminalID, StateID, TerminalID};
use crate::precompute::{precompute, Token, Tokenizer};
use bimap::BiBTreeMap;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};
use crate::constraint::GrammarConstraintState;
use crate::interface::Grammar;

type LLMToken = &'static [u8];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EasyGrammarExpr {
    RegexExpr(Expr),
    Ref(String),
    Sequence(Vec<EasyGrammarExpr>),
    Choice(Vec<EasyGrammarExpr>),
    Optional(Box<EasyGrammarExpr>),
    Repeat(Box<EasyGrammarExpr>),
}

pub fn regex(expr: Expr) -> EasyGrammarExpr {
    EasyGrammarExpr::RegexExpr(expr)
}

pub fn r#ref(name: &str) -> EasyGrammarExpr {
    EasyGrammarExpr::Ref(name.to_string())
}

pub fn sequence(exprs: Vec<EasyGrammarExpr>) -> EasyGrammarExpr {
    EasyGrammarExpr::Sequence(exprs)
}

pub fn choice(exprs: Vec<EasyGrammarExpr>) -> EasyGrammarExpr {
    EasyGrammarExpr::Choice(exprs)
}

pub fn optional(expr: EasyGrammarExpr) -> EasyGrammarExpr {
    EasyGrammarExpr::Optional(Box::new(expr))
}

pub fn repeat(expr: EasyGrammarExpr) -> EasyGrammarExpr {
    EasyGrammarExpr::Repeat(Box::new(expr))
}

impl Grammar {
    /// Constructs a `Grammar` and `Regex` tokenizer from a list of easy grammar expressions.
    /// The first non-terminal in the list is treated as the start symbol.
    pub fn from_easy_exprs(exprs: Vec<(String, EasyGrammarExpr)>) -> (Self, Regex, ExprGroups) {
        let mut productions = Vec::new();
        let mut literal_map = BTreeMap::new();
        let mut terminal_name_to_group_id = BiBTreeMap::new();
        let mut next_terminal_id = 0;

        fn convert_easy_expr(
            expr: &EasyGrammarExpr,
            productions: &mut Vec<Production>,
            non_terminal_map: &mut BiBTreeMap<NonTerminal, NonTerminalID>,
            next_non_terminal_id: &mut usize,
            literal_map: &mut BTreeMap<String, String>,
            tokens: &mut BTreeMap<String, Expr>,
            terminal_name_to_group_id: &mut BiBTreeMap<String, usize>,
            next_terminal_id: &mut usize,
        ) -> Vec<Symbol> {
            match expr {
                EasyGrammarExpr::RegexExpr(regex_expr) => {
                    // Create a unique terminal name for this regex expression
                    let terminal_name = format!("__regex_{}", *next_terminal_id);
                    terminal_name_to_group_id.insert(terminal_name.clone(), *next_terminal_id);
                    tokens.insert(terminal_name.clone(), regex_expr.clone());
                    *next_terminal_id += 1;
                    vec![Symbol::Terminal(Terminal(terminal_name))]
                }
                EasyGrammarExpr::Ref(name) => {
                    vec![Symbol::NonTerminal(NonTerminal(name.clone()))]
                }
                EasyGrammarExpr::Sequence(exprs) => exprs
                    .iter()
                    .flat_map(|e| {
                        convert_easy_expr(
                            e,
                            productions,
                            non_terminal_map,
                            next_non_terminal_id,
                            literal_map,
                            tokens,
                            terminal_name_to_group_id,
                            next_terminal_id,
                        )
                    })
                    .collect(),
                EasyGrammarExpr::Choice(exprs) => {
                    let new_nonterminal = format!("Choice{}", *next_non_terminal_id);
                    let nt = NonTerminal(new_nonterminal.clone());

                    if !non_terminal_map.contains_left(&nt) {
                        non_terminal_map.insert(nt.clone(), NonTerminalID(*next_non_terminal_id));
                        *next_non_terminal_id += 1;
                    }

                    for expr in exprs {
                        let rhs = convert_easy_expr(
                            expr,
                            productions,
                            non_terminal_map,
                            next_non_terminal_id,
                            literal_map,
                            tokens,
                            terminal_name_to_group_id,
                            next_terminal_id,
                        );
                        productions.push(Production {
                            lhs: nt.clone(),
                            rhs,
                        });
                    }

                    vec![Symbol::NonTerminal(nt)]
                }
                EasyGrammarExpr::Optional(expr) => {
                    let mut result = convert_easy_expr(
                        expr,
                        productions,
                        non_terminal_map,
                        next_non_terminal_id,
                        literal_map,
                        tokens,
                        terminal_name_to_group_id,
                        next_terminal_id,
                    );
                    result.push(Symbol::Terminal(Terminal("ε".to_string())));
                    result
                }
                EasyGrammarExpr::Repeat(expr) => {
                    convert_easy_expr(
                        expr,
                        productions,
                        non_terminal_map,
                        next_non_terminal_id,
                        literal_map,
                        tokens,
                        terminal_name_to_group_id,
                        next_terminal_id,
                    )
                }
            }
        }

        let mut non_terminal_map = BiBTreeMap::new();
        let mut next_non_terminal_id = 0;
        let mut tokens = BTreeMap::new();

        for (name, expr) in &exprs {
            let rhs = convert_easy_expr(
                expr,
                &mut productions,
                &mut non_terminal_map,
                &mut next_non_terminal_id,
                &mut literal_map,
                &mut tokens,
                &mut terminal_name_to_group_id,
                &mut next_terminal_id,
            );
            productions.push(Production {
                lhs: NonTerminal(name.clone()),
                rhs,
            });
        }

        // The start symbol is the first non-terminal in the list
        let start_symbol = NonTerminal(exprs.first().unwrap().0.clone());

        let tokenizer_exprs_vec: Vec<ExprGroup> = tokens
            .into_iter()
            .map(|(_, expr)| non_greedy_group(expr))
            .collect();
        let tokenizer_expr_groups = groups(tokenizer_exprs_vec);
        let tokenizer = tokenizer_expr_groups.clone().build();

        (
            Self {
                productions,
                start_symbol,
                literal_map,
                terminal_name_to_group_id,
            },
            tokenizer,
            tokenizer_expr_groups,
        )
    }
}

impl GrammarConstraintState<Regex> {
    /// Constructs a `GrammarConstraintState` from a list of easy grammar expressions.
    pub fn from_easy_exprs(exprs: Vec<(String, EasyGrammarExpr)>, llm_tokens: &[LLMToken]) -> Self {
        let (grammar, tokenizer, _) = Grammar::from_easy_exprs(exprs);
        GrammarConstraintState::new_from_grammar(tokenizer, grammar, llm_tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finite_automata::eat_u8;

    #[test]
    fn test_easy_grammar_from_exprs() {
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

        let llm_tokens = &[b"i".as_slice(), b"+", b"*", b"(", b")", b"(i", b"+i"];
        let mut grammar_state = GrammarConstraintState::from_easy_exprs(exprs, llm_tokens);

        // Simulate generating from a LLM with the grammar constraint.
        // We may have some 'prefill' we want to pass to the parser before we generate the first new LLM token.
        // Let's say the prefill is "(i+i*i".
        // This would be best tokenized as ["(i", "+", "i", "*", "i"].
        //
        // Take note of the ambiguity in the LLM tokens; we could the prefill as ["(", "i", "+", "i", "*", "i"],
        // i.e. break the "(i" token into "(" and "i". But that's a waste of a token.
        // A good LLM tokenizer would greedily emit the longest possible token at each step.
        grammar_state.commit_many(&[b"(i".as_slice(), b"+i", b"*", b"i"]);

        // Get the mask.
        // The valid tokens right now are be ["+", "*", ")", "+i)"].
        let mask = grammar_state.get_mask();
        assert_eq!(mask, BTreeSet::from([b"+".as_slice(), b"*", b")", b"+i)"]));


    }
}