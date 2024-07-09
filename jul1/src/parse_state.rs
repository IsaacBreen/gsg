use crate::{FrameStack, U8Set};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct HorizontalData {
    pub frame_stack: FrameStack,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VerticalData {
    pub u8set: U8Set,
}

impl HorizontalData {
    pub fn squash(horizontal_data: Vec<HorizontalData>) -> Vec<HorizontalData> {
        let mut new_horizontal_data = vec![];
        for hd in horizontal_data {
            if new_horizontal_data.is_empty() || hd != new_horizontal_data.last().cloned().unwrap() {
                new_horizontal_data.push(hd);
            }
        }
        new_horizontal_data
    }
}

impl VerticalData {
    pub fn squash(vertical_data: Vec<VerticalData>) -> Vec<VerticalData> {
        let mut u8set = U8Set::none();
        for vd in vertical_data {
            u8set = u8set.union(&vd.u8set);
        }
        if u8set.is_empty() {
            vec![]
        } else {
            vec![VerticalData { u8set }]
        }
    }
}
