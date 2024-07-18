use crate::*;

pub struct MutateRightData {
    pub run: fn(&mut RightData) -> bool,
}

impl CombinatorTrait for MutateRightData {
    type Parser = MutateRightData;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        if (self.run)(&mut right_data) {
            (MutateRightData { run: self.run }, vec![right_data], vec![])
        } else {
            (MutateRightData { run: self.run }, vec![], vec![])
        }
    }
}

impl ParserTrait for MutateRightData {
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        (vec![], vec![])
    }
    fn collect_stats(&self, stats: &mut Stats) {
        stats.active_parser_type_counts.entry("MutateRightData".to_string()).or_insert(0);
    }
}

pub fn mutate_right_data(run: fn(&mut RightData) -> bool) -> MutateRightData {
    MutateRightData { run }
}
