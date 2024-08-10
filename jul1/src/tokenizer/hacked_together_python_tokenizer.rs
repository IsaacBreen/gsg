use std::collections::{BTreeMap, HashMap};
use crate::tokenizer::finite_automata::RegexState;
use crate::tokenizer::tokenizer_trait::{Token, Tokenizer};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use regex::{Captures, escape, Regex};
use strum_macros::{EnumDiscriminants, EnumIter, EnumString};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, FromPrimitive)]
pub enum TokenId {
    ENDMARKER = 0,
    _NAME = 1,
    NUMBER = 2,
    STRING = 3,
    NEWLINE = 4,
    INDENT = 5,
    DEDENT = 6,
    LPAR = 7,
    RPAR = 8,
    LSQB = 9,
    RSQB = 10,
    COLON = 11,
    COMMA = 12,
    SEMI = 13,
    PLUS = 14,
    MINUS = 15,
    STAR = 16,
    SLASH = 17,
    VBAR = 18,
    AMPER = 19,
    LESS = 20,
    GREATER = 21,
    EQUAL = 22,
    DOT = 23,
    PERCENT = 24,
    LBRACE = 25,
    RBRACE = 26,
    EQEQUAL = 27,
    NOTEQUAL = 28,
    LESSEQUAL = 29,
    GREATEREQUAL = 30,
    TILDE = 31,
    CIRCUMFLEX = 32,
    LEFTSHIFT = 33,
    RIGHTSHIFT = 34,
    DOUBLESTAR = 35,
    PLUSEQUAL = 36,
    MINEQUAL = 37,
    STAREQUAL = 38,
    SLASHEQUAL = 39,
    PERCENTEQUAL = 40,
    AMPEREQUAL = 41,
    VBAREQUAL = 42,
    CIRCUMFLEXEQUAL = 43,
    LEFTSHIFTEQUAL = 44,
    RIGHTSHIFTEQUAL = 45,
    DOUBLESTAREQUAL = 46,
    DOUBLESLASH = 47,
    DOUBLESLASHEQUAL = 48,
    AT = 49,
    ATEQUAL = 50,
    RARROW = 51,
    ELLIPSIS = 52,
    COLONEQUAL = 53,
    EXCLAMATION = 54,
    OP = 55,
    LAMBDA = 56,
    IF = 57,
    TYPE_IGNORE = 58,
    TYPE_COMMENT = 59,
    SOFT_KEYWORD = 60,
    FSTRING_START = 61,
    FSTRING_MIDDLE = 62,
    FSTRING_END = 63,
    COMMENT = 64,
    NL = 65,
    ERRORTOKEN = 66,
    ENCODING = 67,
    N_TOKENS = 68,
    PASS = 69,
    BREAK = 70,
    CONTINUE = 71,
    RETURN = 72,
    RAISE = 73,
    GLOBAL = 74,
    NONLOCAL = 75,
    DEL = 76,
    ASSERT = 77,
    IMPORT = 78,
    FROM = 79,
    CLASS = 80,
    DEF = 81,
    ASYNC = 82,
    ELIF = 83,
    ELSE = 84,
    WHILE = 85,
    FOR = 86,
    IN = 87,
    WITH = 88,
    AS = 89,
    TRY = 90,
    EXCEPT = 91,
    FINALLY = 92,
    MATCH = 93,
    CASE = 94,
    NONE = 95,
    TRUE = 96,
    FALSE = 97,
    UNDERSCORE = 98,
    TYPE = 99,
    YIELD = 100,
    NOT = 101,
    IS = 102,
    AWAIT = 103,
    OR = 104,
    AND = 105,
    FSTRING_START_SINGLE_SINGLE_QUOTE = 106,
    FSTRING_START_SINGLE_DOUBLE_QUOTE = 107,
    FSTRING_START_TRIPLE_SINGLE_QUOTE = 108,
    FSTRING_START_TRIPLE_DOUBLE_QUOTE = 109,
    NEWLINE_AND_WHITESPACE = 254,
    IGNORE = 255,
    NT_OFFSET = 256,
    error,
}

