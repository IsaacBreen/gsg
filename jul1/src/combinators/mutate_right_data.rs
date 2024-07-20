use crate::*;

#[derive(PartialEq, Eq)]
pub struct MutateRightData {
    pub run: fn(&mut RightData) -> bool,
}

impl CombinatorTrait for MutateRightData {
    type Parser = MutateRightData;

    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        if (self.run)(&mut right_data) {
            (MutateRightData { run: self.run }, ParseResults {
                right_data_vec: vec![right_data],
                up_data_vec: vec![],
                cut: false,
            })
        } else {
            (MutateRightData { run: self.run }, ParseResults {
                right_data_vec: vec![],
                up_data_vec: vec![],
                cut: false,
            })
        }
    }
}

impl ParserTrait for MutateRightData {
    fn step(&mut self, c: u8) -> ParseResults {
        ParseResults {
            right_data_vec: vec![],
            up_data_vec: vec![],
            cut: false,
        }
    }
}

pub fn mutate_right_data(run: fn(&mut RightData) -> bool) -> MutateRightData {
    MutateRightData { run }
}
