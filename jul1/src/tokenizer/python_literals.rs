use std::collections::HashMap;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, PartialEq, Debug, FromPrimitive)]
pub enum TokenId {
    ENDMARKER = 0,
    NAME = 1,
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
}

pub fn get_python_literals() -> HashMap<usize, String> {
    let mut map = HashMap::new();
    map.insert(TokenId::EXCLAMATION as usize, "!".to_string());
    map.insert(TokenId::NOTEQUAL as usize, "!=".to_string());
    map.insert(TokenId::PERCENT as usize, "%".to_string());
    map.insert(TokenId::PERCENTEQUAL as usize, "%=".to_string());
    map.insert(TokenId::AMPER as usize, "&".to_string());
    map.insert(TokenId::AMPEREQUAL as usize, "&=".to_string());
    map.insert(TokenId::LPAR as usize, "(".to_string());
    map.insert(TokenId::RPAR as usize, ")".to_string());
    map.insert(TokenId::STAR as usize, "*".to_string());
    map.insert(TokenId::DOUBLESTAR as usize, "**".to_string());
    map.insert(TokenId::DOUBLESTAREQUAL as usize, "**=".to_string());
    map.insert(TokenId::STAREQUAL as usize, "*=".to_string());
    map.insert(TokenId::PLUS as usize, "+".to_string());
    map.insert(TokenId::PLUSEQUAL as usize, "+=".to_string());
    map.insert(TokenId::COMMA as usize, ",".to_string());
    map.insert(TokenId::MINUS as usize, "-".to_string());
    map.insert(TokenId::MINEQUAL as usize, "-=".to_string());
    map.insert(TokenId::RARROW as usize, "->".to_string());
    map.insert(TokenId::DOT as usize, ".".to_string());
    map.insert(TokenId::ELLIPSIS as usize, "...".to_string());
    map.insert(TokenId::SLASH as usize, "/".to_string());
    map.insert(TokenId::DOUBLESLASH as usize, "//".to_string());
    map.insert(TokenId::DOUBLESLASHEQUAL as usize, "//=".to_string());
    map.insert(TokenId::SLASHEQUAL as usize, "/=".to_string());
    map.insert(TokenId::COLON as usize, ":".to_string());
    map.insert(TokenId::COLONEQUAL as usize, ":=".to_string());
    map.insert(TokenId::SEMI as usize, ";".to_string());
    map.insert(TokenId::LESS as usize, "<".to_string());
    map.insert(TokenId::LEFTSHIFT as usize, "<<".to_string());
    map.insert(TokenId::LEFTSHIFTEQUAL as usize, "<<=".to_string());
    map.insert(TokenId::LESSEQUAL as usize, "<=".to_string());
    map.insert(TokenId::EQUAL as usize, "=".to_string());
    map.insert(TokenId::EQEQUAL as usize, "==".to_string());
    map.insert(TokenId::GREATER as usize, ">".to_string());
    map.insert(TokenId::GREATEREQUAL as usize, ">=".to_string());
    map.insert(TokenId::RIGHTSHIFT as usize, ">>".to_string());
    map.insert(TokenId::RIGHTSHIFTEQUAL as usize, ">>=".to_string());
    map.insert(TokenId::AT as usize, "@".to_string());
    map.insert(TokenId::ATEQUAL as usize, "@=".to_string());
    map.insert(TokenId::LSQB as usize, "[".to_string());
    map.insert(TokenId::RSQB as usize, "]".to_string());
    map.insert(TokenId::CIRCUMFLEX as usize, "^".to_string());
    map.insert(TokenId::CIRCUMFLEXEQUAL as usize, "^=".to_string());
    map.insert(TokenId::LBRACE as usize, "{".to_string());
    map.insert(TokenId::VBAR as usize, "|".to_string());
    map.insert(TokenId::VBAREQUAL as usize, "|=".to_string());
    map.insert(TokenId::RBRACE as usize, "}".to_string());
    map.insert(TokenId::TILDE as usize, "~".to_string());
    map.insert(TokenId::PASS as usize, "pass".to_string());
    map.insert(TokenId::BREAK as usize, "break".to_string());
    map.insert(TokenId::CONTINUE as usize, "continue".to_string());
    map.insert(TokenId::RETURN as usize, "return".to_string());
    map.insert(TokenId::RAISE as usize, "raise".to_string());
    map.insert(TokenId::GLOBAL as usize, "global".to_string());
    map.insert(TokenId::NONLOCAL as usize, "nonlocal".to_string());
    map.insert(TokenId::DEL as usize, "del".to_string());
    map.insert(TokenId::ASSERT as usize, "assert".to_string());
    map.insert(TokenId::IMPORT as usize, "import".to_string());
    map.insert(TokenId::FROM as usize, "from".to_string());
    map.insert(TokenId::CLASS as usize, "class".to_string());
    map.insert(TokenId::DEF as usize, "def".to_string());
    map.insert(TokenId::ASYNC as usize, "async".to_string());
    map.insert(TokenId::IF as usize, "if".to_string());
    map.insert(TokenId::ELIF as usize, "elif".to_string());
    map.insert(TokenId::ELSE as usize, "else".to_string());
    map.insert(TokenId::WHILE as usize, "while".to_string());
    map.insert(TokenId::FOR as usize, "for".to_string());
    map.insert(TokenId::IN as usize, "in".to_string());
    map.insert(TokenId::WITH as usize, "with".to_string());
    map.insert(TokenId::AS as usize, "as".to_string());
    map.insert(TokenId::TRY as usize, "try".to_string());
    map.insert(TokenId::EXCEPT as usize, "except".to_string());
    map.insert(TokenId::FINALLY as usize, "finally".to_string());
    map.insert(TokenId::MATCH as usize, "match".to_string());
    map.insert(TokenId::CASE as usize, "case".to_string());
    map.insert(TokenId::NONE as usize, "None".to_string());
    map.insert(TokenId::TRUE as usize, "True".to_string());
    map.insert(TokenId::FALSE as usize, "False".to_string());
    map.insert(TokenId::UNDERSCORE as usize, "_".to_string());
    map.insert(TokenId::TYPE as usize, "type".to_string());
    map.insert(TokenId::YIELD as usize, "yield".to_string());
    map.insert(TokenId::NOT as usize, "not".to_string());
    map.insert(TokenId::IS as usize, "is".to_string());
    map.insert(TokenId::AWAIT as usize, "await".to_string());
    map.insert(TokenId::LAMBDA as usize, "lambda".to_string());
    map.insert(TokenId::OR as usize, "or".to_string());
    map.insert(TokenId::AND as usize, "and".to_string());
    map
}