pub(crate) fn get_token_ids() -> HashMap<String, TokenId> {
    use TokenId::*;
    let TOKEN_IDS: Vec<(&str, TokenId)> = vec![
        ("ENDMARKER", ENDMARKER),
        ("_NAME", _NAME),
        ("NUMBER", NUMBER),
        ("STRING", STRING),
        ("NEWLINE", NEWLINE),
        ("INDENT", INDENT),
        ("DEDENT", DEDENT),
        ("LPAR", LPAR),
        ("RPAR", RPAR),
        ("LSQB", LSQB),
        ("RSQB", RSQB),
        ("COLON", COLON),
        ("COMMA", COMMA),
        ("SEMI", SEMI),
        ("PLUS", PLUS),
        ("MINUS", MINUS),
        ("STAR", STAR),
        ("SLASH", SLASH),
        ("VBAR", VBAR),
        ("AMPER", AMPER),
        ("LESS", LESS),
        ("GREATER", GREATER),
        ("EQUAL", EQUAL),
        ("DOT", DOT),
        ("PERCENT", PERCENT),
        ("LBRACE", LBRACE),
        ("RBRACE", RBRACE),
        ("EQEQUAL", EQEQUAL),
        ("NOTEQUAL", NOTEQUAL),
        ("LESSEQUAL", LESSEQUAL),
        ("GREATEREQUAL", GREATEREQUAL),
        ("TILDE", TILDE),
        ("CIRCUMFLEX", CIRCUMFLEX),
        ("LEFTSHIFT", LEFTSHIFT),
        ("RIGHTSHIFT", RIGHTSHIFT),
        ("DOUBLESTAR", DOUBLESTAR),
        ("PLUSEQUAL", PLUSEQUAL),
        ("MINEQUAL", MINEQUAL),
        ("STAREQUAL", STAREQUAL),
        ("SLASHEQUAL", SLASHEQUAL),
        ("PERCENTEQUAL", PERCENTEQUAL),
        ("AMPEREQUAL", AMPEREQUAL),
        ("VBAREQUAL", VBAREQUAL),
        ("CIRCUMFLEXEQUAL", CIRCUMFLEXEQUAL),
        ("LEFTSHIFTEQUAL", LEFTSHIFTEQUAL),
        ("RIGHTSHIFTEQUAL", RIGHTSHIFTEQUAL),
        ("DOUBLESTAREQUAL", DOUBLESTAREQUAL),
        ("DOUBLESLASH", DOUBLESLASH),
        ("DOUBLESLASHEQUAL", DOUBLESLASHEQUAL),
        ("AT", AT),
        ("ATEQUAL", ATEQUAL),
        ("RARROW", RARROW),
        ("ELLIPSIS", ELLIPSIS),
        ("COLONEQUAL", COLONEQUAL),
        ("EXCLAMATION", EXCLAMATION),
        ("OP", OP),
        ("LAMBDA", LAMBDA),
        ("IF", IF),
        ("TYPE_IGNORE", TYPE_IGNORE),
        ("TYPE_COMMENT", TYPE_COMMENT),
        ("SOFT_KEYWORD", SOFT_KEYWORD),
        ("FSTRING_START", FSTRING_START),
        ("FSTRING_MIDDLE", FSTRING_MIDDLE),
        ("FSTRING_END", FSTRING_END),
        ("COMMENT", COMMENT),
        ("NL", NL),
        ("ERRORTOKEN", ERRORTOKEN),
        ("ENCODING", ENCODING),
        ("N_TOKENS", N_TOKENS),
        ("PASS", PASS),
        ("BREAK", BREAK),
        ("CONTINUE", CONTINUE),
        ("RETURN", RETURN),
        ("RAISE", RAISE),
        ("GLOBAL", GLOBAL),
        ("NONLOCAL", NONLOCAL),
        ("DEL", DEL),
        ("ASSERT", ASSERT),
        ("IMPORT", IMPORT),
        ("FROM", FROM),
        ("CLASS", CLASS),
        ("DEF", DEF),
        ("ASYNC", ASYNC),
        ("ELIF", ELIF),
        ("ELSE", ELSE),
        ("WHILE", WHILE),
        ("FOR", FOR),
        ("IN", IN),
        ("WITH", WITH),
        ("AS", AS),
        ("TRY", TRY),
        ("EXCEPT", EXCEPT),
        ("FINALLY", FINALLY),
        ("MATCH", MATCH),
        ("CASE", CASE),
        ("NONE", NONE),
        ("TRUE", TRUE),
        ("FALSE", FALSE),
        ("UNDERSCORE", UNDERSCORE),
        ("TYPE", TYPE),
        ("YIELD", YIELD),
        ("NOT", NOT),
        ("IS", IS),
        ("AWAIT", AWAIT),
        ("OR", OR),
        ("AND", AND),
        ("FSTRING_START_SINGLE_SINGLE_QUOTE", FSTRING_START_SINGLE_SINGLE_QUOTE),
        ("FSTRING_START_SINGLE_DOUBLE_QUOTE", FSTRING_START_SINGLE_DOUBLE_QUOTE),
        ("FSTRING_START_TRIPLE_SINGLE_QUOTE", FSTRING_START_TRIPLE_SINGLE_QUOTE),
        ("FSTRING_START_TRIPLE_DOUBLE_QUOTE", FSTRING_START_TRIPLE_DOUBLE_QUOTE),
        ("NEWLINE_AND_WHITESPACE", NEWLINE_AND_WHITESPACE),
        ("IGNORE", IGNORE),
        ("NT_OFFSET", NT_OFFSET),
        ("error", error),
    ];
    let mut token_id_map: HashMap<String, TokenId> = HashMap::new();
    for (name, id) in TOKEN_IDS {
        token_id_map.insert(name.to_string(), id.into());
    }
    token_id_map
}

impl From<u8> for TokenId {
    fn from(value: u8) -> Self {
        FromPrimitive::from_u8(value).unwrap()
    }
}

pub fn get_operators() -> Vec<(TokenId, String)> {
    let mut operators = Vec::new();
    operators.push((TokenId::EXCLAMATION, "!".to_string()));
    operators.push((TokenId::NOTEQUAL, "!=".to_string()));
    operators.push((TokenId::PERCENT, "%".to_string()));
    operators.push((TokenId::PERCENTEQUAL, "%=".to_string()));
    operators.push((TokenId::AMPER, "&".to_string()));
    operators.push((TokenId::AMPEREQUAL, "&=".to_string()));
    operators.push((TokenId::LPAR, "(".to_string()));
    operators.push((TokenId::RPAR, ")".to_string()));
    operators.push((TokenId::STAR, "*".to_string()));
    operators.push((TokenId::DOUBLESTAR, "**".to_string()));
    operators.push((TokenId::DOUBLESTAREQUAL, "**=".to_string()));
    operators.push((TokenId::STAREQUAL, "*=".to_string()));
    operators.push((TokenId::PLUS, "+".to_string()));
    operators.push((TokenId::PLUSEQUAL, "+=".to_string()));
    operators.push((TokenId::COMMA, ",".to_string()));
    operators.push((TokenId::MINUS, "-".to_string()));
    operators.push((TokenId::MINEQUAL, "-=".to_string()));
    operators.push((TokenId::RARROW, "->".to_string()));
    operators.push((TokenId::DOT, ".".to_string()));
    operators.push((TokenId::ELLIPSIS, "...".to_string()));
    operators.push((TokenId::SLASH, "/".to_string()));
    operators.push((TokenId::DOUBLESLASH, "//".to_string()));
    operators.push((TokenId::DOUBLESLASHEQUAL, "//=".to_string()));
    operators.push((TokenId::SLASHEQUAL, "/=".to_string()));
    operators.push((TokenId::COLON, ":".to_string()));
    operators.push((TokenId::COLONEQUAL, ":=".to_string()));
    operators.push((TokenId::SEMI, ";".to_string()));
    operators.push((TokenId::LESS, "<".to_string()));
    operators.push((TokenId::LEFTSHIFT, "<<".to_string()));
    operators.push((TokenId::LEFTSHIFTEQUAL, "<<=".to_string()));
    operators.push((TokenId::LESSEQUAL, "<=".to_string()));
    operators.push((TokenId::EQUAL, "=".to_string()));
    operators.push((TokenId::EQEQUAL, "==".to_string()));
    operators.push((TokenId::GREATER, ">".to_string()));
    operators.push((TokenId::GREATEREQUAL, ">=".to_string()));
    operators.push((TokenId::RIGHTSHIFT, ">>".to_string()));
    operators.push((TokenId::RIGHTSHIFTEQUAL, ">>=".to_string()));
    operators.push((TokenId::AT, "@".to_string()));
    operators.push((TokenId::ATEQUAL, "@=".to_string()));
    operators.push((TokenId::LSQB, "[".to_string()));
    operators.push((TokenId::RSQB, "]".to_string()));
    operators.push((TokenId::CIRCUMFLEX, "^".to_string()));
    operators.push((TokenId::CIRCUMFLEXEQUAL, "^=".to_string()));
    operators.push((TokenId::LBRACE, "{".to_string()));
    operators.push((TokenId::VBAR, "|".to_string()));
    operators.push((TokenId::VBAREQUAL, "|=".to_string()));
    operators.push((TokenId::RBRACE, "}".to_string()));
    operators.push((TokenId::TILDE, "~".to_string()));

    // Sort from longest to shortest
    operators.sort_by(|(_, a), (_, b)| b.len().cmp(&a.len()));

    operators
}

