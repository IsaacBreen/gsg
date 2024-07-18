use unicode_general_category::GeneralCategory;

pub fn get_unicode_general_category(general_category: GeneralCategory) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    for c in '\u{0}'..='\u{10FFFF}' {
        result.push(c.to_string().into_bytes());
    }

    result
}