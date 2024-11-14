use crate::finite_automata::{greedy_group, groups, non_greedy_group, ExprGroup, ExprGroups};
use crate::finite_automata::{Expr, Regex};
use crate::glr::grammar::{t, NonTerminal, Production, Symbol, Terminal};
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
        let mut terminal_expr_to_group_id = BiBTreeMap::new();
        let mut next_terminal_id = 0;

        fn convert_easy_expr(
            expr: &EasyGrammarExpr,
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
            //  i.e. collect all nonterminals in teh grammar upfront and pass it to convert_easy_expr.
            match expr {
                EasyGrammarExpr::RegexExpr(regex_expr) => {
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
                            terminal_expr_to_group_id,
                            next_terminal_id,
                        )
                    })
                    .collect(),
                EasyGrammarExpr::Choice(exprs) => {
                    let new_nonterminal = format!("Choice{}", *next_non_terminal_id);
                    let nt = NonTerminal(new_nonterminal.clone());

                    // TODO: what if this is already in the map (e.g. the user happens to create a rule with name `Choice0`?
                    //  We need to generate a unique nonterminal name.
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
                EasyGrammarExpr::Optional(expr) => {
                    // TODO: name the internal rule here Option{} or something rather than Choice{}.
                    convert_easy_expr(
                        &EasyGrammarExpr::Choice(vec![*expr.clone(), EasyGrammarExpr::Sequence(vec![])]),
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
                EasyGrammarExpr::Repeat(expr) => {
                    // TODO: same as above, make sure it's unique.
                    let nonterminal_id = *next_non_terminal_id;
                    let nonterminal_name = format!("Repeat{}", nonterminal_id);
                    non_terminal_map.insert(NonTerminal(nonterminal_name.clone()), NonTerminalID(nonterminal_id));
                    *next_non_terminal_id += 1;
                    let rhs = convert_easy_expr(
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
            let rhs = convert_easy_expr(
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

        // The start symbol is the first non-terminal in the list
        let start_symbol = NonTerminal(exprs.first().unwrap().0.clone());

        let tokenizer_exprs_vec: Vec<ExprGroup> = tokens
            .into_iter()
            .map(|(_, expr)| greedy_group(expr))
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

        let (grammar, tokenizer, tokenizer_expr_groups) = Grammar::from_easy_exprs(exprs.clone());
        dbg!(&tokenizer_expr_groups);
        dbg!(&grammar);

        let llm_tokens = &[b"i".as_slice(), b"+", b"*", b"(", b")", b"(i", b"+i"];
        let mut grammar_state = GrammarConstraintState::from_easy_exprs(exprs, llm_tokens);

        // Get the mask.
        // The valid LLM tokens initially are ["i", "(", "(i"].
        let mask = grammar_state.get_mask();
        assert_eq!(mask, BTreeSet::from([b"i".as_slice(), b"(", b"(i"]));

        // Simulate generating from a LLM with the grammar constraint.
        // We may have some 'prefill' we want to pass to the parser before we generate the first new LLM token.
        // Let's say the prefill is "(i+i*i".
        // This would be best tokenized as ["(i", "+", "i", "*", "i"].
        //
        // Take note of the ambiguity in the LLM tokens; we could the prefill as ["(", "i", "+", "i", "*", "i"],
        // i.e. break the "(i" token into "(" and "i". But that's a waste of a token.
        // A good LLM tokenizer would greedily emit the longest possible token at each step.
        // grammar_state.commit_many(&[b"(i".as_slice(), b"+i", b"*", b"i"]);

        // Get the mask.
        // The valid LLM tokens right now are ["+", "*", ")", "+i)"].
        let mask = grammar_state.get_mask();
        assert_eq!(mask, BTreeSet::from([b"+".as_slice(), b"*", b")", b"+i)"]));


    }
}