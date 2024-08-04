use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExcludeBytestrings {
    pub(crate) inner: Box<Combinator>,
    pub(crate) bytestrings_to_exclude: Vec<Vec<u8>>,
    pub(crate) persist_with_partial_lookahead: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExcludeBytestringsParser {
    pub(crate) inner: Box<Parser>,
    pub(crate) bytestrings_to_exclude: Vec<Vec<u8>>,
    pub(crate) position: usize,
    pub(crate) start_position: usize,
    pub(crate) persist_with_partial_lookahead: bool,
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
        let start_position = right_data.position;
        let (inner, mut parse_results) = self.inner.parse(right_data, bytes);
        let mut bytestrings_to_exclude = self.bytestrings_to_exclude.clone();
        bytestrings_to_exclude.retain(|bytestring| common_prefix(bytes, bytestring));
        parse_results.right_data_vec.retain(|right_data| {
            for bytestring_to_exclude in &bytestrings_to_exclude {
                // Since we know at this point that they share a prefix, we can just check the length
                if start_position + bytestring_to_exclude.len() == right_data.position {
                   return false;
                }
            }
            true
        });
        if self.persist_with_partial_lookahead {
            for right_data in parse_results.right_data_vec.iter_mut() {
                let remaining = bytestrings_to_exclude.iter().map(|bytestring| bytestring[right_data.position - start_position..].to_vec()).collect();
                let remaining_combinator = eat_bytestring_choice(remaining);
                let (remaining_parser, _) = remaining_combinator.parse(right_data.clone(), &[]);
                right_data.lookahead_data.partial_lookaheads.push(PartialLookahead {
                    parser: Box::new(remaining_parser),
                    positive: false,
                });
            }
        } else {
            for right_data in parse_results.right_data_vec.iter_mut() {
                right_data.lookahead_data.has_omitted_partial_lookaheads = true;
            }
        }
        (Parser::ExcludeBytestringsParser(ExcludeBytestringsParser {
            inner: Box::new(inner),
            bytestrings_to_exclude,
            position: start_position + bytes.len(),
            start_position,
            persist_with_partial_lookahead: self.persist_with_partial_lookahead,
        }), parse_results)
    }
}

impl ParserTrait for ExcludeBytestringsParser {
    fn get_u8set(&self) -> U8Set {
        self.inner.get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_results = self.inner.parse(bytes);
        self.bytestrings_to_exclude.retain(|bytestring| common_prefix(bytes, &bytestring[self.position - self.start_position..]));
        parse_results.right_data_vec.retain(|right_data| {
            for bytestring_to_exclude in &self.bytestrings_to_exclude {
                // Since we know at this point that they share a prefix, we can just check the length
                if self.start_position + bytestring_to_exclude.len() == right_data.position {
                    return false;
                }
            }
            true
        });
        if self.persist_with_partial_lookahead {
            for right_data in parse_results.right_data_vec.iter_mut() {
                let remaining = self.bytestrings_to_exclude.iter().map(|bytestring| bytestring[right_data.position - self.start_position..].to_vec()).collect();
                let remaining_combinator = eat_bytestring_choice(remaining);
                let (remaining_parser, _) = remaining_combinator.parse(right_data.clone(), &[]);
                right_data.lookahead_data.partial_lookaheads.push(PartialLookahead {
                    parser: Box::new(remaining_parser),
                    positive: false,
                });
            }
        } else {
            for right_data in parse_results.right_data_vec.iter_mut() {
                right_data.lookahead_data.has_omitted_partial_lookaheads = true;
            }
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
        persist_with_partial_lookahead: false,
    })
}

impl From<ExcludeBytestrings> for Combinator {
    fn from(exclude_bytestrings: ExcludeBytestrings) -> Self {
        Self::ExcludeBytestrings(exclude_bytestrings)
    }
}
