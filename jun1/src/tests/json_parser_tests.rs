#[cfg(test)]
mod json_parser {
    use std::rc::Rc;
    use crate::{ActiveCombinator, choice, eat_string, eat_u8, eat_u8_range, eat_u8_range_complement, in_frame_stack, opt, repeat, seq, add_to_frame_stack, Combinator, forward_ref, U8Set};

    #[ignore]
    #[test]
    fn test_json_parser() {
        // Helper combinators for JSON parsing
        let whitespace = repeat(choice!(eat_u8(' '), eat_u8('\t'), eat_u8('\n'), eat_u8('\r')));
        let digit = eat_u8_range('0', '9');
        let digits = repeat(digit);
        let integer = seq!(opt(choice!(eat_u8('-'), eat_u8('+'))), digits.clone());
        let fraction = seq!(eat_u8('.'), digits.clone());
        let exponent = seq!(choice!(eat_u8('e'), eat_u8('E')), seq!(choice!(eat_u8('+'), eat_u8('-')), digits));
        let number = seq!(integer, opt(fraction), opt(exponent));

        let string_char = eat_u8_range_complement('"', '"');
        let string = seq!(eat_u8('"'), repeat(string_char), eat_u8('"'));

        let json_value: Rc<dyn Combinator> = forward_ref();

        let json_array = Rc::new(seq!(
            eat_u8('['),
            whitespace.clone(),
            opt(seq!(
                json_value.clone(),
                repeat(seq!(whitespace.clone(), eat_u8(','), whitespace.clone(), json_value.clone())),
                whitespace.clone(),
            )),
            eat_u8(']'),
        ));

        let key_value_pair = seq!(string.clone(), whitespace.clone(), eat_u8(':'), whitespace.clone(), json_value.clone());

        let json_object = Rc::new(seq!(
            eat_u8('{'),
            whitespace.clone(),
            opt(seq!(
                key_value_pair.clone(),
                whitespace.clone(),
                repeat(seq!(eat_u8(','), whitespace.clone(), key_value_pair.clone())),
                whitespace.clone(),
            )),
            eat_u8('}'),
        ));

        // json_value.set(
        //     choice!(
        //         string, number,
        //         eat_string("true"), eat_string("false"),
        //         eat_string("null"), json_array, json_object,
        //     )
        // );
        let json_value = Rc::new(choice!(
            string,
            number,
            eat_string("true"),
            eat_string("false"),
            eat_string("null"),
            json_array,
            json_object,
        ));

        // Test cases
        let json_parser = seq!(whitespace, json_value);
        // let json_parser = simplify_combinator(json_parser, &mut HashSet::new());

        let test_cases = [
            "null",
            "true",
            "false",
            "42",
            r#""Hello, world!""#,
            r#"{"key": "value"}"#,
            "[1, 2, 3]",
            r#"{"nested": {"array": [1, 2, 3], "object": {"a": true, "b": false}}}"#,
        ];

        let parse_json = |json_string: &str| -> bool {
            let mut it = ActiveCombinator::new(json_parser.clone());
            let mut result = it.send(None);
            for char in json_string.chars() {
                assert!(result.u8set().contains(char as u8), "Expected {} to be in {:?}", char, result.u8set());
                result = it.send(Some(char));
            }
            result.is_complete
        };

        for json_string in test_cases {
            assert!(parse_json(json_string), "Failed to parse JSON string: {}", json_string);
        }

        let invalid_json_strings = [
            r#"{"unclosed": "object""#,
            "[1, 2, 3",
            r#"{"invalid": "json",""#,
        ];

        for json_string in invalid_json_strings {
            assert!(!parse_json(json_string), "Incorrectly parsed invalid JSON string: {}", json_string);
        }

        let filenames: Vec<&str> = vec![
            // "GeneratedCSV_mini.json",
            // "GeneratedCSV_1.json",
            // "GeneratedCSV_2.json",
            // "GeneratedCSV_10.json",
            "GeneratedCSV_20.json",
            // "GeneratedCSV_100.json",
            // "GeneratedCSV_200.json",
        ];

        // Print execution times for each parser
        for filename in filenames {
            let json_string = std::fs::read_to_string(format!("static/{}", filename)).unwrap();
            let start = std::time::Instant::now();
            let result = parse_json(&json_string);
            let end = std::time::Instant::now();
            println!("{}: {} ms", filename, end.duration_since(start).as_millis());
            assert!(result, "Failed to parse JSON string: {}", json_string);
        }

        // Test with a string of 'a's
        println!("Testing with a string of 'a's of length 100 and length 200");
        for i in vec![1_000, 10_000] {
            let json_string = std::iter::repeat('a').take(i).collect::<String>();
            let json_string = format!(r#"{{"a": "{}"}}"#, json_string);
            let start = std::time::Instant::now();
            let result = parse_json(&json_string);
            let end = std::time::Instant::now();
            println!("{}: {} ms", i, end.duration_since(start).as_millis());
            assert!(result, "Failed to parse JSON string: {}", json_string);
        }
    }

    #[test]
    fn test_names() {
        let mut it = ActiveCombinator::new_with_names(
            in_frame_stack(
                choice!(
                        eat_string("ab"),
                        eat_string("c"),
                        eat_string("cd"),
                        eat_string("ce"),
                    ),
            ),
            vec!["cd".to_string()],
        );
        let result0 = it.send(None);
        assert!(result0.u8set() == &U8Set::from_chars("c"));
        let result1 = it.send(Some('a'));
        assert!(result1.u8set().is_empty());

        let mut it = ActiveCombinator::new_with_names(
            in_frame_stack(
                choice!(
                        eat_string("ab"),
                        eat_string("c"),
                        eat_string("cd"),
                        eat_string("ce"),
                    ),
            ),
            vec!["cd".to_string()],
        );
        let result1 = it.send(None);
        assert!(result1.u8set() == &U8Set::from_chars("c"));
        let result2 = it.send(Some('c'));
        assert!(result2.u8set() == &U8Set::from_chars("d"));
        let result3 = it.send(Some('d'));
        assert!(result3.is_complete);
    }

    #[test]
    fn test_names2() {
        let mut it = ActiveCombinator::new_with_names(
            choice!(
                seq!(add_to_frame_stack(eat_string("a")), in_frame_stack(eat_string("a")), eat_string("b")),
                seq!(eat_string("a"), in_frame_stack(eat_string("a")), eat_string("c")),
            ),
            vec![],
        );
        let result0 = it.send(None);
        assert!(result0.u8set() == &U8Set::from_chars("a"));
        let result1 = it.send(Some('a'));
        assert!(result1.u8set() == &U8Set::from_chars("a"));
        let result2 = it.send(Some('a'));
        assert!(result2.u8set() == &U8Set::from_chars("b"));
        let result3 = it.send(Some('b'));
        assert!(result3.is_complete);
    }
}
