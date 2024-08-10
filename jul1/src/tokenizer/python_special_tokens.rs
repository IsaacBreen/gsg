use std::collections::HashMap;
use num_traits::FromPrimitive;
use crate::tokenizer::escape_regex::escape_regex;
use crate::tokenizer::finite_automata::{FindReturn, Regex, RegexState, Success};

use crate::tokenizer::python_literals::{get_python_literals, get_python_non_literal_token_patterns, TokenId};
use crate::tokenizer::tokenizer_trait::{Token, Tokenizer};

#[derive(Copy, Clone)]
enum BracketType {
    Paren,
    Brace,
    Bracket,
}

#[derive(Copy, Clone)]
enum FStringType {
    SingleSingleQuote,
    SingleDoubleQuote,
    TripleSingleQuote,
    TripleDoubleQuote,
}

#[derive(Copy, Clone)]
enum PythonTokenizerMode {
    Normal,
    /// Bracket mode is the same as normal mode, but we also ignore newlines. We cannot enter Newline
    /// mode while in Bracket mode.
    Bracket(BracketType),
    /// In Newline mode, we're deciding whether the next token(s) is a normal token, an indent, or (one or more) dedents.
    Newline,
    /// In FString mode, we're parsing the closing quotes, a single open brace, or the f-string middle.
    /// An open brace pushes Bracket mode, an f-string middle doesn't push anymore, and a closing quote
    /// pops the current FString mode.
    FString(FStringType),
}

pub struct PythonTokenizer {
    mode_stack: Vec<PythonTokenizerMode>,
    indent_stack: Vec<String>,
    regex_state: RegexState,
    group_id_to_token_id: HashMap<usize, TokenId>,
}

impl PythonTokenizer {
    fn make_regex(mode: PythonTokenizerMode) -> (Regex, HashMap<usize, TokenId>) {
        match mode {
            PythonTokenizerMode::Normal | PythonTokenizerMode::Bracket(_) => {
                let literals = get_python_literals();
                let patterns = get_python_non_literal_token_patterns();

                let mut all_patterns = Vec::new();
                let mut group_id_to_token_id = HashMap::new();

                for (token_id, literal) in literals {
                    group_id_to_token_id.insert(all_patterns.len(), TokenId::from_usize(token_id).unwrap());
                    all_patterns.push(escape_regex(&literal));
                }

                for (token_id, pattern) in patterns {
                    group_id_to_token_id.insert(all_patterns.len(), TokenId::from_usize(token_id).unwrap());
                    all_patterns.push(pattern);
                }

                (Regex::from_strs(all_patterns), group_id_to_token_id)
            }
            PythonTokenizerMode::Newline => {
                // let mut all_patterns = Vec::new();
                // let mut group_id_to_token_id = HashMap::new();
                // Match zero or more whitespace
                // patterns.insert(TokenId::INDENT as usize, r"[^\S\n]*");
                // all_patterns.push(r"[^\S\n]*");

                // Ignore zero or more whitespace and a newline
                // patterns.insert(TokenId::IGNORE as usize, r"[^\S\n]*\n");
                // Note: the regex engine is greedy, so it'll match the IGNORE token if possible,
                // and the NEWLINE otherwise
                todo!();
            },
            PythonTokenizerMode::FString(_) => todo!()
        }
    }
}

impl Default for PythonTokenizer {
    fn default() -> Self {
        let (regex, group_id_to_token_id) = Self::make_regex(PythonTokenizerMode::Normal);
        PythonTokenizer {
            mode_stack: vec![PythonTokenizerMode::Normal],
            indent_stack: vec![],
            regex_state: regex.init(),
            group_id_to_token_id,
        }
    }
}

