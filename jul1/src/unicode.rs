use unicode_general_category::{GeneralCategory, GENERAL_CATEGORY};

use crate::*;

pub fn get_unicode_general_category_bytestrings(general_category: GeneralCategory) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    // for c in '\u{0}'..='\u{10FFFF}' {
    //     if get_general_category(c) == general_category {
    //         let utf8_bytes = c.to_string().as_bytes().to_vec();
    //         result.push(utf8_bytes);
    //     }
    // }

    for (start, end, category) in GENERAL_CATEGORY

    result
}

pub fn get_unicode_general_category_combinator(general_category: GeneralCategory) -> Combinator {
    let bytestrings = get_unicode_general_category_bytestrings(general_category);
    eat_bytestring_choice(bytestrings)
}