use std::collections::BTreeMap;
use crate::glr::table::StateID;
use crate::gss::GSSNode;

#[derive(Debug, Clone)]
pub(crate) enum PrecomputeGSSNode<GrammarToken, Leaf> {
    Internal(BTreeMap<GrammarToken, PrecomputeGSSNode<GrammarToken, Leaf>>),
    Leaf(Leaf),
}

impl<GrammarToken, Leaf> PrecomputeGSSNode<GrammarToken, Leaf> {
    pub(crate) fn flatten(&self) -> BTreeMap<Vec<GrammarToken>, Leaf> {
        todo!()
    }
}