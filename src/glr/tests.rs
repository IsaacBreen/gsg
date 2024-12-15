use crate::glr::grammar::{nt, prod, t, Terminal};
use crate::glr::parser::GLRParser;
use crate::glr::table::{generate_glr_parser, TerminalID};

fn create_simple_parser() -> GLRParser {
    let productions = vec![
        prod("S", vec![nt("A")]),
        prod("A", vec![nt("A"), t("a")]),
        prod("A", vec![t("b")]),
    ];

    generate_glr_parser(&productions, 0)
}

fn create_expression_parser() -> GLRParser {
    let productions = vec![
        prod("S", vec![nt("E")]),
        prod("E", vec![nt("E"), t("+"), nt("T")]),
        prod("E", vec![nt("T")]),
        prod("T", vec![nt("T"), t("*"), nt("F")]),
        prod("T", vec![nt("F")]),
        prod("F", vec![t("("), nt("E"), t(")")]),
        prod("F", vec![t("i")]),
    ];

    generate_glr_parser(&productions, 0)
}

fn tokenize(parser: &GLRParser, input: &str) -> Vec<TerminalID> {
    input.chars()
        .filter_map(|c| parser.terminal_map.get_by_left(&Terminal(c.to_string())).copied())
        .collect()
}

#[test]
fn test_simple_parse_table() {
    let parser = create_simple_parser();
    
    let valid_inputs = ["b", "ba", "baa"];
    let invalid_inputs = ["a", "bb"];

    for input in valid_inputs {
        assert!(parser.parse(&tokenize(&parser, input)).fully_matches(), 
                "Failed for valid input: {}", input);
    }

    for input in invalid_inputs {
        assert!(!parser.parse(&tokenize(&parser, input)).fully_matches(), 
                "Failed for invalid input: {}", input);
    }
}

#[test]
fn test_parse_simple_expression() {
    let parser = create_expression_parser();
    
    let valid_inputs = ["i", "i+i*i", "i+i", "i*i", "(i+i)*i"];
    let invalid_inputs = ["i+", "i++i", "", ")"];

    for input in valid_inputs {
        assert!(parser.parse(&tokenize(&parser, input)).fully_matches(), 
                "Failed for valid input: {}", input);
    }

    for input in invalid_inputs {
        assert!(!parser.parse(&tokenize(&parser, input)).fully_matches(), 
                "Failed for invalid input: {}", input);
    }
}
