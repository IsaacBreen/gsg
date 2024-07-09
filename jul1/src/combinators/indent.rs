use crate::{BruteForceFn, DataEnum, HorizontalData, U8Set, VerticalData};

const INDENT_FN: BruteForceFn = |values: &Vec<u8>, horizontal_data: &HorizontalData| {
    let mut i = 0;
    for indent_chunk in &horizontal_data.indents {
        let values_chunk = &values[i..i + indent_chunk.len()];
        if values_chunk != indent_chunk {
            if indent_chunk.starts_with(values_chunk) {
                // This could be a valid indentation, but we need more
                let next_u8 = values_chunk.get(indent_chunk.len()).cloned().unwrap();
                return DataEnum::Vertical(VerticalData { u8set: U8Set::from_u8(next_u8) });
            } else {
                // We have invalid indentation
                return DataEnum::None;
            }
        }
        i += indent_chunk.len();
    }
    DataEnum::Horizontal(horizontal_data.clone())
};
