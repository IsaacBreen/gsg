use bitvec::prelude::*;

// Trait for the parser state
pub trait ParserState: Clone {
    fn new() -> Self;
    fn parse(&mut self, read_char: &impl ReadChar);
    fn valid_next_chars(&self) -> BitVec;
    fn is_valid(&self) -> bool {
        !self.valid_next_chars().is_empty()
    }
}

// Trait for reading characters
pub trait ReadChar: Fn(usize) -> Option<char> {}
impl<F: Fn(usize) -> Option<char>> ReadChar for F {}

// Simple implementation of ParserState for JSON-like parsing
#[derive(Clone, Debug, PartialEq)]
pub struct JsonLikeState {
    depth: usize,
    in_string: bool,
    escaped: bool,
}

impl ParserState for JsonLikeState {
    fn new() -> Self {
        JsonLikeState {
            depth: 0,
            in_string: false,
            escaped: false,
        }
    }

    fn parse(&mut self, read_char: &impl ReadChar) {
        if let Some(c) = read_char(0) {
            if self.in_string {
                if self.escaped {
                    self.escaped = false;
                } else if c == '\\' {
                    self.escaped = true;
                } else if c == '"' {
                    self.in_string = false;
                }
            } else {
                match c {
                    '{' | '[' => self.depth += 1,
                    '}' | ']' => {
                        if self.depth > 0 {
                            self.depth -= 1;
                        }
                    }
                    '"' => self.in_string = true,
                    _ => {}
                }
            }
        }
    }

    fn valid_next_chars(&self) -> BitVec {
        let mut valid = bitvec![0; 128];
        if self.in_string {
            if self.escaped {
                valid.set('"' as usize, true);
                valid.set('\\' as usize, true);
                valid.set('/' as usize, true);
                valid.set('b' as usize, true);
                valid.set('f' as usize, true);
                valid.set('n' as usize, true);
                valid.set('r' as usize, true);
                valid.set('t' as usize, true);
                valid.set('u' as usize, true);
            } else {
                for i in 32..127 {
                    valid.set(i, true);
                }
                valid.set('\\' as usize, true);
            }
        } else if self.depth == 0 {
            // At the root level, only { and [ are valid
            valid.set('{' as usize, true);
            valid.set('[' as usize, true);
        } else {
            // Inside an object or array
            valid.set('}' as usize, true);
            valid.set(']' as usize, true);
            valid.set('"' as usize, true);
            valid.set(':' as usize, true);
            valid.set(',' as usize, true);
            for c in b'0'..=b'9' {
                valid.set(c as usize, true);
            }
            valid.set('-' as usize, true);
            valid.set('t' as usize, true); // true
            valid.set('f' as usize, true); // false
            valid.set('n' as usize, true); // null
            // Allow nested arrays, but not nested objects directly
            valid.set('[' as usize, true);
        }
        valid
    }
}

// Helper function to create a ReadChar from a string
pub fn string_reader(s: &str) -> impl ReadChar + '_ {
    move |i| s.chars().nth(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_like_state() {
        let mut state = JsonLikeState::new();
        let input = r#"{"key": "value", "array": [1, 2, 3]}"#;
        let reader = string_reader(input);

        for i in 0..input.len() {
            state.parse(&|j| reader(i + j));
            assert!(state.is_valid());
        }

        assert_eq!(state.depth, 0);
        assert!(!state.in_string);
        assert!(!state.escaped);
    }

    #[test]
    fn test_valid_next_chars() {
        let mut state = JsonLikeState::new();

        // Test root level
        let valid_chars = state.valid_next_chars();
        assert!(valid_chars['{' as usize]);
        assert!(valid_chars['[' as usize]);
        assert!(!valid_chars['"' as usize]);

        // Test inside an object
        state.parse(&|_| Some('{'));
        let valid_chars = state.valid_next_chars();
        assert!(valid_chars['"' as usize]);
        assert!(!valid_chars['{' as usize]);
        assert!(!valid_chars['[' as usize]);

        // Test after object key
        state.parse(&|_| Some('"'));
        state.parse(&|_| Some('k'));
        state.parse(&|_| Some('e'));
        state.parse(&|_| Some('y'));
        state.parse(&|_| Some('"'));
        let valid_chars = state.valid_next_chars();
        assert!(valid_chars[':' as usize]);
        assert!(!valid_chars['{' as usize]);
        assert!(!valid_chars['[' as usize]);
        assert!(!valid_chars['"' as usize]);

        // Test after colon
        state.parse(&|_| Some(':'));
        let valid_chars = state.valid_next_chars();
        assert!(valid_chars['"' as usize]);
        assert!(valid_chars['[' as usize]);
        assert!(valid_chars['t' as usize]); // true
        assert!(valid_chars['f' as usize]); // false
        assert!(valid_chars['n' as usize]); // null
        assert!(valid_chars['0' as usize]); // number
        assert!(!valid_chars['{' as usize]); // nested object not allowed directly
    }

    #[test]
    fn test_escaped_characters() {
        let mut state = JsonLikeState::new();
        let input = r#"{"key": "value with \"quotes\""}"#;
        let reader = string_reader(input);

        for i in 0..input.len() {
            state.parse(&|j| reader(i + j));
            assert!(state.is_valid());
        }

        assert_eq!(state.depth, 0);
        assert!(!state.in_string);
        assert!(!state.escaped);
    }

    #[test]
    fn test_invalid_json() {
        let mut state = JsonLikeState::new();
        let input = r#"{"key": "unclosed string"#;
        let reader = string_reader(input);

        for i in 0..input.len() {
            state.parse(&|j| reader(i + j));
        }

        assert!(state.in_string);
        assert_ne!(state.depth, 0);
    }

    #[test]
    fn test_fast_forward() {
        let mut state = JsonLikeState::new();
        let input = r#"{"key": "value", "array": [1, 2, 3]}"#;
        let reader = string_reader(input);
        let mut i = 0;

        while i < input.len() {
            let valid_chars = state.valid_next_chars();
            let mut fast_forward = true;
            let mut next_char = None;

            for j in 0..valid_chars.len() {
                if valid_chars[j] {
                    if next_char.is_some() {
                        fast_forward = false;
                        break;
                    }
                    next_char = Some(j as u8 as char);
                }
            }

            if fast_forward {
                if let Some(c) = next_char {
                    assert_eq!(reader(i), Some(c));
                    state.parse(&|j| reader(i + j));
                    i += 1;
                } else {
                    break;
                }
            } else {
                state.parse(&|j| reader(i + j));
                i += 1;
            }
        }

        assert_eq!(i, input.len());
        assert_eq!(state.depth, 0);
        assert!(!state.in_string);
        assert!(!state.escaped);
    }
}