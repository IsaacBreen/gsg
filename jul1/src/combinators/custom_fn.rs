use crate::*;

pub struct CustomFn<Parser: ParserTrait> {
    pub run: fn(&mut RightData) -> (Parser, Vec<RightData>, Vec<UpData>),
}

impl<Parser: ParserTrait + 'static> CombinatorTrait for CustomFn<Parser> {
    type Parser = Parser;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        (self.run)(&mut right_data)
    }
}

impl<Parser: ParserTrait> ParserTrait for CustomFn<Parser> {
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        (vec![], vec![])
    }
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("CustomFn".to_string()).or_insert(0);
    }
}

pub fn custom_fn<Parser: ParserTrait>(run: fn(&mut RightData) -> (Parser, Vec<RightData>, Vec<UpData>)) -> CustomFn<Parser> {
    CustomFn { run }
}