fn get_keywords() -> Vec<(TokenId, String)> {
    let mut keywords = Vec::new();
    keywords.push((TokenId::PASS, "pass".to_string()));
    keywords.push((TokenId::BREAK, "break".to_string()));
    keywords.push((TokenId::CONTINUE, "continue".to_string()));
    keywords.push((TokenId::RETURN, "return".to_string()));
    keywords.push((TokenId::RAISE, "raise".to_string()));
    keywords.push((TokenId::GLOBAL, "global".to_string()));
    keywords.push((TokenId::NONLOCAL, "nonlocal".to_string()));
    keywords.push((TokenId::DEL, "del".to_string()));
    keywords.push((TokenId::ASSERT, "assert".to_string()));
    keywords.push((TokenId::IMPORT, "import".to_string()));
    keywords.push((TokenId::FROM, "from".to_string()));
    keywords.push((TokenId::CLASS, "class".to_string()));
    keywords.push((TokenId::DEF, "def".to_string()));
    keywords.push((TokenId::ASYNC, "async".to_string()));
    keywords.push((TokenId::IF, "if".to_string()));
    keywords.push((TokenId::ELIF, "elif".to_string()));
    keywords.push((TokenId::ELSE, "else".to_string()));
    keywords.push((TokenId::WHILE, "while".to_string()));
    keywords.push((TokenId::FOR, "for".to_string()));
    keywords.push((TokenId::IN, "in".to_string()));
    keywords.push((TokenId::WITH, "with".to_string()));
    keywords.push((TokenId::AS, "as".to_string()));
    keywords.push((TokenId::TRY, "try".to_string()));
    keywords.push((TokenId::EXCEPT, "except".to_string()));
    keywords.push((TokenId::FINALLY, "finally".to_string()));
    keywords.push((TokenId::MATCH, "match".to_string()));
    keywords.push((TokenId::CASE, "case".to_string()));
    keywords.push((TokenId::NONE, "None".to_string()));
    keywords.push((TokenId::TRUE, "True".to_string()));
    keywords.push((TokenId::FALSE, "False".to_string()));
    keywords.push((TokenId::UNDERSCORE, "_".to_string()));
    keywords.push((TokenId::TYPE, "type".to_string()));
    keywords.push((TokenId::YIELD, "yield".to_string()));
    keywords.push((TokenId::NOT, "not".to_string()));
    keywords.push((TokenId::IS, "is".to_string()));
    keywords.push((TokenId::AWAIT, "await".to_string()));
    keywords.push((TokenId::LAMBDA, "lambda".to_string()));
    keywords.push((TokenId::OR, "or".to_string()));
    keywords.push((TokenId::AND, "and".to_string()));

    // Sort from longest to shortest
    keywords.sort_by(|(_, a), (_, b)| b.len().cmp(&a.len()));

    keywords
}

fn get_patterns_for_other_tokens() -> Vec<(TokenId, String)> {
    let mut patterns = Vec::new();

    patterns.push((TokenId::FSTRING_START_TRIPLE_SINGLE_QUOTE, "f'''".to_string()));
    patterns.push((TokenId::FSTRING_START_SINGLE_SINGLE_QUOTE, "f'".to_string()));
    patterns.push((TokenId::FSTRING_START_TRIPLE_DOUBLE_QUOTE, "f\"\"\"".to_string()));
    patterns.push((TokenId::FSTRING_START_SINGLE_DOUBLE_QUOTE, "f\"".to_string()));
    patterns.push((TokenId::NEWLINE, "[\\n\\r]".to_string()));
    patterns.push((TokenId::_NAME, r"[_\p{XID_Start}]\p{XID_Continue}*".to_string()));
    // patterns.push((TokenId::COMMENT, "#[^\\n]*".to_string()));
    patterns.push((TokenId::NUMBER, r"[-+]?((\d+(_\d+)*(\.\d*(_\d*)?)?|\.\d+(_\d*)?)([eE][-+]?\d+(_\d*)?)?|0[bB][01]+(_[01]+)*|0[oO][0-7]+(_[0-7]+)*|0[xX][\dA-Fa-f]+(_[\dA-Fa-f]+)*)[jJ]?".to_string()));
    patterns.push((TokenId::STRING, vec![
        r"'''((?s).*?)'''",
        r#""""((?s).*?)""""#,
        r"'([^'\\]*(\\.[^'\\]*)*)'",
        r#""([^"\\]*(\\.[^"\\]*)*)""#,
    ].join("|")));
    patterns.push((TokenId::IGNORE, r"([^\S\n\r]|\\\n)+(#[^\n]*)?".to_string()));

    patterns
}

pub fn get_all_tokens_as_patterns() -> Vec<(TokenId, String)> {
    get_operators().into_iter().map(|(k, v)| (k, escape(&v)))
        .chain(get_keywords())
        .chain(get_patterns_for_other_tokens())
        .collect()
}

