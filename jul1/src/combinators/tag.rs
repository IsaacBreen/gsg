use crate::*;

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

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (parser, right_data, up_data) = self.inner.parser(right_data);
        (TaggedParser { inner: parser, tag: self.tag.clone() }, right_data, up_data)
    }
}

impl<A> ParserTrait for TaggedParser<A>
where
    A: ParserTrait,
{
    fn step(&mut self, c: u8) -> ParseResults {
        let ParseResults(right_data, up_data) = self.inner.step(c);
        ParseResults(right_data, up_data)
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        Box::new(std::iter::once(&self.inner as &dyn ParserTrait))
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        Box::new(std::iter::once(&mut self.inner as &mut dyn ParserTrait))
    }

    fn collect_stats(&self, stats: &mut Stats) {
        self.inner.collect_stats(stats);
        stats.active_parser_type_counts.insert("TaggedParser".to_string(), 1);
        stats.active_tags.insert(self.tag.clone(), 1);
    }
}