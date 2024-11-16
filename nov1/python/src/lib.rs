// python/src/lib.rs
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use tre::finite_automata::{eat_u8, choice, empty, not_empty, optional, plus, range, seq, star, Expr, Regex};
use tre::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use tre::glr::parser::GLRParser;
use tre::glr::table::{generate_glr_parser, StateID};
use tre::interface::{Grammar, GrammarExpr, choice as grammar_choice, optional as grammar_optional, regex as grammar_regex, repeat as grammar_repeat, r#ref as grammar_ref, sequence as grammar_sequence};
use tre::constraint::{GrammarConstraint, LLMTokenID};
use tre::precompute::Tokenizer;
use std::collections::{BTreeMap, BTreeSet};
use bimap::BiBTreeMap;

#[pyclass]
#[derive(Clone)]
struct PyGrammarExpr {
    inner: GrammarExpr,
}

#[pymethods]
impl PyGrammarExpr {
    #[new]
    fn new(expr: &str) -> PyResult<Self> {
        // Attempt to parse the expression. If it fails, return an error.
        let inner = parse_grammar_expr(expr)?;
        Ok(Self { inner })
    }

    #[staticmethod]
    fn regex(regex_str: &str) -> PyResult<Self> {
        let regex_expr = parse_regex_expr(regex_str)?;
        Ok(Self {
            inner: grammar_regex(regex_expr),
        })
    }

    #[staticmethod]
    fn r#ref(name: &str) -> PyResult<Self> {
        Ok(Self {
            inner: grammar_ref(name),
        })
    }

    #[staticmethod]
    fn sequence(exprs: Vec<PyGrammarExpr>) -> Self {
        Self {
            inner: grammar_sequence(exprs.into_iter().map(|e| e.inner).collect()),
        }
    }

    #[staticmethod]
    fn choice(exprs: Vec<PyGrammarExpr>) -> Self {
        Self {
            inner: grammar_choice(exprs.into_iter().map(|e| e.inner).collect()),
        }
    }

    #[staticmethod]
    fn optional(expr: PyGrammarExpr) -> Self {
        Self {
            inner: grammar_optional(expr.inner),
        }
    }

    #[staticmethod]
    fn repeat(expr: PyGrammarExpr) -> Self {
        Self {
            inner: grammar_repeat(expr.inner),
        }
    }
}



fn parse_grammar_expr(expr: &str) -> PyResult<GrammarExpr> {
    // Very simple parser for demonstration. Replace with a real parser.
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() == 1 {
        if parts[0].starts_with("'") && parts[0].ends_with("'") {
            let literal = &parts[0][1..parts[0].len() - 1];
            let expr = grammar_regex(Expr::Literal(literal.as_bytes().to_vec()));
            Ok(expr)
        } else {
            Ok(grammar_ref(parts[0]))
        }
    } else if parts[1] == "|" {
        Ok(grammar_choice(vec![
            parse_grammar_expr(parts[0])?,
            parse_grammar_expr(parts[2])?,
        ]))
    } else if parts[1] == "+" {
        Ok(grammar_sequence(vec![
            parse_grammar_expr(parts[0])?,
            parse_grammar_expr(parts[2])?,
        ]))
    } else {
        Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Invalid grammar expression: {}",
            expr
        )))
    }
}

fn parse_regex_expr(regex_str: &str) -> PyResult<Expr> {
    // Very simple parser for demonstration. Replace with a real parser.
    if regex_str.starts_with("'") && regex_str.ends_with("'") {
        let literal = ®ex_str[1..regex_str.len() - 1];
        Ok(Expr::Literal(literal.as_bytes().to_vec()))
    } else if regex_str == "." {
        Ok(not_empty())
    } else {
        Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Invalid regex expression: {}",
            regex_str
        )))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyGrammar {
    inner: Grammar<Regex>,
}

#[pymethods]
impl PyGrammar {
    #[new]
    fn new(exprs: Vec<(String, PyGrammarExpr)>) -> Self {
        let inner = Grammar::from_exprs(exprs.into_iter().map(|(s, e)| (s, e.inner)).collect());
        Self { inner }
    }

    fn glr_parser(&self) -> PyGLRParser {
        PyGLRParser { inner: self.inner.glr_parser() }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyGLRParser {
    inner: GLRParser,
}

#[pyclass]
pub struct PyGrammarConstraint {
    inner: GrammarConstraint<Regex>,
}

#[pymethods]
impl PyGrammarConstraint {
    #[new]
    fn new(grammar: PyGrammar, llm_tokens: Vec<&PyBytes>) -> Self {
        let llm_tokens_vec: Vec<LLMToken> = llm_tokens.into_iter().map(|token| token.as_bytes()).collect();
        Self { inner: GrammarConstraint::from_grammar(grammar.inner, &llm_tokens_vec) }
    }

    fn init(&self) -> PyGrammarConstraintState {
        PyGrammarConstraintState { inner: self.inner.init() }
    }
}

#[pyclass]
pub struct PyGrammarConstraintState {
    inner: GrammarConstraintState<'static, Regex>,
}

#[pymethods]
impl PyGrammarConstraintState {
    fn get_mask(&self) -> Vec<usize> {
        self.inner.get_mask().into_iter().map(|id| id.0).collect()
    }

    fn commit(&mut self, llm_token_id: usize) {
        self.inner.commit(LLMTokenID(llm_token_id));
    }
}



/// A Python module implemented in Rust.
#[pymodule]
fn _tre(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGrammarExpr>()?;
    m.add_class::<PyGrammar>()?;
    m.add_class::<PyGrammarConstraint>()?;
    m.add_class::<PyGrammarConstraintState>()?;
    Ok(())
}