#[derive(Debug, Clone)]
struct TokenMatcher {
    regex: Regex,
    regex_group_id_to_token_id: HashMap<usize, TokenId>,
    keyword_to_token_id: HashMap<String, TokenId>,
}

impl TokenMatcher {
    fn new() -> TokenMatcher {
        let patterns = get_patterns_for_other_tokens();
        let mut group_id_to_token_id = HashMap::new();
        let mut group_id = 1;
        for (token_id, pattern) in patterns.iter() {
            group_id_to_token_id.insert(group_id, *token_id);
            // Count the number of capture groups in the pattern.
            let capture_groups = Regex::new(pattern).unwrap().captures_len();
            group_id += capture_groups;
        }

        let mut pattern = patterns.iter().map(|(token_id, pattern)| format!("({})", pattern)).collect::<Vec<_>>().join("|");

        // Match from the beginning of the string
        pattern = format!("^(?:{})", pattern);

        let keyword_to_token_id = get_keywords().into_iter().map(|(k, v)| (v, k)).collect();

        TokenMatcher {
            regex: Regex::new(&pattern).unwrap(),
            regex_group_id_to_token_id: group_id_to_token_id,
            keyword_to_token_id,
        }
    }

    fn find(&self, input: &str) -> Option<Token> {
        // Match operators first.
        // The operators should already be sorted from longest to shortest.
        // Match byte-by-byte. If there isn't enough input to rule out a match, return None.

        for (token_id, operator) in get_operators() {
            if operator.len() <= input.len() {
                if input.starts_with(&operator) {
                    return Some(Token {
                        id: token_id as u8,
                        width: operator.len(),
                    });
                }
            } else {
                if operator.starts_with(input) {
                    // There isn't enough input to fully match this operator. But we might be able to rule it out.
                    // E.g. if the input is "*" then the operator could be "**", so we can't definitively match anything yet.
                    return None;
                } else {
                    // BUT it clearly can't be "!=", "/=", etc. So we can safely rule the operator in this case (i.e. just move on).
                }
            }
        }

        // Match other tokens.
        if let Some(captures) = self.regex.captures(input) {
            let group_id = get_group_id_from_regex_capture(&captures);
            let mut token_id = self.regex_group_id_to_token_id[&group_id];
            let capture_end = captures.get(group_id).unwrap().end();

            // For _NAME, COMMENT, NUMBER, FSTRING_START_SINGLE_SINGLE_QUOTE, or FSTRING_START_SINGLE_DOUBLE_QUOTE, we need to see the next character to be sure it's not part of this token.
            // If we can't see the next character yet (i.e. we've consumed all the input), return.
            if matches!(token_id, TokenId::_NAME | TokenId::COMMENT | TokenId::NUMBER | TokenId::FSTRING_START_SINGLE_SINGLE_QUOTE | TokenId::FSTRING_START_SINGLE_DOUBLE_QUOTE) {
                if capture_end == input.len() {
                    return None;
                }

                // A name can be a keyword.
                let matched = &input[..capture_end];
                if let Some(keyword_token_id) = self.keyword_to_token_id.get(matched) {
                    token_id = *keyword_token_id;
                }
            }

            return Some(Token {
                id: token_id as u8,
                width: capture_end,
            });
        }
        None
    }

