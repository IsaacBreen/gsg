// src/combinators/negative_lookahead.rs
use std::collections::HashSet;
use crate::*;
use crate::trie::TrieNode;
use crate::{ApplyToChildren, VecX};

#[derive(Debug)]
pub struct ExcludeBytestrings<T: CombinatorTrait> {
    pub(crate) inner: Box<T>,
    // pub(crate) bytestrings_to_exclude: VecX<Vec<u8>>,
    pub(crate) root: Rc<TrieNode>,
}

#[derive(Debug)]
pub struct ExcludeBytestringsParser<'a> {
    pub(crate) inner: Box<Parser<'a>>,
    pub(crate) node: Option<&'a TrieNode>,
    pub(crate) start_position: usize,
}

impl<T: CombinatorTrait + 'static> CombinatorTrait for ExcludeBytestrings<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let start_position = right_data.right_data_inner.fields1.position;
        match self.inner.one_shot_parse(right_data, bytes) {
            Ok(right_data) => {
                let end_position = right_data.right_data_inner.fields1.position;
                let offset = end_position - start_position;
                if self.root.fully_matches(&bytes[..offset]) {
                    return Err(UnambiguousParseError::Fail);
                } else {
                     Ok(right_data)
                 }
            },
            Err(err) => Err(err),
        }
    }
    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
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
            node: node.map(|node| node),
            start_position,
        }), parse_results)
    }
}

impl<T: CombinatorTrait + 'static> ApplyToChildren for ExcludeBytestrings<T> {
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        f(&self.inner);
    }
}

impl ParserTrait for ExcludeBytestringsParser<'_> {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_results = self.inner.parse(bytes);
        if let Some(node) = self.node {
            let (indices, node) = node.get_indices(bytes);
            let indices: HashSet<usize> = indices.into_iter().collect();
            parse_results.right_data_vec.retain(|right_data| {
                !indices.contains(&(right_data.right_data_inner.fields1.position - self.start_position))
            });
            self.node = node;
        }
        self.start_position += bytes.len();
        parse_results
    }
}

pub fn exclude_strings(inner: impl IntoCombinator + 'static, bytestrings_to_exclude: Vec<&str>)-> impl CombinatorTrait {
    let bytestrings_to_exclude: Vec<Vec<u8>> = bytestrings_to_exclude.iter().map(|s| s.as_bytes().to_vec()).collect();
    ExcludeBytestrings {
        inner: Box::new(Box::new(inner.into_combinator())),
        root: Rc::new(bytestrings_to_exclude.into()),
    }
}

// impl From<ExcludeBytestrings> for Combinator {
//     fn from(exclude_bytestrings: ExcludeBytestrings) -> Self {
//         Self::ExcludeBytestrings(exclude_bytestrings)
//     }
// }