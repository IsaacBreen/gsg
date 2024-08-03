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

impl CombinatorTrait for ExcludeBytestrings {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        let (inner, mut parse_results) = self.inner.parser(right_data);
        let mut exclusion_filter = U8Set::none();
        let mut position = 0;
        // Exclude character if it's the last character of a bytestring.
        for bytestring in &self.bytestrings {
            if bytestring.len() == position + 1 {
                let c = bytestring[position];
                exclusion_filter |= U8Set::from_byte(c);
            }
        }
        exclusion_filter = exclusion_filter.complement(); dbg!(exclusion_filter);
        for up_data in parse_results.up_data_vec.iter_mut() {
            up_data.u8set &= exclusion_filter;
        }
        (Parser::ExcludeBytestringsParser(ExcludeBytestringsParser {
            inner: Box::new(inner),
            bytestrings: self.bytestrings.clone(),
            position,
        }), parse_results)
    }

    fn parser_with_steps(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let (inner, mut parse_results) = self.inner.parser_with_steps(right_data.clone(), bytes);
        let mut exclusion_filter = U8Set::none();
        let mut position = bytes.len();
        // Exclude character if it's the last character of a bytestring.
        for bytestring in &self.bytestrings {
            if bytestring.len() == position + 1 {
                let c = bytestring[position];
                exclusion_filter |= U8Set::from_byte(c);
            }
        }
        exclusion_filter = exclusion_filter.complement(); dbg!(exclusion_filter);
        for up_data in parse_results.up_data_vec.iter_mut() {
            up_data.u8set &= exclusion_filter;
        }
        (Parser::ExcludeBytestringsParser(ExcludeBytestringsParser {
            inner: Box::new(inner),
            bytestrings: self.bytestrings.clone(),
            position,
        }), parse_results)
    }
}

impl ParserTrait for ExcludeBytestringsParser {
    fn step(&mut self, c: u8) -> ParseResults {
        let mut parse_results = self.inner.step(c);
        let mut exclusion_filter = U8Set::none();
        self.position += 1;
        // Exclude character if it's the last character of a bytestring.
        for bytestring in &self.bytestrings {
            if bytestring.len() == self.position + 1 {
                let c = bytestring[self.position];
                exclusion_filter |= U8Set::from_byte(c);
            }
        }
        exclusion_filter = exclusion_filter.complement(); dbg!(exclusion_filter);
        for up_data in parse_results.up_data_vec.iter_mut() {
            up_data.u8set &= exclusion_filter;
        }
        parse_results
    }

    fn steps(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_results = self.inner.steps(bytes);
        let mut exclusion_filter = U8Set::none();
        self.position += bytes.len();
        // Exclude character if it's the last character of a bytestring.
        for bytestring in &self.bytestrings {
            if bytestring.len() == self.position + 1 {
                let c = bytestring[self.position];
                exclusion_filter |= U8Set::from_byte(c);
            }
        }
        exclusion_filter = exclusion_filter.complement(); dbg!(exclusion_filter);
        for up_data in parse_results.up_data_vec.iter_mut() {
            up_data.u8set &= exclusion_filter;
        }
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