    fn potential_token_ids(&self, mut input: &str) -> Vec<TokenId> {
        // Returns all valid next tokens IDs.
        // Note: this assumes there is a possible next token. If there isn't, it might behave in unexpected ways.
        // For example, if the input is "1a" (which can't possibly be a valid token), it will return [TokenId::NUMBER].
        // Note: this assumes you've already tokenized as far as you can with the given input.
        let mut possible_token_ids = Vec::new();

        // // By this point, the next character should be a non-whitespace character.
        // assert!(input.chars().next().map_or(true, |c| c.is_ascii()));

        // Skip to the next non-whitespace character (but don't skip newlines).
        while let Some(c) = input.chars().next() {
            if !c.is_ascii_whitespace() && c != '\n' {
                break;
            }
            input = &input[1..];
        }

        for (token_id, operator) in get_operators() {
            if operator.len() <= input.len() {
                if input.starts_with(&operator) {
                    possible_token_ids.push(token_id);
                }
            } else if operator.starts_with(input) {
                possible_token_ids.push(token_id);
            }
        }

        // Go through each case separately.
        // FString starts with f' or f"
        let possibilities = vec![
            "f'",
            "f\"",
        ];
        for possibility in possibilities {
            // If they share a prefix, we can match it.
            if input.starts_with(possibility) || possibility.starts_with(input) {
                possible_token_ids.push(TokenId::FSTRING_START);
            }
        }

        // As long as the next character is a whitespace, we could have a newline.
        if input.len() == 0 || input.chars().next().unwrap().is_ascii_whitespace() {
            possible_token_ids.push(TokenId::NEWLINE);
        }

        // For _NAME, we can match the usual pattern. As long as it consumes all the remaining input, we're good.
        // if input.len() == 0 || Regex::new(r"[_\p{XID_Start}]\p{XID_Continue}*").unwrap().is_match(input) {
        //     possible_token_ids.push(TokenId::_NAME);
        // }
        if let Some(captures) = Regex::new(r"([_\p{XID_Start}]\p{XID_Continue}*)?").unwrap().captures(input) {
            if captures.get(0).unwrap().end() == input.len() {
                possible_token_ids.push(TokenId::_NAME);
            }
        }

        // A comment starts with a hash.
        if input.len() == 0 || input.starts_with("#") {
            possible_token_ids.push(TokenId::COMMENT);
        }

        // A number always starts with a digit.
        if input.len() == 0 || input.chars().next().unwrap().is_ascii_digit() {
            possible_token_ids.push(TokenId::NUMBER);
        }

        // A string starts with a quote ' or "
        let possibilities = vec![
            "'",
            "\"",
        ];
        for possibility in possibilities {
            if input.starts_with(possibility) {
                possible_token_ids.push(TokenId::STRING);
            }
        }

        // An ignore should already have been matched by now (they're matched greedily).

        return possible_token_ids;
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum BracketType {
    Paren,
    Brace,
    Bracket,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum FStringType {
    SingleSingleQuote,
    SingleDoubleQuote,
    TripleSingleQuote,
    TripleDoubleQuote,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    /// Error mode. There's no recovering from this.
    Error,
}

#[derive(Debug, Clone)]
pub struct PythonTokenizer {
    pos: usize,
    mode_stack: Vec<PythonTokenizerMode>,
    indent_stack: Vec<String>,
    token_matcher: TokenMatcher,
}

impl Default for PythonTokenizer {
    fn default() -> Self {
        PythonTokenizer {
            pos: 0,
            mode_stack: vec![PythonTokenizerMode::Normal, PythonTokenizerMode::Newline],
            indent_stack: vec![],
            token_matcher: TokenMatcher::new(),
        }
    }
}

fn get_group_id_from_regex_capture(captures: &Captures) -> usize {
    // Prefer longer matches, then matches with lower group ID.
    let mut group_id = 0;
    let mut length = 0;
    for i in 1..captures.len() {
        if let Some(capture) = captures.get(i) {
            let capture_length = capture.end();
            if length < capture_length {
                group_id = i;
                length = capture_length;
            }
        }
    }
    group_id
}

impl Tokenizer for PythonTokenizer {
    fn tokenize(&mut self, input: &str) -> (Vec<Token>, usize, usize) {
        let mut tokens = Vec::new();

        // // If this is the beginning of the string, skip any blank lines and set the indentation level to the whitespace before the first non-whitespace character.
        // if self.indent_stack.is_empty() {
        //     let blank_line_regex = Regex::new(r"^\s*\n").unwrap();
        //     while let Some(captures) = blank_line_regex.captures(&input[self.pos..]) {
        //         self.pos += captures.get(0).unwrap().end();
        //     }
        //
        //     let indent_regex = Regex::new(r"^\s*").unwrap();
        //     let indent = indent_regex.captures(&input[self.pos..]).unwrap().get(0).unwrap().as_str();
        //
        //     // Ensure the next character is a non-whitespace character.
        //     if input.as_bytes().get(self.pos + indent.len()).is_none() {
        //         // We can't match the initial indent yet because we don't know where it begins.
        //         return (tokens, self.pos, 0);
        //     }
        //
        //     self.indent_stack.push(indent.to_string());
        //     self.pos += indent.len();
        // }

        while self.pos < input.len() {
            match self.mode_stack.last().unwrap() {
                PythonTokenizerMode::Normal | PythonTokenizerMode::Bracket(_) => {
                    // Keep parsing until we find a mode-change-triggering token
                    if let Some(token) = self.token_matcher.find(&input[self.pos..]) {
                        tokens.push(token);
                        self.pos += token.width;
                        let token_id: TokenId = token.id.into();
                        match token_id {
                            TokenId::NEWLINE => {
                                match self.mode_stack.last().unwrap() {
                                    PythonTokenizerMode::Normal => {
                                        self.mode_stack.push(PythonTokenizerMode::Newline);
                                    },
                                    PythonTokenizerMode::Bracket(_) => {
                                        // Ignore
                                        tokens.pop();
                                    },
                                    _ => unreachable!(),
                                }
                            },
                            TokenId::IGNORE => {
                                // Ignore
                                tokens.pop();
                            },
                            TokenId::LBRACE => {
                                self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Brace));
                            },
                            TokenId::LSQB => {
                                self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Bracket));
                            },
                            TokenId::LPAR => {
                                self.mode_stack.push(PythonTokenizerMode::Bracket(BracketType::Paren));
                            },
                            TokenId::RBRACE => {
                                if self.mode_stack.pop().unwrap() != PythonTokenizerMode::Bracket(BracketType::Brace) {
                                    self.mode_stack.push(PythonTokenizerMode::Error);
                                }
                            },
                            TokenId::RSQB => {
                                if self.mode_stack.pop().unwrap() != PythonTokenizerMode::Bracket(BracketType::Bracket) {
                                    self.mode_stack.push(PythonTokenizerMode::Error);
                                }
                            },
                            TokenId::RPAR => {
                                if self.mode_stack.pop().unwrap() != PythonTokenizerMode::Bracket(BracketType::Paren) {
                                    self.mode_stack.push(PythonTokenizerMode::Error);
                                }
                            },
                            TokenId::FSTRING_START_SINGLE_SINGLE_QUOTE | TokenId::FSTRING_START_SINGLE_DOUBLE_QUOTE | TokenId::FSTRING_START_TRIPLE_SINGLE_QUOTE | TokenId::FSTRING_START_TRIPLE_DOUBLE_QUOTE => {
                                self.mode_stack.push(PythonTokenizerMode::FString(match token_id {
                                    TokenId::FSTRING_START_SINGLE_SINGLE_QUOTE => FStringType::SingleSingleQuote,
                                    TokenId::FSTRING_START_SINGLE_DOUBLE_QUOTE => FStringType::SingleDoubleQuote,
                                    TokenId::FSTRING_START_TRIPLE_SINGLE_QUOTE => FStringType::TripleSingleQuote,
                                    TokenId::FSTRING_START_TRIPLE_DOUBLE_QUOTE => FStringType::TripleDoubleQuote,
                                    _ => unreachable!(),
                                }));
                                tokens.last_mut().unwrap().id = TokenId::FSTRING_START as u8;
                            },
                            _ => {}
                        }
                    } else {
                        break;
                    }
                },
                PythonTokenizerMode::FString(fstring_type) => {
                    // Try to match everything up to the closing quote or a single left brace (but not a double brace).
                    let quotes = match fstring_type {
                        FStringType::SingleSingleQuote => "'",
                        FStringType::SingleDoubleQuote => "\"",
                        FStringType::TripleSingleQuote => "'''",
                        FStringType::TripleDoubleQuote => "\"\"\"",
                    };
                    let regex = match fstring_type {
                        FStringType::SingleSingleQuote | FStringType::SingleDoubleQuote => {
                            Regex::new(&(r"([^{\n]|\{\{)*(\{\{|".to_string() + quotes + ")"))
                        }
                        FStringType::TripleSingleQuote | FStringType::TripleDoubleQuote => {
                            // If it's one of the triple quotes, also match a newline.
                            Regex::new(&(r"([^\s\S]|\{\{)*(\{\{|".to_string() + quotes + "|\\n)"))
                        }
                    }.unwrap();
                    if let Some(captures) = regex.captures(&input[self.pos..]) {
                        // Get the last capture, which is the closing quote or the brace.
                        let capture = captures.get(captures.len() - 1).unwrap();
                        let middle_width = capture.end() - quotes.len();
                        if middle_width > 0 {
                            tokens.push(Token {
                                id: TokenId::FSTRING_MIDDLE as u8,
                                width: middle_width,
                            });
                        }
                        tokens.push(Token {
                            id: TokenId::FSTRING_END as u8,
                            width: quotes.len(),
                        });
                        self.pos += capture.end();
                        self.mode_stack.pop();
                    } else {
                        break;
                    }
                },
                PythonTokenizerMode::Newline => {
                    // First, skip any completely blank lines (including comments).
                    let mut pos = self.pos;
                    let blank_line_regex = Regex::new(r"^\s*(#[^\n]*)?\n").unwrap();
                    while let Some(captures) = blank_line_regex.captures(&input[pos..]) {
                        pos += captures.get(0).unwrap().end();
                    }

                    // Get the indent.
                    let indent_regex = Regex::new(r"^\s*").unwrap();
                    let indent = indent_regex.captures(&input[pos..]).unwrap().get(0).unwrap().as_str();
                    let mut remaining_indent = indent;

                    // For reasons similar to those discussed below (see the indentation handler for the `potential_token_ids` function),
                    // we can only match an indentation if we haven't reached the end of the input yet.
                    if pos + indent.len() == input.len() {
                        break;
                    }

                    // Match each existing indent.
                    let mut i = 0;
                    while i < self.indent_stack.len() {
                        let expected_indent = &self.indent_stack[i];
                        if let Some(new_remaining_indent) = remaining_indent.strip_prefix(expected_indent) {
                            remaining_indent = new_remaining_indent;
                            i += 1;
                        } else {
                            break;
                        }
                    }

                    if !remaining_indent.is_empty() && i < self.indent_stack.len() {
                        if !self.indent_stack[i].starts_with(remaining_indent) {
                            // Invalid indentation
                            self.mode_stack.push(PythonTokenizerMode::Error);
                        }
                        break;
                    } else if i < self.indent_stack.len() {
                        // Dedent
                        for _ in i..self.indent_stack.len() {
                            self.indent_stack.pop();
                            tokens.push(Token {
                                id: TokenId::DEDENT as u8,
                                width: 0,
                            });
                        }
                        // The last dedent token gets the full width of the indent.
                        tokens.last_mut().unwrap().width = indent.len();
                    } else if !remaining_indent.is_empty() {
                        // Indent
                        self.indent_stack.push(remaining_indent.to_string());
                        tokens.push(Token {
                            id: TokenId::INDENT as u8,
                            width: indent.len(),
                        });
                    } else {
                        // Same indentation level. No action required (just pop the indent mode).
                    }

                    self.mode_stack.pop();
                    self.pos = pos;
                }
                PythonTokenizerMode::Error => {
                    // We can't match anything here.
                    // TODO: error token?
                    break;
                }
            }
        }

