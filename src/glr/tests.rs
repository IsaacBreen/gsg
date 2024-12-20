use crate::glr::grammar::{nt, prod, t, Terminal};
use crate::glr::parser::GLRParser;
use crate::glr::table::{generate_glr_parser, TerminalID};

#[test]
fn test_simple_parse_table() {
    let productions = vec![
        // S -> A
        prod("S", vec![nt("A")]),
        // A -> A a | b
        prod("A", vec![nt("A"), t("a")]),
        prod("A", vec![t("b")]),
    ];

    let parser = generate_glr_parser(&productions, 0);

    println!("{}", parser);

    let tokenize = |input: &str, parser: &GLRParser| -> Vec<TerminalID> {
        input.chars().filter_map(|c| parser.terminal_map.get_by_left(&Terminal(c.to_string()))
            .copied()).collect()
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
        // S -> E
        prod("S", vec![nt("E")]),
        // E -> E + T
        prod("E", vec![nt("E"), t("+"), nt("T")]),
        // E -> T
        prod("E", vec![nt("T")]),
        // T -> T * F
        prod("T", vec![nt("T"), t("*"), nt("F")]),
        // T -> F
        prod("T", vec![nt("F")]),
        // F -> ( E )
        prod("F", vec![t("("), nt("E"), t(")")]),
        // F -> i
        prod("F", vec![t("i")]),
    ];

    let parser = generate_glr_parser(&productions, 0);

    println!("{}", parser);

    let tokenize = |input: &str, parser: &GLRParser| -> Vec<TerminalID> {
        input.chars().filter_map(|c| parser.terminal_map.get_by_left(&Terminal(c.to_string()))
            .copied()).collect()
    };

    assert!(parser.parse(&tokenize("i", &parser)).fully_matches());
    assert!(parser.parse(&tokenize("i+i*i", &parser)).fully_matches());
    assert!(parser.parse(&tokenize("i+i", &parser)).fully_matches());
    assert!(parser.parse(&tokenize("i*i", &parser)).fully_matches());
    assert!(parser.parse(&tokenize("i", &parser)).fully_matches());
    assert!(parser.parse(&tokenize("(i+i)*i", &parser)).fully_matches());

    assert!(!parser.parse(&tokenize("i+", &parser)).fully_matches());
    assert!(!parser.parse(&tokenize("i++i", &parser)).fully_matches());
    assert!(!parser.parse(&tokenize("", &parser)).fully_matches());
    assert!(!parser.parse(&tokenize(")", &parser)).fully_matches());
}