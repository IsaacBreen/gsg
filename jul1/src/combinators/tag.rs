use std::any::Any;
use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Tagged<A> {
    pub inner: A,
    pub tag: String,
}

pub struct TaggedParser<A> {
    pub inner: A,
    pub tag: String,
}

impl<A> CombinatorTrait for Tagged<A>
where
    A: CombinatorTrait,
{
    type Parser = TaggedParser<A::Parser>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        let (parser, parse_results) = self.inner.parser(right_data);
        (TaggedParser { inner: parser, tag: self.tag.clone() }, parse_results)
    }
}

impl<A> ParserTrait for TaggedParser<A>
where
    A: ParserTrait + 'static,
{
    fn step(&mut self, c: u8) -> ParseResults {
        let ParseResults { right_data_vec: right_data, up_data_vec: up_data, cut } = self.inner.step(c);
        ParseResults {
            right_data_vec: right_data,
            up_data_vec: up_data,
            cut,
        }
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        Box::new(std::iter::once(&self.inner as &dyn ParserTrait))
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        Box::new(std::iter::once(&mut self.inner as &mut dyn ParserTrait))
    }

    fn collect_stats(&self, stats: &mut Stats) {
        self.inner.collect_stats(stats);
        stats.active_parser_type_counts.entry("TaggedParser".to_string()).and_modify(|c| *c += 1).or_insert(1);
        *stats.active_tags.entry(self.tag.clone()).and_modify(|c| *c += 1).or_insert(1);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn tag<A>(tag: &str, a: A) -> Tagged<A::Output>
where
    A: IntoCombinator,
{
    Tagged { inner: a.into_combinator(), tag: tag.to_string() }
}

impl<A> IntoCombinator for &Tagged<A>
where
    A: CombinatorTrait + Clone,
{
    type Output = Tagged<A>;
    fn into_combinator(self) -> Self::Output {
        self.clone()
    }
}