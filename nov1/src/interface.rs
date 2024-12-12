// src/interface.rs
use crate::finite_automata::{greedy_group, groups, non_greedy_group, ExprGroup, ExprGroups};
use crate::finite_automata::{Expr, Regex};
use crate::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use crate::glr::parser::{GLRParser, ParseState};
use crate::glr::table::{assign_non_terminal_ids, generate_glr_parser, generate_glr_parser_with_maps, NonTerminalID, StateID, TerminalID};
use crate::precompute::{precompute, LLMTokenID, Token, Tokenizer};
use bimap::BiBTreeMap;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::{Debug, Formatter};
use kdam::tqdm;
use crate::analyze_grammar::drop_dead;
use crate::constraint::{precompute_add_eof, GrammarConstraint};

type LLMToken<'a> = &'a [u8];
type LLMTokenMap = BiBTreeMap<Vec<u8>, LLMTokenID>;

#[derive(Clone)]
pub struct Grammar<T> {
    pub productions: Vec<Production>,
    pub start_production_id: usize,
    pub literal_map: BTreeMap<String, String>,
    pub terminal_name_to_group_id: BiBTreeMap<String, usize>,
    pub terminal_expr_to_group_id: BiBTreeMap<Expr, usize>,
    pub tokenizer: T,
}

impl<T> Debug for Grammar<T> where T: Debug {
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

        writeln!(f, "Tokenizer:");
        writeln!(f, "{:?}", &self.tokenizer);

        Ok(())
    }
}

