use crate::*;
use crate::unicode_categories::{char_ranges_in_general_category, GeneralCategory};

pub fn get_unicode_general_category_bytestrings(general_category: GeneralCategory) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    for (start, end) in char_ranges_in_general_category(general_category) {
        let utf8_bytes = start.to_string().as_bytes().to_vec();
        result.push(utf8_bytes);
        let utf8_bytes = end.to_string().as_bytes().to_vec();
        result.push(utf8_bytes);
    }

    result
}

pub fn get_unicode_general_category_combinator(general_category: GeneralCategory) -> Combinator {
    let bytestrings = get_unicode_general_category_bytestrings(general_category);
    eat_bytestring_choice(bytestrings)
}