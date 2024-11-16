// python/src/lib.rs
use sep1::finite_automata::Regex;
use sep1::finite_automata::Expr;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use sep1::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use sep1::glr::parser::GLRParser;
use sep1::glr::table::{generate_glr_parser, StateID};
use sep1::interface::{Grammar, GrammarExpr, choice as grammar_choice, optional as grammar_optional, regex as grammar_regex, repeat as grammar_repeat, r#ref as grammar_ref, sequence as grammar_sequence};
use sep1::constraint::{GrammarConstraint, GrammarConstraintState, LLMTokenID};
use sep1::precompute::Tokenizer;
use std::collections::{BTreeMap, BTreeSet};
use bimap::BiBTreeMap;

#[pyclass]
#[derive(Clone)]
struct PyGrammarExpr {
    inner: GrammarExpr,
}

#[pymethods]
impl PyGrammarExpr {
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

// todo: quick fix
type LLMToken = &'static [u8];

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
fn _sep1(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyGrammarExpr>()?;
    m.add_class::<PyGrammar>()?;
    m.add_class::<PyGrammarConstraint>()?;
    m.add_class::<PyGrammarConstraintState>()?;
    Ok(())
}