        (tokens, 0, 0)
    }

    fn potential_token_ids(&self, input: &str) -> Vec<u8> {
        // Returns all valid next tokens IDs. A token ID is valid if and only if it is returned by this function.
        // Note: This only returns all *lexically*-valid tokens. They aren't necessarily grammatically valid.
        // Note: this assumes you've already tokenized as far as you can with the given input.
        let mut possible_token_ids = Vec::new();

        let mode = self.mode_stack.last().unwrap();
        match mode {
            PythonTokenizerMode::Normal => {
                for token_id in self.token_matcher.potential_token_ids(&input[self.pos..]) {
                    // Include all next tokens except ignored ones and closing brackets.
                    if matches!(token_id, TokenId::IGNORE | TokenId::RPAR | TokenId::RBRACE | TokenId::RSQB) {
                        continue;
                    }
                    possible_token_ids.push(token_id);
                }
            }
            PythonTokenizerMode::Bracket(bracket_type) => {
                for token_id in self.token_matcher.potential_token_ids(&input[self.pos..]) {
                    // Include all next tokens except ignored ones and closing brackets other than the current one.
                    let closing_bracket = match bracket_type {
                        BracketType::Paren => TokenId::RPAR,
                        BracketType::Brace => TokenId::RBRACE,
                        BracketType::Bracket => TokenId::RSQB,
                    };
                    if matches!(token_id, TokenId::IGNORE | TokenId::NEWLINE | TokenId::RPAR | TokenId::RBRACE | TokenId::RSQB) {
                        if token_id != closing_bracket {
                            continue;
                        }
                    }
                    possible_token_ids.push(token_id);
                }
            }
            PythonTokenizerMode::FString(fstring_type) => {
                // In FString mode, we can always match a middle token.
                possible_token_ids.push(TokenId::FSTRING_MIDDLE);

                // However, we might be restricted from matching a closing quote.
                let close_quotes: &str = match fstring_type {
                    FStringType::SingleSingleQuote => "'",
                    FStringType::SingleDoubleQuote => r#"""#,
                    FStringType::TripleSingleQuote => "'''",
                    FStringType::TripleDoubleQuote => r#"""""#,
                };
                if close_quotes.starts_with(&input[self.pos..]) || input[self.pos..].starts_with(close_quotes) {
                    possible_token_ids.push(TokenId::FSTRING_END);
                }
            }
            PythonTokenizerMode::Newline => {
                // This is much easier than it seems.
                // Suppose we start writing the next line.
                // At the beginning, anything is possible! Indent, dedent, or just the next normal-mode token.
                // Now suppose we type some spaces. After some number of spaces, we'll be at the same indentation level.
                // Are we now locked out of dedenting?
                // Nope! Just chuck in a newline, and we're back to the beginning of the next (logical) line.
                // The blank link we just created above it will get ignored.
                if !self.indent_stack.is_empty() {
                    // However, we don't emit an indent at the beginning of the string when matching the initial indent level.
                    possible_token_ids.push(TokenId::INDENT);
                    possible_token_ids.push(TokenId::DEDENT);
                }
                for token_id in self.token_matcher.potential_token_ids("") {
                    if matches!(token_id, TokenId::IGNORE | TokenId::NEWLINE | TokenId::RPAR | TokenId::RBRACE | TokenId::RSQB) {
                        continue;
                    }
                    possible_token_ids.push(token_id);
                }
            }
            PythonTokenizerMode::Error => {
                // We can't match anything here.
                // TODO: error token?
            }
        }

        possible_token_ids.into_iter().map(|token_id| token_id as u8).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::FromPrimitive;

    fn tokenize_ids(input: &str) -> Vec<TokenId> {
        let mut tokenizer = PythonTokenizer::default();
        let (tokens, _, _) = tokenizer.tokenize(input);
        tokens
            .iter()
            .map(|token| <TokenId as FromPrimitive>::from_u8(token.id).unwrap())
            .collect()
    }

    #[test]
    fn test_simple_cases() {
        let input = "a = 1";
        let expected = vec![
            TokenId::_NAME,
            TokenId::EQUAL,
            // Can't match the last number without seeing the next char (e.g. could be "1.0")
            // TokenId::NUMBER,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let input = "if else while def class pass break continue";
        let expected = vec![
            TokenId::IF,
            TokenId::ELSE,
            TokenId::WHILE,
            TokenId::DEF,
            TokenId::CLASS,
            TokenId::PASS,
            TokenId::BREAK,
            // We can't match the final "continue" yet because, although the rest of the input could just be "continue\n", it could also be something like "continue_variable\n".
            // In the former case, it'd be a "continue" token, but in the latter case, it'd be a _NAME.
            // TokenId::CONTINUE,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }

    #[test]
    fn test_operators_and_punctuation() {
        let input = "+ - * / == != <= >= -> : := ** //";
        let expected = vec![
            TokenId::PLUS,
            TokenId::MINUS,
            TokenId::STAR,
            TokenId::SLASH,
            TokenId::EQEQUAL,
            TokenId::NOTEQUAL,
            TokenId::LESSEQUAL,
            TokenId::GREATEREQUAL,
            TokenId::RARROW,
            TokenId::COLON,
            TokenId::COLONEQUAL,
            TokenId::DOUBLESTAR,
            // We can't match the final "//" yet because it could be "//" or "//=" - we need to see the next character to know.
            // TokenId::DOUBLESLASH,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }

    #[test]
    fn test_fstrings() {
        let input = "f'Hello, {name}!'";
        let expected = vec![
            TokenId::FSTRING_START,
            TokenId::FSTRING_MIDDLE,
            TokenId::FSTRING_END,
        ];
        assert_eq!(tokenize_ids(input), expected);

        let input = "f\"Welcome, {user}!\"";
        let expected = vec![
            TokenId::FSTRING_START,
            TokenId::FSTRING_MIDDLE,
            TokenId::FSTRING_END,
        ];
        assert_eq!(tokenize_ids(input), expected);

        let input = "f'''Complex {fstring} example'''";
        let expected = vec![
            TokenId::FSTRING_START,
            TokenId::FSTRING_MIDDLE,
            TokenId::FSTRING_END,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }

    #[test]
    fn test_comments_and_newlines() {
        let input = "# This is a comment\nx = 42\n# Another comment";
        let expected = vec![
            TokenId::COMMENT,
            TokenId::NEWLINE,
            TokenId::_NAME,
            TokenId::EQUAL,
            TokenId::NUMBER,
            TokenId::NEWLINE,
            // Can't match the last comment without seeing the next char (e.g. could be "# Another comment surprise!")
            // TokenId::COMMENT,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }

    #[test]
    fn test_brackets_and_braces() {
        let input = "(x + y) * {a: b} [1, 2, 3]";
        let expected = vec![
            TokenId::LPAR,
            TokenId::_NAME,
            TokenId::PLUS,
            TokenId::_NAME,
            TokenId::RPAR,
            TokenId::STAR,
            TokenId::LBRACE,
            TokenId::_NAME,
            TokenId::COLON,
            TokenId::_NAME,
            TokenId::RBRACE,
            TokenId::LSQB,
            TokenId::NUMBER,
            TokenId::COMMA,
            TokenId::NUMBER,
            TokenId::COMMA,
            TokenId::NUMBER,
            TokenId::RSQB,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }

    #[test]
    fn test_strings() {
        let input = "'single quotes' \"double quotes\" '''triple single''' \"\"\"triple double\"\"\"";
        let expected = vec![
            TokenId::STRING,
            TokenId::STRING,
            TokenId::STRING,
            TokenId::STRING,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }

    #[test]
    fn test_indentation() {
        let input = "if True:\n    print('Indented')\nelse:\n    pass\n";
        let expected = vec![
            TokenId::IF,
            TokenId::TRUE,
            TokenId::COLON,
            TokenId::NEWLINE,
            TokenId::INDENT,
            TokenId::_NAME,
            TokenId::LPAR,
            TokenId::STRING,
            TokenId::RPAR,
            TokenId::NEWLINE,
            TokenId::DEDENT,
            TokenId::ELSE,
            TokenId::COLON,
            TokenId::NEWLINE,
            TokenId::INDENT,
            TokenId::PASS,
            TokenId::NEWLINE,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }

    #[test]
    fn test_numbers() {
        let input = "0 123 3.14 0.001 1e10 3.14e-10";
        let expected = vec![
            TokenId::NUMBER,
            TokenId::NUMBER,
            TokenId::NUMBER,
            TokenId::NUMBER,
            TokenId::NUMBER,
            // Can't match the last one without seeing the next char (e.g. could be "3.14e-100")
            // TokenId::NUMBER,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }

    #[test]
    fn test_underscore_in_identifiers_and_numbers() {
        let input = "_private 1_000 0x1_00";
        let expected = vec![
            TokenId::_NAME,
            TokenId::NUMBER,
            TokenId::NUMBER,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }

    #[test]
    fn test_special_tokens() {
        let input = "not and or is in lambda yield await ";
        let expected = vec![
            TokenId::NOT,
            TokenId::AND,
            TokenId::OR,
            TokenId::IS,
            TokenId::IN,
            TokenId::LAMBDA,
            TokenId::YIELD,
            TokenId::AWAIT,
        ];
        assert_eq!(tokenize_ids(input), expected);
    }
}

#[cfg(test)]
mod test_potential_token_ids {
    use super::*;
    use TokenId::*;

    fn run_test(input: &str, mut expected: Vec<TokenId>) {
        let mut tokenizer = PythonTokenizer::default();
        tokenizer.tokenize(input);
        let mut actual: Vec<TokenId> = tokenizer.potential_token_ids(input).into_iter().map(|id| Into::<TokenId>::into(id)).collect();
        expected.sort();
        actual.sort();
        assert_eq!(actual, expected);
    }

    fn get_all_normal_token_ids() -> Vec<TokenId> {
        vec![DOUBLESTAREQUAL, ELLIPSIS, DOUBLESLASHEQUAL, LEFTSHIFTEQUAL, RIGHTSHIFTEQUAL, NOTEQUAL, PERCENTEQUAL, AMPEREQUAL, DOUBLESTAR, STAREQUAL, PLUSEQUAL, MINEQUAL, RARROW, DOUBLESLASH, SLASHEQUAL, COLONEQUAL, LEFTSHIFT, LESSEQUAL, EQEQUAL, GREATEREQUAL, RIGHTSHIFT, ATEQUAL, CIRCUMFLEXEQUAL, VBAREQUAL, EXCLAMATION, PERCENT, AMPER, LPAR, STAR, PLUS, COMMA, MINUS, DOT, SLASH, COLON, SEMI, LESS, EQUAL, GREATER, AT, LSQB, CIRCUMFLEX, LBRACE, VBAR, TILDE, FSTRING_START, FSTRING_START, _NAME, COMMENT, NUMBER]
    }

    #[test]
    fn test_potential_token_ids_normal_mode() {
        let mut expected = get_all_normal_token_ids();

        run_test("", expected.clone());
        run_test(" ", expected.clone());
        run_test("\n", expected.clone());

        run_test("x", vec![TokenId::_NAME]);

        expected.push(TokenId::NEWLINE);
        run_test("x ", expected.clone());
        run_test("\"x\"", expected.clone());
    }

    #[test]
    fn test_potential_token_ids_fstring_mode() {
        // Single quotes
        run_test("f'", vec![TokenId::FSTRING_START]);
        run_test("f'hello", vec![TokenId::FSTRING_MIDDLE]);

        // Double quotes
        run_test(r#"f"world"#, vec![TokenId::FSTRING_MIDDLE]);

        // Triple single quotes
        run_test("f'''", vec![TokenId::FSTRING_MIDDLE, TokenId::FSTRING_END]);
        run_test("f'''''", vec![TokenId::FSTRING_MIDDLE, TokenId::FSTRING_END]);
        run_test("f'''multiline", vec![TokenId::FSTRING_MIDDLE]);

        // Termination
        let mut expected = get_all_normal_token_ids();
        expected.push(TokenId::NEWLINE);
        run_test(r#"f"""string""""#, expected);
    }

    #[test]
    fn test_potential_token_ids_indentation() {
        let mut expected = get_all_normal_token_ids();
        expected.push(INDENT);
        expected.push(DEDENT);
        run_test("if True:\n    if True:\n", expected.clone());
        run_test("if True:\n    if True:\n", expected.clone());
        run_test("if True:\n    if True:\n        x\n", expected.clone());
        run_test("if True:\n    if True:\n        x\n ", expected.clone());
        run_test("if True:\n    if True:\n        x\n    ", expected.clone());
        run_test("if True:\n    if True:\n        x\n        ", expected.clone());
        run_test("if True:\n    if True:\n        x\n            ", expected.clone());
    }

    #[test]
    fn test_potential_token_ids_bracket_mode() {
        let mut expected = get_all_normal_token_ids();
        expected.push(TokenId::RPAR);
        run_test("(", expected.clone());
        run_test("(x", vec![_NAME]);
        run_test("(x,", expected.clone());
    }
}

#[cfg(test)]
mod test_historical_failure_cases {
    use super::*;
    use TokenId::*;

    fn check_tokens(input: &str, expected: Vec<TokenId>) {
        let mut tokenizer = PythonTokenizer::default();
        let (tokens, _, _) = tokenizer.tokenize(input);
        let actual: Vec<TokenId> = tokens
            .iter()
            .map(|token| <TokenId as FromPrimitive>::from_u8(token.id).unwrap())
            .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_fibonacci() {
        let input = "def fibonacci(n):\n  a, b = 0, 1\n  for _ in range(n):\n    yield a\n    a, b = b, a + b\n";

        let expected = vec![
            DEF, _NAME, LPAR, _NAME, RPAR, COLON, NEWLINE, INDENT,
            _NAME, COMMA, _NAME, EQUAL, NUMBER, COMMA, NUMBER, NEWLINE,
            FOR, UNDERSCORE, IN, _NAME, LPAR, _NAME, RPAR, COLON, NEWLINE, INDENT,
            YIELD, _NAME, NEWLINE,
            _NAME, COMMA, _NAME, EQUAL, _NAME, COMMA, _NAME, PLUS, _NAME, NEWLINE,
        ];
        check_tokens(input, expected);
    }

    #[test]
    fn test_line_comment() {
        let input = "# This is a comment\nprint('Hello, world!')\n";

        let expected = vec![
            _NAME, LPAR, STRING, RPAR, NEWLINE,
        ];
        check_tokens(input, expected);
    }

    #[test]
    fn test_inline_comment() {
        let input = "print('Hello, world!') # This is a comment\n";

        let expected = vec![
            _NAME, LPAR, STRING, RPAR, NEWLINE,
        ];
        check_tokens(input, expected);
    }

    #[test]
    fn test_newline_escape() {
        let input = "total = 10 +\\\n  20 +\\\n  30\\\n";

        let expected = vec![
            _NAME, EQUAL, NUMBER, PLUS, NUMBER, PLUS, NUMBER,
        ];
        check_tokens(input, expected);
    }
}