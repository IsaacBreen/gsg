
#[cfg(test)]
mod glalr_tests {
    use crate::glr::grammar::{nt, prod, t, Terminal};
    use crate::glr::parser::GLRParser;
    use crate::glr::table::{generate_glr_parser, TerminalID};
    use super::*;

    #[test]
    fn test_simple_parse_table() {
        let productions = vec![
            // S -> a
            prod("S", vec![nt("A")]),
            // A -> A a | b
            prod("A", vec![nt("A"), t("a")]),
            prod("A", vec![t("b")]),
        ];

        let (stage_7_table, terminal_map, non_terminal_map, item_set_map, start_state_id) =
            generate_glr_parser(&productions);

        let parser = GLRParser {
            stage_7_table,
            start_state_id,
            terminal_map,
            non_terminal_map,
            item_set_map,
        };

        println!("{}", parser);

        let tokenize = |input: &str, parser: &GLRParser| -> Vec<TerminalID> {
            let mut result = Vec::new();
            for c in input.chars() {
                let terminal = Terminal(c.to_string());
                if let Some(id) = parser.terminal_map.get_by_left(&terminal) {
                    result.push(*id);
                } else {
                    panic!("Unknown token: {}", c);
                }
            }
            result
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

        let (stage_7_table, terminal_map, non_terminal_map, item_set_map, start_state_id) =
            generate_glr_parser(&productions);

        let parser = GLRParser {
            stage_7_table,
            start_state_id,
            terminal_map,
            non_terminal_map,
            item_set_map,
        };

        println!("{}", parser);

        let tokenize = |input: &str, parser: &GLRParser| -> Vec<TerminalID> {
            let mut result = Vec::new();
            for c in input.chars() {
                let terminal = Terminal(c.to_string());
                if let Some(id) = parser.terminal_map.get_by_left(&terminal) {
                    result.push(*id);
                } else {
                    panic!("Unknown token: {}", c);
                }
            }
            result
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
}

fn main() {
    println!("GLR Parser Example");
    // Add your main function code here if needed.  The tests above provide example usage.
}