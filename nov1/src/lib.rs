use crate::constraint::GrammarConstraintState;
use pyo3::prelude::*;
use crate::finite_automata::{greedy_group, groups, non_greedy_group, ExprGroup, ExprGroups, eat_u8};
use crate::finite_automata::{Expr, Regex};
use crate::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use crate::glr::parser::{GLRParser, ParseState};
use crate::glr::table::{assign_non_terminal_ids, generate_glr_parser, generate_glr_parser_with_maps, NonTerminalID, StateID, TerminalID};
use crate::precompute::{precompute, precompute_add_incomplete_token, Token, Tokenizer};
use bimap::BiBTreeMap;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};
use crate::constraint::{GrammarConstraint, LLMTokenID, convert_precomputed_to_llm_token_ids};

extern crate core;

pub mod frozenset;
pub mod charmap;
pub mod precompute;
pub mod tokenizer_combinators;
pub mod u8set;
pub mod finite_automata;
mod gss;
mod glr;
mod constraint;
mod interface;

type LLMToken = &'static [u8];

#[pymodule]
fn rust_grammar_constraint(_py: Python<'_>, m: &PyModule) -> PyResult<()> {

    #[pyfunction]
    fn eat_u8(byte: u8) -> Expr {
        eat_u8(byte)
    }

    #[pyfunction]
    fn groups(groups: Vec<ExprGroup>) -> ExprGroups {
        groups(groups)
    }

    #[pyfunction]
    fn greedy_group(expr: Expr) -> ExprGroup {
        greedy_group(expr)
    }

    #[pyfunction]
    fn non_greedy_group(expr: Expr) -> ExprGroup {
        non_greedy_group(expr)
    }

    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct PyExpr {
        pub inner: Expr
    }

    #[pymethods]
    impl PyExpr {
        #[new]
        fn new(inner: Expr) -> Self {
            PyExpr { inner }
        }
    }

    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct PyExprGroup {
        pub inner: ExprGroup
    }

    #[pymethods]
    impl PyExprGroup {
        #[new]
        fn new(inner: ExprGroup) -> Self {
            PyExprGroup { inner }
        }
    }

    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct PyExprGroups {
        pub inner: ExprGroups
    }

    #[pymethods]
    impl PyExprGroups {
        #[new]
        fn new(inner: ExprGroups) -> Self {
            PyExprGroups { inner }
        }

        #[pyo3(signature = ())]
        pub fn build(&self) -> Regex {
            self.inner.clone().build()
        }
    }

    #[pyclass]
    #[derive(Clone, Debug)]
    pub struct PyGrammarExpr {
        pub inner: interface::GrammarExpr
    }

    #[pymethods]
    impl PyGrammarExpr {
        #[new]
        fn new(inner: interface::GrammarExpr) -> Self {
            PyGrammarExpr { inner }
        }

        #[staticmethod]
        #[pyo3(signature = (expr))]
        pub fn regex(expr: PyExpr) -> Self {
            Self { inner: interface::regex(expr.inner) }
        }

        #[staticmethod]
        #[pyo3(signature = (name))]
        pub fn r#ref(name: String) -> Self {
            Self { inner: interface::r#ref(&name) }
        }

        #[staticmethod]
        #[pyo3(signature = (exprs))]
        pub fn sequence(exprs: Vec<PyGrammarExpr>) -> Self {
            Self { inner: interface::sequence(exprs.into_iter().map(|e| e.inner).collect()) }
        }

        #[staticmethod]
        #[pyo3(signature = (exprs))]
        pub fn choice(exprs: Vec<PyGrammarExpr>) -> Self {
            Self { inner: interface::choice(exprs.into_iter().map(|e| e.inner).collect()) }
        }

        #[staticmethod]
        #[pyo3(signature = (expr))]
        pub fn optional(expr: PyGrammarExpr) -> Self {
            Self { inner: interface::optional(expr.inner) }
        }

        #[staticmethod]
        #[pyo3(signature = (expr))]
        pub fn repeat(expr: PyGrammarExpr) -> Self {
            Self { inner: interface::repeat(expr.inner) }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    pub struct PyGrammar {
        inner: interface::Grammar<Regex>,
    }

    #[pymethods]
    impl PyGrammar {
        #[new]
        #[pyo3(signature = (exprs))]
        pub fn new(exprs: Vec<(String, PyGrammarExpr)>) -> Self {
            Self { inner: interface::Grammar::from_exprs(exprs.into_iter().map(|(s, e)| (s, e.inner)).collect()) }
        }

        pub fn glr_parser(&self) -> PyGLRParser {
            PyGLRParser { inner: self.inner.glr_parser() }
        }

        pub fn __repr__(&self) -> String {
            format!("{:?}", self.inner)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    pub struct PyGLRParser {
        inner: GLRParser,
    }

    #[pymethods]
    impl PyGLRParser {
        pub fn __repr__(&self) -> String {
            format!("{}", self.inner)
        }
    }

    #[pyclass]
    pub struct PyGrammarConstraint {
        inner: GrammarConstraint<Regex>,
    }

    #[pymethods]
    impl PyGrammarConstraint {
        #[new]
        #[pyo3(signature = (grammar, llm_tokens))]
        pub fn new(grammar: PyGrammar, llm_tokens: Vec<&[u8]>) -> Self {
            Self { inner: GrammarConstraint::from_grammar(grammar.inner, &llm_tokens) }
        }

        pub fn init(&self) -> PyGrammarConstraintState {
            PyGrammarConstraintState { inner: self.inner.init() }
        }
    }

    #[pyclass]
    pub struct PyGrammarConstraintState {
        inner: GrammarConstraintState<'static, Regex>,
    }

    #[pymethods]
    impl PyGrammarConstraintState {
        pub fn get_mask(&self) -> Vec<usize> {
            self.inner.get_mask().iter().map(|&LLMTokenID(id)| id).collect()
        }

        pub fn commit(&mut self, llm_token_id: usize) {
            self.inner.commit(LLMTokenID(llm_token_id));
        }

        pub fn commit_many(&mut self, llm_token_ids: Vec<usize>) {
            self.inner.commit_many(&llm_token_ids.iter().map(|&id| LLMTokenID(id)).collect::<Vec<_>>());
        }
    }

    m.add_class::<PyGrammarExpr>()?;
    m.add_class::<PyGrammar>()?;
    m.add_class::<PyGLRParser>()?;
    m.add_class::<PyGrammarConstraint>()?;
    m.add_class::<PyGrammarConstraintState>()?;
    m.add_class::<PyExpr>()?;
    m.add_class::<PyExprGroup>()?;
    m.add_class::<PyExprGroups>()?;
    m.add_function(wrap_pyfunction!(crate::eat_u8, m)?)?;
    m.add_function(wrap_pyfunction!(crate::groups, m)?)?;
    m.add_function(wrap_pyfunction!(crate::greedy_group, m)?)?;
    m.add_function(wrap_pyfunction!(crate::non_greedy_group, m)?)?;

    Ok(())
}