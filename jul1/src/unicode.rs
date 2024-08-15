use crate::*;
use crate::unicode_categories::{char_ranges_in_general_category, GeneralCategory};

pub fn get_unicode_general_category_bytestrings(general_category: GeneralCategory) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    for (start, end) in char_ranges_in_general_category(general_category).iter().cloned() {
        for c in start..=end {
            let byte_len = c.len_utf8();
            let mut bytes = vec![0; byte_len];
            c.encode_utf8(&mut bytes);
            result.push(bytes.to_vec());
        }
    }

    result
}

pub fn get_unicode_general_category_combinator(general_category: GeneralCategory)-> impl CombinatorTrait {
    let bytestrings = get_unicode_general_category_bytestrings(general_category);
    eat_bytestring_choice(bytestrings)
}