impl Tokenizer for PythonTokenizer {
    fn tokenize(&mut self, input: &str) -> (Vec<Token>, usize, usize) {
        let mut tokens = Vec::new();
        let mut total_width = 0;
        let mut num_read = 0;
        let mut pos = 0;
        let mut regex_state_start_pos = self.regex_state.position;
        let mut skipped = 0;

        loop {
            self.regex_state.execute(&input[pos..]);
            if let Some( FindReturn { inner: Some(Success { position: width, group_id }), .. }) = &self.regex_state.find_return {
                let id = self.group_id_to_token_id[&group_id];
                let token = Token { id: id as u8, width: *width };

                match self.mode_stack.last().unwrap() {
                    PythonTokenizerMode::Normal => {
                        match id {
                            // Skip ignores
                            TokenId::IGNORE => skipped += *width,
                            // Enter bracket mode
                            TokenId::LBRACE => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Brace)),
                            TokenId::LPAR => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Paren)),
                            TokenId::LSQB => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Bracket)),
                            // Enter newline mode
                            TokenId::NEWLINE => self.mode_stack.push(PythonTokenizerMode::Newline),
                            // Enter FString mode
                            TokenId::FSTRING_START_SINGLE_SINGLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::SingleSingleQuote)),
                            TokenId::FSTRING_START_SINGLE_DOUBLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::SingleDoubleQuote)),
                            TokenId::FSTRING_START_TRIPLE_SINGLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::TripleSingleQuote)),
                            TokenId::FSTRING_START_TRIPLE_DOUBLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::TripleDoubleQuote)),
                            _ => (),
                        }
                    }
                    PythonTokenizerMode::Bracket(BracketType::Brace) => {
                        match id {
                            // Skip ignores
                            TokenId::IGNORE => skipped += *width,
                            // Enter bracket mode
                            TokenId::LBRACE => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Brace)),
                            TokenId::LPAR => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Paren)),
                            TokenId::LSQB => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Bracket)),
                            // If we encounter a closing brace, pop the bracket mode
                            TokenId::RBRACE => { self.mode_stack.pop(); },
                            // Skip newlines
                            TokenId::NEWLINE => skipped += *width,
                            // Enter FString mode
                            TokenId::FSTRING_START_SINGLE_SINGLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::SingleSingleQuote)),
                            TokenId::FSTRING_START_SINGLE_DOUBLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::SingleDoubleQuote)),
                            TokenId::FSTRING_START_TRIPLE_SINGLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::TripleSingleQuote)),
                            TokenId::FSTRING_START_TRIPLE_DOUBLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::TripleDoubleQuote)),
                            _ => (),
                        }
                    }
                    PythonTokenizerMode::Bracket(BracketType::Bracket) => {
                        match id {
                            // Skip ignores
                            TokenId::IGNORE => skipped += *width,
                            // Enter bracket mode
                            TokenId::LBRACE => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Brace)),
                            TokenId::LPAR => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Paren)),
                            TokenId::LSQB => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Bracket)),
                            // If we encounter a closing bracket, pop the bracket mode
                            TokenId::RSQB => { self.mode_stack.pop(); },
                            // Skip newlines
                            TokenId::NEWLINE => skipped += *width,
                            // Enter FString mode
                            TokenId::FSTRING_START_SINGLE_SINGLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::SingleSingleQuote)),
                            TokenId::FSTRING_START_SINGLE_DOUBLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::SingleDoubleQuote)),
                            TokenId::FSTRING_START_TRIPLE_SINGLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::TripleSingleQuote)),
                            TokenId::FSTRING_START_TRIPLE_DOUBLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::TripleDoubleQuote)),
                            _ => (),
                        }
                    }
                    PythonTokenizerMode::Bracket(BracketType::Paren) => {
                        match id {
                            // Skip ignores
                            TokenId::IGNORE => skipped += *width,
                            // Enter bracket mode
                            TokenId::LBRACE => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Brace)),
                            TokenId::LPAR => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Paren)),
                            TokenId::LSQB => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Bracket)),
                            // If we encounter a closing parenthesis, pop the bracket mode
                            TokenId::RPAR => { self.mode_stack.pop(); },
                            // Skip newlines
                            TokenId::NEWLINE => skipped += *width,
                            // Enter FString mode
                            TokenId::FSTRING_START_SINGLE_SINGLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::SingleSingleQuote)),
                            TokenId::FSTRING_START_SINGLE_DOUBLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::SingleDoubleQuote)),
                            TokenId::FSTRING_START_TRIPLE_SINGLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::TripleSingleQuote)),
                            TokenId::FSTRING_START_TRIPLE_DOUBLE_QUOTE => self.mode_stack.push(PythonTokenizerMode::FString(FStringType::TripleDoubleQuote)),
                            _ => (),
                        }
                    }
                    PythonTokenizerMode::FString(_) => {
                        match id {
                            // If we match the end, pop the FString mode
                            TokenId::FSTRING_END => { self.mode_stack.pop(); },
                            // If we match a right brace, enter bracket mode
                            TokenId::RBRACE => self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Brace)),
                            _ => (),
                        }
                    }
                    PythonTokenizerMode::Newline => {
                        // Check the next character.
                        // - If it's a newline, we have an ignore token
                        // - Otherwise, we have one of:
                        //   - a newline
                        //   - a newline followed by some dedents
                        //   - a newline followed by an indent
                        let next_char = input.chars().nth(pos + 1).unwrap();
                        if next_char == '\n' {
                            // Ignore
                            skipped += width;
                        } else {
                            // Match the indents
                            let mut actual_indent = &input[pos..pos + width];
                            for (i, expected_indent) in self.indent_stack.clone().iter().enumerate() {
                                if actual_indent.len() == 0 {
                                    // Dedent
                                    // Pop the remaining indents from the stack
                                    for _ in i..self.indent_stack.len() {
                                        tokens.push(Token { id: TokenId::DEDENT as u8, width: 0 });
                                        self.indent_stack.pop();
                                    }
                                } else if let Some(remaining_actual_indent) = actual_indent.strip_prefix(expected_indent) {
                                    // Ok
                                    actual_indent = remaining_actual_indent;
                                } else {
                                    // Indentation mismatch
                                    todo!();
                                }
                            }
                            if actual_indent.len() == 0 {
                                // Perfect indentation match. No indents or dedents.
                            } else {
                                // Indent
                                self.indent_stack.push(actual_indent.to_string());
                                tokens.push(Token { id: TokenId::INDENT as u8, width: 0 });
                            }
                        }
                    }
                }

                tokens.push(token);
                total_width += *width;
                pos += *width;
                num_read = num_read.max(pos + self.regex_state.position - regex_state_start_pos);

                // Reset regex state
                self.regex_state = Self::make_regex(*self.mode_stack.last().unwrap()).0.init();
                regex_state_start_pos = 0;
            } else {
                num_read = num_read.max(pos + self.regex_state.position - regex_state_start_pos);
                break;
            }
        }

        (tokens, total_width, num_read)
    }

    fn potential_token_ids(&self, input: &str) -> Vec<u8> {
        todo!()
    }
}

