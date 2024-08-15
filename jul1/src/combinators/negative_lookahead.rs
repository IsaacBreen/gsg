use std::collections::HashSet;
use crate::*;
use crate::trie::TrieNode;
use crate::VecX;

#[derive(Debug, Clone)]
pub struct ExcludeBytestrings {
    pub(crate) inner: Box<Combinator>,
    // pub(crate) bytestrings_to_exclude: VecX<Vec<u8>>,
    pub(crate) root: Rc<TrieNode>,
}

#[derive(Debug, Clone)]
pub struct ExcludeBytestringsParser {
    pub(crate) inner: Box<Parser>,
    pub(crate) node: Option<Rc<TrieNode>>,
    pub(crate) start_position: usize,
}

impl CombinatorTrait for ExcludeBytestrings {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (inner, mut parse_results) = self.inner.parse(right_data.clone(), bytes);
        let (indices, node) = self.root.get_indices(bytes);
        let indices: HashSet<usize> = indices.into_iter().collect();
        // Retain only results that don't coincide with the indices
        let start_position = right_data.right_data_inner.fields1.position;
        parse_results.right_data_vec.retain(|right_data| {
            !indices.contains(&(right_data.right_data_inner.fields1.position - start_position))
        });
        (Parser::ExcludeBytestringsParser(ExcludeBytestringsParser {
            inner: Box::new(inner),
            node: node.map(|node| Rc::new(node.clone())),
            start_position,
        }), parse_results)
    }
}

impl ParserTrait for ExcludeBytestringsParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_results = self.inner.parse(bytes);
        if let Some(node) = &self.node {
            let (indices, node) = node.get_indices(bytes);
            let indices: HashSet<usize> = indices.into_iter().collect();
            parse_results.right_data_vec.retain(|right_data| {
                !indices.contains(&(right_data.right_data_inner.fields1.position - self.start_position))
            });
            self.node = node.map(|node| Rc::new(node.clone()));
        }
        self.start_position += bytes.len();
        parse_results
    }
}

pub fn exclude_strings(inner: impl Into<Combinator>, bytestrings_to_exclude: Vec<&str>) -> Combinator {
    let bytestrings_to_exclude: Vec<Vec<u8>> = bytestrings_to_exclude.iter().map(|s| s.as_bytes().to_vec()).collect();
    Combinator::ExcludeBytestrings(ExcludeBytestrings {
        inner: Box::new(inner.into()),
        root: Rc::new(bytestrings_to_exclude.into()),
    })
}

impl From<ExcludeBytestrings> for Combinator {
    fn from(exclude_bytestrings: ExcludeBytestrings) -> Self {
        Self::ExcludeBytestrings(exclude_bytestrings)
    }
}
