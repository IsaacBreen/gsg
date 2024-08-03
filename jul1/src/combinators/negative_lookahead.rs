use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExcludeBytestrings {
    pub(crate) inner: Box<Combinator>,
    pub(crate) bytestrings: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExcludeBytestringsParser {
    pub(crate) inner: Box<Parser>,
    pub(crate) bytestrings: Vec<Vec<u8>>,
    pub(crate) position: usize,
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
    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (inner, mut parse_results) = self.inner.parser_with_steps(right_data.clone(), bytes);
        let mut exclusion_filter = U8Set::none();
        let mut bytestrings = self.bytestrings.clone();
        bytestrings.retain(|bytestring| bytes.len() < bytestring.len());
        bytestrings.retain(|bytestring| common_prefix(bytes, bytestring));
        let mut position = bytes.len();
        // Exclude character if it's the last character of a bytestring.
        for bytestring in &bytestrings {
            if bytestring.len() == position + 1 {
                let c = bytestring[position];
                exclusion_filter |= U8Set::from_byte(c);
            }
        }
        dbg!(exclusion_filter); exclusion_filter = exclusion_filter.complement();
        if bytestrings.iter().any(|bytestring| bytes.starts_with(bytestring)) {
            println!("Clearing right data");
            parse_results.right_data_vec.clear();
        }
        (Parser::ExcludeBytestringsParser(ExcludeBytestringsParser {
            inner: Box::new(inner),
            bytestrings,
            position,
        }), parse_results)
    }
}

impl ParserTrait for ExcludeBytestringsParser {
    fn get_u8set(&self) -> U8Set {
        todo!()
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_results = self.inner.steps(bytes);
        let mut exclusion_filter = U8Set::none();
        self.bytestrings.retain(|bytestring| self.position + bytes.len() < bytestring.len());
        self.position += bytes.len();
        // Exclude character if it's the last character of a bytestring.
        for bytestring in &self.bytestrings {
            if bytestring.len() == self.position + 1 {
                let c = bytestring[self.position];
                exclusion_filter |= U8Set::from_byte(c);
            }
        }
        dbg!(exclusion_filter); exclusion_filter = exclusion_filter.complement();
        if self.bytestrings.iter().any(|bytestring| common_prefix(bytes, &bytestring[self.position - bytes.len()..])) {
            parse_results.right_data_vec.clear();
        }
        self.bytestrings.retain(|bytestring| common_prefix(bytes, &bytestring[self.position - bytes.len()..]));
        parse_results
    }
}

pub fn exclude_strings(inner: Combinator, bytestrings: Vec<&str>) -> Combinator {
    let bytestrings = bytestrings.iter().map(|s| s.as_bytes().to_vec()).collect();
    Combinator::ExcludeBytestrings(ExcludeBytestrings {
        inner: Box::new(inner),
        bytestrings,
    })
}

impl From<ExcludeBytestrings> for Combinator {
    fn from(exclude_bytestrings: ExcludeBytestrings) -> Self {
        Self::ExcludeBytestrings(exclude_bytestrings)
    }
}