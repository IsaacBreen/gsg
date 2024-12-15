use crate::glr::grammar::{nt, prod, t, Terminal};
use crate::glr::parser::GLRParser;
use crate::glr::table::{generate_glr_parser, TerminalID};

#[test]
fn test_simple_parse_table() {
    let productions = vec![
        prod("S", vec![nt("A")]),
        prod("A", vec![nt("A"), t("a")]),
        prod("A", vec![t("b")]),
    ];

    let parser = generate_glr_parser(&productions, 0);
    let tokenize = |input: &str, parser: &GLRParser| -> Vec<TerminalID> {
        input.chars()
            .filter_map(|c| parser.terminal_map.get_by_left(&Terminal(c.to_string())).copied())
            .collect()
    };

    assert!(parser.parse(&tokenize("b", &parser)).fully_matches());
    assert!(parser.parse(&tokenize("ba", &parser)).fully_matches());
    assert!(parser.parse(&tokenize("baa", &parser)).fully_matches());

    assert!(!parser.parse(&tokenize("a", &parser)).fully_matches());
    assert!(!parser.parse(&tokenize("bb", &parser)).fully_matches());
}

#[test]
fn test_parse_simple_expression() {
    let productions = vec![
        prod("S", vec![nt("E")]),
        prod("E", vec![nt("E"), t("+"), nt("T")]),
        prod("E", vec![nt("T")]),
        prod("T", vec![nt("T"), t("*"), nt("F")]),
        prod("T", vec![nt("F")]),
        prod("F", vec![t("("), nt("E"), t(")")]),
        prod("F", vec![t("i")]),
    ];

    let parser = generate_glr_parser(&productions, 0);
    let tokenize = |input: &str, parser: &GLRParser| -> Vec<TerminalID> {
        input.chars()
            .filter_map(|c| parser.terminal_map.get_by_left(&Terminal(c.to_string())).copied())
            .collect()
    };

    let valid_inputs = ["i", "i+i*i", "i+i", "i*i", "(i+i)*i"];
    let invalid_inputs = ["i+", "i++i", "", ")"];

    for input in valid_inputs {
        assert!(parser.parse(&tokenize(input, &parser)).fully_matches(), "Failed for valid input: {}", input);
    }

    for input in invalid_inputs {
        assert!(!parser.parse(&tokenize(input, &parser)).fully_matches(), "Failed for invalid input: {}", input);
    }
}