pub fn get_python_non_literal_token_patterns() -> HashMap<usize, String> {
    let mut map = HashMap::new();
    map.insert(TokenId::COMMENT as usize, "#[^\n]*".to_string());
    map.insert(TokenId::NAME as usize, "[a-zA-Z_][a-zA-Z0-9_]*".to_string());
    map.insert(TokenId::NUMBER as usize, "[0-9]+".to_string());
    map.insert(TokenId::STRING as usize, vec![
            r"'.*'",
            r"'''[\s\S]*'''",
            r#"".*""#,
            r#""""[\s\S]*""""#,
        ].join("|"));
    // A newline followed by zero or more whitespaces. Could be a NEWLINE, INDENT, DEDENT, or a blank (ignored) line depending on the context.
    map.insert(TokenId::NEWLINE_AND_WHITESPACE as usize, r"\n[^\S\n\r]*".to_string());
    map.insert(TokenId::IGNORE as usize, r"[^\S\n\r]+".to_string());
    map.insert(TokenId::FSTRING_START_SINGLE_SINGLE_QUOTE as usize, "f'".to_string());
    map.insert(TokenId::FSTRING_START_SINGLE_DOUBLE_QUOTE as usize, "f\"".to_string());
    map.insert(TokenId::FSTRING_START_TRIPLE_SINGLE_QUOTE as usize, "f'''".to_string());
    map.insert(TokenId::FSTRING_START_TRIPLE_DOUBLE_QUOTE as usize, "f\"\"\"".to_string());
    map
}
