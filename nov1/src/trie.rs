use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct TrieNode<T, E> {
    value: Option<T>,
    children: BTreeMap<E, Arc<Mutex<TrieNode<T, E>>>>,
}

impl<T, E> TrieNode<T, E> {
    fn new() -> Self {
        TrieNode {
            value: None,
            children: BTreeMap::new(),
        }
    }

    fn merge(&mut self, other: TrieNode<T, E>) {
        // TODO: `execute_all_from_state` merges nodes by input position. This is essential for
        //  efficiency in certain cases. For example, say we have two grammar tokens "=" and "==".
        //  The LLM token "========" can be tokenized (by the grammar tokenizer) in a huge number
        //  of ways.
        //  The number of ways explodes with the number of "="s in the input.
        //  But, thanks to the fact that `execute_all_from_state` merges by input position,
        //  the trie generated by `execute_all_from_state` doesn't explode.
        //  Unfortunately, this merge implementation undoes this. It unmerges nodes unnecessarily.
        //  So, if our two grammar tokens are "=|-" (the regex that matches "=" or "-") and
        //  "==|--", and we have two LLM tokens "========" and "--------", then running
        //  `execute_all_from_state` on both of these LLM tokens separately will produce identical
        //  tries, both nice and compact thanks to its merging strategy.
        //  But using `merge` on these two trees unmerges the nodes

        todo!()
    }
}