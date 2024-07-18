use unicode_general_category::{GeneralCategory, get_general_category};
use crate::*;

pub fn get_unicode_general_category_bytestrings(general_category: GeneralCategory) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    for c in '\u{0}'..='\u{10FFFF}' {
        if get_general_category(c) == general_category {
            let utf8_bytes = c.to_string().into_bytes();
            result.push(utf8_bytes);
        }
    }

    result
}

pub fn get_unicode_general_category_combinator(general_category: GeneralCategory) -> Box<DynCombinator> {
    let bytestrings = get_unicode_general_category_bytestrings(general_category);

    let mut children = Vec::new();
    for bytestring in bytestrings {
        children.push(eat_bytes(&bytestring).into_boxed());
    }

    choice_from_vec(children).into_boxed()
}