pub fn to_cpython_exact_type(token_id: u8) -> u8 {
    let python_literals = get_python_literals();
    if let Some(token_string) = python_literals.get(&(token_id as usize)) {
        // If it's an identifier, Python treats it as a token of exact type 'NAME'
        let identifier_regex = regex::Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
        if identifier_regex.is_match(token_string) {
            return TokenId::NAME as u8;
        }
    }

    token_id
}

pub fn tokenize_cpython_compatible(mut input: &str) -> Vec<u8> {
    let mut tokenizer = PythonTokenizer::default();
    let (tokens, _, _) = tokenizer.tokenize(input);
    let mut token_cpython_ids = Vec::new();
    for token in tokens {
        token_cpython_ids.push(to_cpython_exact_type(token.id));
    }
    token_cpython_ids
}

#[cfg(test)]
mod tests {
    use num_traits::FromPrimitive;

    use super::*;

    #[test]
    fn test_bracket_open_close() {
        let mut tokenizer = PythonTokenizer::default();
        let input = "[";
        let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        let mut tokens_iter = tokens.iter();let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::LSQB as u8);
        assert_eq!(token.width, 1);

        let input = "]";let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::RSQB as u8);
        assert_eq!(token.width, 1);
    }

    #[test]
    fn test_parenthesis_open_close() {
        let mut tokenizer = PythonTokenizer::default();
        let input = "(";
        let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        let mut tokens_iter = tokens.iter();let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::LPAR as u8);
        assert_eq!(token.width, 1);

        let input = ")";let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::RPAR as u8);
        assert_eq!(token.width, 1);
    }

    #[test]
    fn test_brace_open_close() {
        let mut tokenizer = PythonTokenizer::default();
        let input = "{";
        let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        let mut tokens_iter = tokens.iter();let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::LBRACE as u8);
        assert_eq!(token.width, 1);

        let input = "}";let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::RBRACE as u8);
        assert_eq!(token.width, 1);
    }

    #[test]
    fn test_newline_indent_dedent() {
        let mut tokenizer = PythonTokenizer::default();
        let input = "\n    ";
        let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        let mut tokens_iter = tokens.iter();let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::NEWLINE as u8);
        assert!(num_read > 0);

        // Further tests would be needed to check for correct handling of indent and dedent
    }

    #[test]
    fn test_fstring_handling() {
        let mut tokenizer = PythonTokenizer::default();
        let input = "f'hello {()}!'";
        let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        let mut tokens_iter = tokens.iter();
        let token = tokens_iter.next().unwrap();
        assert_eq!(<TokenId as FromPrimitive>::from_u8(token.id).unwrap(), TokenId::FSTRING_START);
        assert_eq!(num_read, 2);  // Width of "f'"

        let input = "hello {()}!'";
        let token = tokens_iter.next().unwrap();
        assert_eq!(<TokenId as FromPrimitive>::from_u8(token.id).unwrap(), TokenId::FSTRING_MIDDLE);
        assert_eq!(num_read, 7);  // Width of "hello {()}!"

        let input = "()}!'";
        let token = tokens_iter.next().unwrap();
        assert_eq!(<TokenId as FromPrimitive>::from_u8(token.id).unwrap(), TokenId::LPAR);
        assert_eq!(token.width, 1);  // Width of "("

        let input = ")}!";
        let token = tokens_iter.next().unwrap();
        assert_eq!(<TokenId as FromPrimitive>::from_u8(token.id).unwrap(), TokenId::RPAR);
        assert_eq!(token.width, 1);  // Width of ")"

        let input = "}!'";
        let token = tokens_iter.next().unwrap();
        assert_eq!(<TokenId as FromPrimitive>::from_u8(token.id).unwrap(), TokenId::FSTRING_MIDDLE);
        assert_eq!(num_read, 3);  // Width of "}!"

        let input = "'";
        let token = tokens_iter.next().unwrap();
        assert_eq!(<TokenId as FromPrimitive>::from_u8(token.id).unwrap(), TokenId::FSTRING_END);
        assert_eq!(num_read, 3);  // Width of "'"
    }

    #[test]
    fn test_keywords() {
        let mut tokenizer = PythonTokenizer::default();

        let input = "and pass in for if not or while";
        let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        let mut tokens_iter = tokens.iter();let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::AND as u8);

        let input = "pass in for if not or while";let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::PASS as u8);

        let input = "in for if not or while";let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::IN as u8);

        let input = "for if not or while";let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::FOR as u8);

        let input = "if not or while";let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::IF as u8);

        let input = "not or while";let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::NOT as u8);

        let input = "or while";let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::OR as u8);

        let input = "while";let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::WHILE as u8);
    }

    #[test]
    fn test_ignore_tokens() {
        let mut tokenizer = PythonTokenizer::default();

        let input = "   x ";
        let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        let mut tokens_iter = tokens.iter();
        let token = tokens_iter.next().unwrap();
        assert_eq!(token.id, TokenId::NAME as u8);
        assert_eq!(token.width, 4);
    }

    #[test]
    fn test_ignore_tokens_simple() {
        let mut tokenizer = PythonTokenizer::default();

        let input = "x ";
        let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        assert_eq!(tokens, vec![Token { id: TokenId::NAME as u8, width: 1 }]);

        let input = ", ";
        let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        assert_eq!(tokens, vec![Token { id: TokenId::COMMA as u8, width: 1 }]);

        // let input = "= ";
        // let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        // assert_eq!(tokens, vec![Token { id: TokenId::EQUAL as u8, width: 1 }]);

        // let input = "is ";
        // let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        // assert_eq!(tokens, vec![Token { id: TokenId::IS as u8, width: 1 }]);
    }

    #[test]
    fn test_exact_tokens() {
        let mut tokenizer = PythonTokenizer::default();

        let input = "= =+ // @ # this is a comment\n* % sadf";
        let (tokens, total_width, num_read) = tokenizer.tokenize(input);
        let mut tokens_iter = tokens.iter();
        let token = tokens_iter.next().unwrap();

        let expected_tokens = vec![
            TokenId::EQUAL,
            TokenId::PLUS,
            TokenId::DOUBLESLASH,
            TokenId::AT,
            TokenId::COMMENT,
            TokenId::NEWLINE,
            TokenId::STAR,
            TokenId::PERCENT,
            TokenId::NAME,
        ];

        for (token, expected_token) in tokens_iter.zip(expected_tokens.iter()) {
            let token_id = <TokenId as FromPrimitive>::from_u8(token.id).unwrap();
            assert_eq!(token_id, *expected_token);
        }
    }

    #[test]
    fn test_tokenize_cpython_compatible() {
        let input = "def f(x):\n    return x\n\nprint(f(1)) # should print 1";
        let rust_tokens = tokenize_cpython_compatible(input);
        let mut python_tokens =  vec![67, 1, 1, 7, 1, 8, 11, 4, 5, 1, 1, 4, 65, 6, 1, 7, 1, 7, 2, 8, 8, 64, 4, 0];
        // Filter out start token 67 and end token 0, NL token 65...
        python_tokens.retain(|&t| t != 67 && t != 0 && t != 65);
        // ...and any trailing NEWLINE tokens 4
        while python_tokens.len() > 0 && *python_tokens.last().unwrap() == 4 {
            python_tokens.pop();
        }
        assert_eq!(rust_tokens, python_tokens);
    }
}