impl<T> Grammar<T> {
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

impl<T> Grammar<T> {
    pub fn glr_parser(&self) -> GLRParser {
        generate_glr_parser(&self.productions, self.start_production_id)
    }
}

impl Grammar<Regex> {
    /// Constructs a `Grammar` and `Regex` tokenizer from a list of grammar expressions.
    /// The first non-terminal in the list is treated as the start symbol.
    pub fn from_exprs(exprs: Vec<(String, GrammarExpr)>) -> Self {
        let mut productions = Vec::new();
        let mut literal_map = BTreeMap::new();
        let mut terminal_name_to_group_id = BiBTreeMap::new();
        let mut terminal_expr_to_group_id = BiBTreeMap::new();
        let mut next_terminal_id = 0;

        // Add a start production.
        // make sure the start production name is not already taken by adding apostrophes to it until it's unique.
        let mut start_production_name = "start'".to_string();
        let nonterminals: HashSet<&str> = exprs.iter().map(|(name, _)| name.as_str()).collect();
        while nonterminals.contains(&start_production_name.as_str()) {
            start_production_name.push('\'');
        }
        crate::dbgprintln2!("start_production_name: {:?}", start_production_name);
        productions.push(Production {
            lhs: NonTerminal(start_production_name.clone()),
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
            // todo: make this `terminal_group_id_to_expr` instead
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

        for (name, expr) in tqdm!(exprs.iter()) {
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

        // TODO: this is bad. prob remove this.
        // crate::dbgprintln2!("Dropping dead productions");
        // let productions = drop_dead(&productions);

        let tokenizer_exprs_vec: Vec<ExprGroup> = tokens
            .into_iter()
            .map(|(_, expr)| greedy_group(expr))
            .collect();
        let tokenizer_expr_groups = groups(tokenizer_exprs_vec);
        crate::dbgprintln2!("Building tokenizer");
        let tokenizer = tokenizer_expr_groups.clone().build();

        crate::dbgprintln2!("Done defining grammar");
        Self {
            productions,
            start_production_id: 0,
            literal_map,
            terminal_name_to_group_id,
            terminal_expr_to_group_id,
            tokenizer,
        }
    }
}

impl<T: Tokenizer> GrammarConstraint<T> {
    pub fn from_grammar(grammar: Grammar<T>, llm_tokens: LLMTokenMap, eof_llm_token_id: usize, max_token_id: usize) -> Self {
        crate::dbgprintln2!("GrammarConstraint::from_grammar");
        let terminal_map = grammar.terminal_name_to_group_id.iter().map(|(name, group_id)| { (Terminal(name.clone()), TerminalID(*group_id)) }).collect();
        let non_terminal_map = assign_non_terminal_ids(&grammar.productions);
        crate::dbgprintln2!("Generating GLR parser");
        let parser = generate_glr_parser_with_maps(&grammar.productions, grammar.start_production_id, terminal_map, non_terminal_map);

        crate::dbgprintln2!("Precomputing");
        let mut precomputed = precompute(&grammar.tokenizer, &llm_tokens, LLMTokenID(eof_llm_token_id), max_token_id);
        crate::dbgprintln2!("precomputed.len(): {}", precomputed.len());
        precompute_add_eof(&mut precomputed, LLMTokenID(eof_llm_token_id), parser.eof_terminal_id.0, max_token_id);
        // precompute_add_eof(&mut precomputed, LLMTokenID(eof_llm_token_id), llm_tokens.len(), max_token_id);
        crate::dbgprintln2!("precomputed.len(): {}", precomputed.len());
        crate::dbgprintln2!("Done precomputing");

        // // todo: remove this
        // crate::dbgprintln2!("GrammarConstraint::from_grammar");
        // let terminal_map = grammar.terminal_name_to_group_id.iter().map(|(name, group_id)| { (Terminal(name.clone()), TerminalID(*group_id)) }).collect();
        // let non_terminal_map = assign_non_terminal_ids(&grammar.productions);
        // crate::dbgprintln2!("Generating GLR parser");
        // let parser = generate_glr_parser_with_maps(&grammar.productions, grammar.start_production_id, terminal_map, non_terminal_map);

        Self {
            tokenizer: grammar.tokenizer,
            parser,
            precomputed,
            max_token_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use bitvec::prelude::*;
    use super::*;
    use crate::finite_automata::eat_u8;
    use crate::glr::table::generate_glr_parser;
    use crate::precompute::{print_precomputed, LLMTokenID};
    use crate::{choice_fast, groups, seq_fast};
    use crate::tokenizer_combinators::{eat_u8_fast, eat_u8_negation_fast, eat_u8_range_fast, repeat0_fast};
    use crate::trie::TrieNode;


    fn bitvec_with_capacity_and_values(capacity: usize, values: Vec<usize>) -> BitVec {
        let mut bitvec = BitVec::new();
        bitvec.resize(capacity, false);
        for value in values {
            bitvec.set(value, true);
        }
        bitvec
    }

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

        let llm_tokens: Vec<Vec<u8>> = vec![b"i".to_vec(), b"+".to_vec(), b"*".to_vec(), b"(".to_vec(), b")".to_vec(), b"(i".to_vec(), b"+i".to_vec()];
        let llm_token_map: LLMTokenMap = llm_tokens.iter().enumerate().map(|(i, token)| (token.clone(), LLMTokenID(i))).collect();
        let eof_llm_token_id = llm_tokens.len() + 1;
        let max_token_id = llm_tokens.len() + 1;
        let grammar_constraint = GrammarConstraint::from_grammar(grammar, llm_token_map.clone(), eof_llm_token_id, max_token_id);
        let mut grammar_constraint_state = grammar_constraint.init();

        macro_rules! llm_token_vec {
            ($($token:expr),* $(,)?) => {
                vec![
                    $(
                        llm_token_map.get_by_left(&$token.to_vec()).unwrap().0,
                    )*
                ]
            }
        }

        for (tokenizer_state, root) in &grammar_constraint_state.parent.precomputed {
            crate::dbgprintln!("Tokenizer state: {}", tokenizer_state.0);
            for node in TrieNode::all_nodes(Arc::new(Mutex::new(root.clone()))) {
                crate::dbgprintln!("Node address: {:p}, value: {:?}", Arc::as_ptr(&node), node.try_lock().unwrap().value);
                // print edge values and destination addresses
                for (edge, dest) in node.try_lock().unwrap().children() {
                    crate::dbgprintln!("    Edge value: {:?}, destination address: {:p}", edge, Arc::as_ptr(&dest));
                }
            }
        }

        // Get the mask.
        // The valid LLM tokens initially are ["i", "(", "(i"].
        let mask = grammar_constraint_state.get_mask();
        let expected_mask = bitvec_with_capacity_and_values(llm_tokens.len() + 1, llm_token_vec!(b"i", b"(", b"(i"));
        assert_eq!(mask, expected_mask);

        // Simulate generating from a LLM with the grammar constraint.
        // We may have some 'prefill' we want to pass to the parser before we generate the first new LLM token.
        // Let's say the prefill is "(i+i*i".
        // This would be best tokenized as ["(i", "+", "i", "*", "i"].
        //
        // Take note of the ambiguity in the LLM tokens; we could the prefill as ["(", "i", "+", "i", "*", "i"],
        // i.e. break the "(i" token into "(" and "i". But that's a waste of a token.
        // A good LLM tokenizer would greedily emit the longest possible token at each step.
        let prefill: Vec<_> = llm_token_vec!(b"(i", b"+i", b"*", b"i").into_iter().map(|token_id| LLMTokenID(token_id)).collect();
        grammar_constraint_state.commit_many(&prefill);

        // Get the mask.
        // The valid LLM tokens right now are ["+", "*", ")", "+i)"].
        let mask = grammar_constraint_state.get_mask();
        let expected_mask = bitvec_with_capacity_and_values(llm_tokens.len() + 1, llm_token_vec!(b"+", b"*", b")", b"+i"));
        assert_eq!(mask, expected_mask);

        // Finish it
        let terminals: Vec<_> = llm_token_vec!(b")").into_iter().map(|token_id| LLMTokenID(token_id)).collect();
        grammar_constraint_state.commit_many(&terminals);
        let mask = grammar_constraint_state.get_mask();
        let mut expected_mask = bitvec_with_capacity_and_values(llm_tokens.len() + 1, llm_token_vec!(b"+", b"*", b"+i"));
        // Add the EOF token
        expected_mask.set(llm_tokens.len(), true);
        assert_eq!(mask, expected_mask);

    }

    #[test]
    fn test_grammar_from_exprs_simple() {
        let exprs = vec![
            (
                "E".to_string(),
                sequence(vec![
                    regex(eat_u8(b'a')),
                    regex(eat_u8(b'b')),
                ]),
            ),
        ];

        let grammar = Grammar::from_exprs(exprs.clone());
        dbg!(&grammar);

        let parser = grammar.glr_parser();
        dbg!(&parser);

        let llm_tokens: Vec<Vec<u8>> = vec![b"a".to_vec(), b"b".to_vec()];
        let llm_token_map: LLMTokenMap = llm_tokens.iter().enumerate().map(|(i, token)| (token.clone(), LLMTokenID(i))).collect();
        let eof_llm_token_id = llm_tokens.len() + 1;
        let max_token_id = llm_tokens.len() + 1;
        let grammar_constraint = GrammarConstraint::from_grammar(grammar, llm_token_map.clone(), eof_llm_token_id, max_token_id);
        let mut grammar_constraint_state = grammar_constraint.init();

        for (tokenizer_state, root) in &grammar_constraint_state.parent.precomputed {
            crate::dbgprintln!("Tokenizer state: {}", tokenizer_state.0);
            for node in TrieNode::all_nodes(Arc::new(Mutex::new(root.clone()))) {
                crate::dbgprintln!("Node address: {:p}, value: {:?}", Arc::as_ptr(&node), node.try_lock().unwrap().value);
                // print edge values and destination addresses
                for (edge, dest) in node.try_lock().unwrap().children() {
                    crate::dbgprintln!("    Edge value: {:?}, destination address: {:p}", edge, Arc::as_ptr(&dest));
                }
            }
        }

        macro_rules! llm_token_vec {
            ($($token:expr),* $(,)?) => {
                vec![
                    $(
                        llm_token_map.get_by_left(&$token.to_vec()).unwrap().0,
                    )*
                ]
            }
        }

        // Get the mask.
        let mask = grammar_constraint_state.get_mask();
        let expected_mask = bitvec_with_capacity_and_values(llm_tokens.len() + 1, llm_token_vec!(b"a"));
        assert_eq!(mask, expected_mask);

        // Commit "a"
        let terminals: Vec<_> = llm_token_vec!(b"a").into_iter().map(|token_id| LLMTokenID(token_id)).collect();
        grammar_constraint_state.commit_many(&terminals);

        // Get the mask.
        let mask = grammar_constraint_state.get_mask();
        let expected_mask = bitvec_with_capacity_and_values(llm_tokens.len() + 1, llm_token_vec!(b"b"));
        assert_eq!(mask, expected_mask);
    }

    #[test]
    fn test_grammar_from_exprs_very_simple() {
        let exprs = vec![
            (
                "E".to_string(),
                regex(eat_u8(b'a')),
            ),
        ];

        let grammar = Grammar::from_exprs(exprs.clone());
        dbg!(&grammar);

        let parser = grammar.glr_parser();
        dbg!(&parser);

        let llm_tokens: Vec<Vec<u8>> = vec![b"a".to_vec()];
        let llm_token_map: LLMTokenMap = llm_tokens.iter().enumerate().map(|(i, token)| (token.clone(), LLMTokenID(i))).collect();
        let eof_llm_token_id = llm_tokens.len() + 1;
        let max_token_id = llm_tokens.len() + 1;
        let grammar_constraint = GrammarConstraint::from_grammar(grammar, llm_token_map.clone(), eof_llm_token_id, max_token_id);
        let mut grammar_constraint_state = grammar_constraint.init();

        print_precomputed(&grammar_constraint_state.parent.precomputed);

        for (tokenizer_state, root) in &grammar_constraint_state.parent.precomputed {
            crate::dbgprintln!("Tokenizer state: {}", tokenizer_state.0);
            for node in TrieNode::all_nodes(Arc::new(Mutex::new(root.clone()))) {
                crate::dbgprintln!("Node address: {:p}, value: {:?}", Arc::as_ptr(&node), node.try_lock().unwrap().value);
                // print edge values and destination addresses
                for (edge, dest) in node.try_lock().unwrap().children() {
                    crate::dbgprintln!("    Edge value: {:?}, destination address: {:p}", edge, Arc::as_ptr(&dest));
                }
            }
        }

        macro_rules! llm_token_vec {
            ($($token:expr),* $(,)?) => {
                vec![
                    $(
                        llm_token_map.get_by_left(&$token.to_vec()).unwrap().0,
                    )*
                ]
            }
        }

        // Get the mask.
        let mask = grammar_constraint_state.get_mask();
        let expected_mask = bitvec_with_capacity_and_values(llm_tokens.len() + 1, llm_token_vec!(b"a"));
        assert_eq!(mask, expected_mask);

        // Commit "a"
        let terminals: Vec<_> = llm_token_vec!(b"a").into_iter().map(|token_id| LLMTokenID(token_id)).collect();
        grammar_constraint_state.commit_many(&terminals);

        // Get the mask.
        let mask = grammar_constraint_state.get_mask();
        let mut expected_mask = bitvec_with_capacity_and_values(llm_tokens.len() + 1, llm_token_vec!());
        // Add the EOF token
        expected_mask.set(llm_tokens.len(), true);
        assert_eq!(mask, expected_mask);
    }

    #[test]
    fn test_precompute_for_python_name_token() {
        // ignore = rep(choice([
        //     eat_u8(ord(" ")),
        //     seq([eat_u8(ord("#")), rep(eat_u8_negation(ord("\n"))), eat_u8(ord("\n"))]),
        // ]))
        // digit = choice([eat_u8(c) for c in range(ord("0"), ord("9") + 1)])
        // alph_lower = choice([eat_u8(c) for c in range(ord("a"), ord("z") + 1)])
        // alph_upper = choice([eat_u8(c) for c in range(ord("A"), ord("Z") + 1)])
        //
        // name_start = choice([
        //     alph_lower,
        //     alph_upper,
        //     eat_u8(ord("_"))
        // ])
        // name_middle = choice([
        //     name_start,
        //     digit,
        // ])
        let ignore = repeat0_fast(choice_fast!(eat_u8_fast(b' '), seq_fast!(eat_u8_fast(b'#'), repeat0_fast(eat_u8_negation_fast(b'\n')), eat_u8_fast(b'\n'))));

        let digit = eat_u8_range_fast(b'0', b'9');
        let alph_lower = eat_u8_range_fast(b'a', b'z');
        let alph_upper = eat_u8_range_fast(b'A', b'Z');

        let name_start = choice_fast!(alph_lower, alph_upper, eat_u8_fast(b'_'));
        let name_middle = choice_fast!(name_start.clone(), digit);
        let name = seq_fast!(ignore, name_start, repeat0_fast(seq_fast!(name_middle)));

        let tokenizer = name.build();
        dbg!(&tokenizer);

        // // Define LLM tokens
        let llm_tokens: Vec<Vec<u8>> = (0..2).map(|i| format!("abcdefghijk{}", i).as_bytes().to_vec()).collect();
        let llm_tokens_slices: Vec<&[u8]> = llm_tokens.iter().map(|token| &token[..]).collect();
        let llm_token_map: LLMTokenMap = llm_tokens.iter().enumerate().map(|(i, token)| (token.clone(), LLMTokenID(i))).collect();
        let eof_llm_token_id = llm_tokens.len() + 1;
        let max_token_id = llm_tokens.len() + 1;
        let precomputed = precompute(&tokenizer, &llm_token_map, LLMTokenID(eof_llm_token_id), max_token_id);
        print_precomputed(&precomputed);
        println!("Done precomputing");
        // print_precomputed(&precomputed);
    }

    #[test]
    fn test_precompute_explosion() {
        let tokenizer = groups![
            seq_fast![eat_u8(b'a'), eat_u8(b'a'), eat_u8(b'a')],
            seq_fast![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'a')],
            seq_fast![eat_u8(b'b'), eat_u8(b'a'), eat_u8(b'a')],
            seq_fast![eat_u8(b'b'), eat_u8(b'a'), eat_u8(b'b')],
            seq_fast![eat_u8(b'a'), eat_u8(b'a')],
            seq_fast![eat_u8(b'b'), eat_u8(b'a')],
            seq_fast![eat_u8(b'b'), eat_u8(b'b')],
            repeat0_fast(seq_fast![eat_u8(b'a'), eat_u8(b'a')]),
            repeat0_fast(eat_u8(b'a')),
            // eat_u8(b'a'),
            eat_u8(b'a'),
            eat_u8(b'b'),
        ].build();

        // Define the LLM token as 32 'a's
        let llm_tokens: Vec<Vec<u8>> = vec![b"ab".to_vec(), b"aaaaaaaa".to_vec(), b"babababa".to_vec(), b"aabbaabb".to_vec()];
        let llm_token_map: LLMTokenMap = llm_tokens.iter().enumerate().map(|(i, token)| (token.clone(), LLMTokenID(i))).collect();
        let eof_llm_token_id = llm_tokens.len() + 1;
        let max_token_id = llm_tokens.len() + 1;
        let precomputed = precompute(&tokenizer, &llm_token_map, LLMTokenID(eof_llm_token_id), max_token_id);
        print_precomputed(&precomputed);
        println!("Done precomputing");
    }
}