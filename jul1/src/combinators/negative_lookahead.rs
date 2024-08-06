use crate::*;
use crate::VecX;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExcludeBytestrings {
    pub(crate) inner: Box<Combinator>,
    pub(crate) bytestrings_to_exclude: VecX<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExcludeBytestringsParser {
    pub(crate) inner: Box<Parser>,
    pub(crate) bytestrings_to_exclude: VecX<Vec<u8>>,
    pub(crate) position: usize,
    pub(crate) start_position: usize,
}

fn common_prefix(a: &[u8], b: &[u8]) -> bool {
    let mut i = 0;
    while i < a.len() && i < b.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

impl CombinatorTrait for ExcludeBytestrings {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let start_position = right_data.right_data_inner.position;
        let (inner, mut parse_results) = self.inner.parse(right_data, bytes);
        let mut bytestrings_to_exclude = self.bytestrings_to_exclude.clone();
        bytestrings_to_exclude.retain(|bytestring| common_prefix(bytes, bytestring));
        parse_results.right_data_vec.retain(|right_data| {
            for bytestring_to_exclude in &bytestrings_to_exclude {
                // Since we know at this point that they share a prefix, we can just check the length
                if start_position + bytestring_to_exclude.len() == right_data.right_data_inner.position {
                   return false;
                }
            }
            true
        });
            for right_data in parse_results.right_data_vec.iter_mut() {
                Rc::make_mut(&mut right_data.right_data_inner).lookahead_data.has_omitted_partial_lookaheads = true;
            }
        (Parser::ExcludeBytestringsParser(ExcludeBytestringsParser {
            inner: Box::new(inner),
            bytestrings_to_exclude,
            position: start_position + bytes.len(),
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
        self.bytestrings_to_exclude.retain(|bytestring| self.position - self.start_position < bytestring.len() && common_prefix(bytes, &bytestring[self.position - self.start_position..]));
        parse_results.right_data_vec.retain(|right_data| {
            for bytestring_to_exclude in &self.bytestrings_to_exclude {
                // Since we know at this point that they share a prefix, we can just check the length
                if self.start_position + bytestring_to_exclude.len() == right_data.right_data_inner.position {
                    return false;
                }
            }
            true
        });
            for right_data in parse_results.right_data_vec.iter_mut() {
                Rc::make_mut(&mut right_data.right_data_inner).lookahead_data.has_omitted_partial_lookaheads = true;
        }
        self.position += bytes.len();
        parse_results
    }
}

pub fn exclude_strings(inner: Combinator, bytestrings_to_exclude: Vec<&str>) -> Combinator {
    let bytestrings_to_exclude = bytestrings_to_exclude.iter().map(|s| s.as_bytes().to_vec()).collect();
    Combinator::ExcludeBytestrings(ExcludeBytestrings {
        inner: Box::new(inner),
        bytestrings_to_exclude,
    })
}

impl From<ExcludeBytestrings> for Combinator {
    fn from(exclude_bytestrings: ExcludeBytestrings) -> Self {
        Self::ExcludeBytestrings(exclude_bytestrings)
    }
}
