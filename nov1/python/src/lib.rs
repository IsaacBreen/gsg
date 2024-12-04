// python/src/lib.rs
use sep1::finite_automata::{Expr as RegexExpr, ExprGroups as RegexGroups, greedy_group, non_greedy_group, groups as regex_groups, _choice as regex_choice, eat_u8, eat_u8_negation, eat_u8_set, eps, opt, prec, rep, rep1, _seq as regex_seq};
use sep1::finite_automata::Regex;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes, PyDict};
use sep1::glr::grammar::{NonTerminal, Production, Symbol, Terminal};
use sep1::glr::parser::GLRParser;
use sep1::glr::table::{generate_glr_parser, StateID};
use sep1::interface::{Grammar, GrammarExpr, choice as grammar_choice, optional as grammar_optional, regex as grammar_regex, repeat as grammar_repeat, r#ref as grammar_ref, sequence as grammar_sequence};
use sep1::constraint::{GrammarConstraint, GrammarConstraintState};
use sep1::precompute::{print_precomputed, LLMTokenID, Tokenizer};
use std::collections::{BTreeMap, BTreeSet};
use bimap::BiBTreeMap;
use numpy::{IntoPyArray, PyArray1, ToPyArray};
use sep1::u8set::U8Set;

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

    #[staticmethod]
    fn regex(regex: PyRegexExpr) -> Self {
        Self {
            inner: grammar_regex(regex.inner)
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct PyRegexExpr {
    inner: RegexExpr,
}

#[pymethods]
impl PyRegexExpr {
    #[staticmethod]
    fn eat_u8(c: u8) -> Self {
        Self { inner: eat_u8(c) }
    }

    #[staticmethod]
    fn eat_u8_negation(c: u8) -> Self {
        Self { inner: eat_u8_negation(c) }
    }

    #[staticmethod]
    fn rep(expr: PyRegexExpr) -> Self {
        Self { inner: rep(expr.inner) }
    }

    #[staticmethod]
    fn rep1(expr: PyRegexExpr) -> Self {
        Self { inner: rep1(expr.inner) }
    }

    #[staticmethod]
    fn opt(expr: PyRegexExpr) -> Self {
        Self { inner: opt(expr.inner) }
    }

    #[staticmethod]
    fn prec(precedence: isize, expr: PyRegexExpr) -> PyRegexGroup {
        PyRegexGroup { inner: prec(precedence, expr.inner) }
    }

    #[staticmethod]
    fn eps() -> Self {
        Self { inner: eps() }
    }

    #[staticmethod]
    fn seq(exprs: Vec<PyRegexExpr>) -> Self {
        Self { inner: regex_seq(exprs.into_iter().map(|e| e.inner).collect()) }
    }

    #[staticmethod]
    fn choice(exprs: Vec<PyRegexExpr>) -> Self {
        Self { inner: regex_choice(exprs.into_iter().map(|e| e.inner).collect()) }
    }

    fn build(&self) -> PyResult<PyRegex> {
        Ok(PyRegex { inner: self.inner.clone().build() })
    }
}

#[pyclass]
#[derive(Clone)]
struct PyRegexGroup {
    inner: sep1::finite_automata::ExprGroup,
}

#[pymethods]
impl PyRegexGroup {
    #[staticmethod]
    fn greedy_group(expr: PyRegexExpr) -> Self {
        Self { inner: greedy_group(expr.inner) }
    }

    #[staticmethod]
    fn non_greedy_group(expr: PyRegexExpr) -> Self {
        Self { inner: non_greedy_group(expr.inner) }
    }
}

#[pyclass]
#[derive(Clone)]
struct PyRegexGroups {
    inner: RegexGroups,
}

#[pymethods]
impl PyRegexGroups {
    #[staticmethod]
    fn groups(groups: Vec<PyRegexGroup>) -> Self {
        Self {
            inner: regex_groups(groups.into_iter().map(|g| g.inner).collect()),
        }
    }

    fn build(&self) -> PyResult<PyRegex> { // &self, not self
        Ok(PyRegex { inner: self.inner.clone().build() }) // clone the inner RegexExpr
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyRegex {
    inner: Regex,
}

#[pymethods]
impl PyRegex {
    // Add methods here as needed to expose Regex functionality to Python
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

    fn print(&self) {
        println!("{:?}", self.inner)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct PyGLRParser {
    inner: GLRParser,
}

#[pyclass]
#[derive(Clone)]
pub struct PyGrammarConstraint {
    inner: GrammarConstraint<Regex>,
}

#[pymethods]
impl PyGrammarConstraint {
    #[new]
    fn new(py: Python, grammar: PyGrammar, token_to_id: &PyDict, eof_llm_token_id: usize, max_token_id: usize) -> PyResult<Self> {
        // Convert the Python dictionary into a BiBTreeMap
        let mut llm_token_map: BiBTreeMap<Vec<u8>, LLMTokenID> = BiBTreeMap::new();
        for (key, value) in token_to_id.iter() {
            let token = key.extract::<&[u8]>()?;
            let id = value.extract::<usize>()?;
            llm_token_map.insert(token.to_vec(), LLMTokenID(id));
        }

        let inner = GrammarConstraint::from_grammar(grammar.inner, llm_token_map, eof_llm_token_id, max_token_id);
        Ok(Self { inner })
    }

    fn print(&self) {
        print_precomputed(&self.inner.precomputed);
    }
}


#[pyclass]
pub struct PyGrammarConstraintState {
    inner: GrammarConstraintState<Regex>,
}

#[pymethods]
impl PyGrammarConstraintState {
    #[new]
    fn new(grammar_constraint: PyGrammarConstraint) -> Self {
        Self { inner: grammar_constraint.inner.init() }
    }

    fn get_mask<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray1<bool>>> { // Correct return type
        let bitset = self.inner.get_mask();
        let bools: Vec<bool> = bitset.iter().map(|bit_ref| *bit_ref).collect();
        let array = bools.into_pyarray_bound(py); // Correct usage
        Ok(array)
    }

    fn commit(&mut self, llm_token_id: usize) {
        self.inner.commit(LLMTokenID(llm_token_id));
    }
}



/// A Python module implemented in Rust.
#[pymodule]
fn _sep1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGrammarExpr>()?;
    m.add_class::<PyRegexExpr>()?;
    m.add_class::<PyRegexGroup>()?;
    m.add_class::<PyRegexGroups>()?;
    m.add_class::<PyGrammar>()?;
    m.add_class::<PyGrammarConstraint>()?;
    m.add_class::<PyGrammarConstraintState>()?;
    Ok(())
}