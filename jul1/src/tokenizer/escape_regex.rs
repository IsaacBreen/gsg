use crate::tokenizer::parse_regex::special_chars;
pub fn escape_regex(input: &str) -> String {
    // List of special regex characters that we want to escape
    let mut escaped = String::new();

    // Iterate through each character in the input string
    for c in input.chars() {
        // If the character is special, prepend it with a backslash
        if special_chars.contains(c) {
            escaped.push('\\');
        }
        // Add the original character to the result
        escaped.push(c);
    }

    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_regex() {
        assert_eq!(escape_regex(r"a.c"), r"a\.c");
        assert_eq!(escape_regex(r"^abc$"), r"\^abc\$");
        assert_eq!(escape_regex(r"3.5"), r"3\.5");
        assert_eq!(escape_regex(r"(1+2)*3"), r"\(1\+2\)\*3");
        assert_eq!(escape_regex(r"Use \d+"), r"Use \\d\+");
        assert_eq!(escape_regex(r"/path/to/some/file"), r"/path/to/some/file");
        assert_eq!(escape_regex(r"{2}"), r"\{2\}");
        assert_eq!(escape_regex(r"| or ||"), r"\| or \|\|");
    }
}

fn main() {
    // Example usage
    let original = r"(abc)*|{2}\d+.[xyz]";
    let escaped = escape_regex(original);
    println!("Original: {}", original);
    println!("Escaped: {}", escaped);
}