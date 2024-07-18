use crate::{assert_fails, assert_parses, python_file};

#[test]
fn test_simple() {
    println!("beginning test_simple");
    let combinator = python_file();

    assert_parses!(combinator, "1");
    assert_parses!(combinator, "11");
    assert_parses!(combinator, "1111");
    assert_parses!(combinator, "11111111");
    assert_parses!(combinator, "1111111111111111");

    assert_parses!(combinator, "x=12\nx=2\nx");

    assert_parses!(combinator, "x = 12\nx = 2\nx");
    assert_parses!(combinator, "x = (\n12\n+\n2\n)\nx");

    assert_fails!(combinator, "x = 12\n+\n2\n");
}

#[test]
fn test_gpt4_suggestions_0() {
    println!("beginning test_simple");
    let combinator = python_file();

    // g0 = "a".repeat(100);
    // let binding1 = "a".repeat(10000);
    // let test_cases = vec![
    //     // Input Size
    //     ("Empty input", true, vec![""]),
    //     ("Very small input", true, vec!["a", "1"]),
    //     ("Medium-sized input", true, vec![binding0.as_str()]),
    //     ("Large input", true, vec![binding1.as_str()]),
    //
    //     // Input Content
    //     ("Valid, well-formed input", true, vec!["valid_input"]),
    //     ("Input with all possible valid tokens/constructs", true, vec!["token1 token2 token3"]),
    //     ("Input with repeated elements", false, vec!["repeat repeat repeat"]),
    //     ("Input with nested structures", true, vec!["nested(start(inner))"]),
    //
    //     // Edge Cases
    //     ("Input with only whitespace", false, vec![" "]),
    //     ("Input with mixed whitespace", false, vec![" \t\n "]),
    //     ("Input with Unicode characters", true, vec!["こんにちは"]),
    //     ("Input with escape sequences", true, vec!["escape\\nsequence"]),
    //     ("Input with comments", true, vec!["# this is a comment\nvalid_input"]),
    //
    //     // Error Handling
    //     ("Malformed input", false, vec!["malformed{"]),
    //     ("Incomplete input", false, vec!["incomplete"]),
    //     ("Input with syntax errors", false, vec!["syntax error"]),
    //     ("Input with semantic errors", false, vec!["semantic error"]),
    //     ("Input with invalid characters", false, vec!["invalid\x00char"]),
    //     ("Input with mismatched delimiters", false, vec!["mismatched{"]),
    //
    //     // Special Characters
    //     ("Input with special characters", true, vec!["!@#$%^&*()"]),
    //     ("Input with quotation marks", true, vec!["'single' \"double\""]),
    //     ("Input with backslashes", true, vec!["back\\slash"]),
    //
    //     // Numeric Values
    //     ("Integer values", true, vec!["123", "-123", "0"]),
    //     ("Floating-point values", true, vec!["123.456", "-123.456", "0.0"]),
    //     ("Scientific notation", true, vec!["1.23e10", "-1.23e-10"]),
    //     ("Very large numbers", true, vec!["12345678901234567890"]),
    //     ("Very small numbers", true, vec!["0.000000000123456789"]),
    //
    //     // String Values
    //     ("Empty strings", true, vec!["\"\""]),
    //     ("Strings with spaces", true, vec!["\"a string with spaces\""]),
    //     ("Strings with escape sequences", true, vec!["\"escape\\tsequence\""]),
    //     ("Multi-line strings", true, vec!["\"\"\"multi\nline\nstring\"\"\""]),
    //
    //     // Boolean Values
    //     ("True/False values", true, vec!["true", "false"]),
    //     ("Case sensitivity testing", false, vec!["True", "False"]),
    //
    //     // Null/None Values
    //     ("Null or None values", true, vec!["null"]),
    //
    //     // Arrays/Lists
    //     ("Empty arrays", true, vec!["[]"]),
    //     ("Arrays with single element", true, vec!["[1]"]),
    //     ("Arrays with multiple elements", true, vec!["[1, 2, 3]"]),
    //     ("Nested arrays", true, vec!["[[1, 2], [3, 4]]"]),
    //
    //     // Objects/Dictionaries
    //     ("Empty objects", true, vec!["{}"]),
    //     ("Objects with single key-value pair", true, vec!["{\"key\": \"value\"}"]),
    //     ("Objects with multiple key-value pairs", true, vec!["{\"key1\": \"value1\", \"key2\": \"value2\"}"]),
    //     ("Nested objects", true, vec!["{\"outer\": {\"inner\": \"value\"}}"]),
    //
    //     // Encoding
    //     ("Different character encodings", true, vec!["utf8_string"]),
    //     ("Byte Order Mark (BOM) handling", true, vec!["\u{feff}bom_handling"]),
    //
    //     // Localization
    //     ("Input in different languages", true, vec!["bonjour", "hola", "你好"]),
    //     ("Input with different locale-specific formatting", true, vec!["1,000.00", "1.000,00"]),
    // ];
    //
    // for (description, should_parse, cases) in test_cases {
    //     println!("description: {}", description);
    //     println!("should_parse: {}", should_parse);
    //     for case in cases {
    //         println!("case: {}", case);
    //         if should_parse {
    //             assert_parses!(combinator, case);
    //         } else {
    //             assert_fails!(combinator, case);
    //         }
    //     }
    // }

    // Input Size
    assert_parses!(combinator, "", "Empty input");
    assert_parses!(combinator, "a", "Very small input");
    assert_parses!(combinator, "a".repeat(100), "Medium-sized input");
    assert_parses!(combinator, "a".repeat(10000), "Large input");

    // Input Content
    assert_parses!(combinator, "valid_input", "Valid, well-formed input");
    assert_parses!(combinator, "token1 token2 token3", "Input with all possible valid tokens/constructs");
    assert_fails!(combinator, "repeat repeat repeat", "Input with repeated elements");
    assert_parses!(combinator, "nested(start(inner))", "Input with nested structures");

    // Edge Cases
    assert_fails!(combinator, " ", "Input with only whitespace");
    assert_fails!(combinator, " \t\n ", "Input with mixed whitespace");
    assert_parses!(combinator, "こんにちは", "Input with Unicode characters");
    assert_parses!(combinator, "escape\\nsequence", "Input with escape sequences");
    assert_parses!(combinator, "# this is a comment\nvalid_input", "Input with comments");

    // Error Handling
    assert_fails!(combinator, "malformed{", "Malformed input");
    assert_fails!(combinator, "incomplete", "Incomplete input");
    assert_fails!(combinator, "syntax error", "Input with syntax errors");
    assert_fails!(combinator, "semantic error", "Input with semantic errors");
    assert_fails!(combinator, "invalid\x00char", "Input with invalid characters");
    assert_fails!(combinator, "mismatched{", "Input with mismatched delimiters");

    // Special Characters
    assert_parses!(combinator, "!@#$%^&*()", "Input with special characters");
    assert_parses!(combinator, "'single' \"double\"", "Input with quotation marks");
    assert_parses!(combinator, "back\\slash", "Input with backslashes");

    // Numeric Values
    assert_parses!(combinator, "123", "Integer values");
    assert_parses!(combinator, "-123", "Integer values");
    assert_parses!(combinator, "0", "Integer values");
    assert_parses!(combinator, "123.456", "Floating-point values");
    assert_parses!(combinator, "-123.456", "Floating-point values");
    assert_parses!(combinator, "0.0", "Floating-point values");
    assert_parses!(combinator, "1.23e10", "Scientific notation");
    assert_parses!(combinator, "-1.23e-10", "Scientific notation");
    assert_parses!(combinator, "12345678901234567890", "Very large numbers");
    assert_parses!(combinator, "0.000000000123456789", "Very small numbers");

    // String Values
    assert_parses!(combinator, "\"\"", "Empty strings");
    assert_parses!(combinator, "\"a string with spaces\"", "Strings with spaces");
    assert_parses!(combinator, "\"escape\\tsequence\"", "Strings with escape sequences");
    assert_parses!(combinator, "\"\"\"multi\nline\nstring\"\"\"", "Multi-line strings");

    // Boolean Values
    assert_parses!(combinator, "true", "True/False values");
    assert_fails!(combinator, "True", "Case sensitivity testing");

    // Null/None Values
    assert_parses!(combinator, "null");

    // Arrays/Lists
    assert_parses!(combinator, "[]", "Empty arrays");
    assert_parses!(combinator, "[1]", "Arrays with single element");
    assert_parses!(combinator, "[1, 2, 3]", "Arrays with multiple elements");
    assert_parses!(combinator, "[[1, 2], [3, 4]]", "Nested arrays");

    // Objects/Dictionaries
    assert_parses!(combinator, "{}", "Empty objects");
    assert_parses!(combinator, "{\"key\": \"value\"}", "Objects with single key-value pair");
    assert_parses!(combinator, "{\"key1\": \"value1\", \"key2\": \"value2\"}", "Objects with multiple key-value pairs");
    assert_parses!(combinator, "{\"outer\": {\"inner\": \"value\"}}", "Nested objects");

    // Encoding
    assert_parses!(combinator, "utf8_string");
    assert_parses!(combinator, "\u{feff}bom_handling");

    // Localization
    assert_parses!(combinator, "bonjour", "Input in different languages");
    assert_parses!(combinator, "1,000.00", "Input with different locale-specific formatting");
}