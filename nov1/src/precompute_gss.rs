use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub(crate) enum PrecomputeGSSNode<GrammarToken, Leaf> {
    Internal(BTreeMap<GrammarToken, PrecomputeGSSNode<GrammarToken, Leaf>>),
    Leaf(Leaf),
}

impl<GrammarToken, Leaf> PrecomputeGSSNode<GrammarToken, Leaf>
where
    GrammarToken: Clone + Ord,
    Leaf: Clone,
{
    pub(crate) fn flatten(&self) -> BTreeMap<Vec<GrammarToken>, Leaf> {
        let mut result = BTreeMap::new();
        self.flatten_recursive(&mut result, Vec::new());
        result
    }

    fn flatten_recursive(
        &self,
        result: &mut BTreeMap<Vec<GrammarToken>, Leaf>,
        path: Vec<GrammarToken>,
    ) {
        match self {
            PrecomputeGSSNode::Internal(children) => {
                for (token, child) in children {
                    let mut new_path = path.clone();
                    new_path.push(token.clone());
                    child.flatten_recursive(result, new_path);
                }
            }
            PrecomputeGSSNode::Leaf(leaf) => {
                result.insert(path, leaf.clone());
            }
        }
    }
    
    pub(crate) fn map<F, U>(&self, f: F) -> PrecomputeGSSNode<GrammarToken, U>
    where
        F: Copy + Fn(&Leaf) -> U,
    {
        match self {
            PrecomputeGSSNode::Internal(children) => {
                let mut new_children = BTreeMap::new();
                for (token, child) in children {
                    new_children.insert(token.clone(), child.map(f));
                }
                PrecomputeGSSNode::Internal(new_children)
            }
            PrecomputeGSSNode::Leaf(leaf) => {
                PrecomputeGSSNode::Leaf(f(leaf))
            }
        }
    }
}