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

    let binding0 = "a".repeat(100);
    let binding1 = "a".repeat(10000);
    let binding2 = "max_elements ".repeat(100);
    let test_cases = vec![
        // Input Size
        ("Empty input", true, vec![""]),
        ("Very small input", true, vec!["a", "1"]),
        ("Medium-sized input", true, vec![binding0.as_str()]),
        ("Large input", true, vec![binding1.as_str()]),

        // Input Content
        ("Valid, well-formed input", true, vec!["valid_input"]),
        ("Input with all possible valid tokens/constructs", true, vec!["token1 token2 token3"]),
        ("Input with minimum required elements", true, vec!["min_elements"]),
        ("Input with maximum allowed elements", true, vec![binding2.as_str()]),
        ("Input with repeated elements", true, vec!["repeat repeat repeat"]),
        ("Input with nested structures", true, vec!["nested(start(inner))"]),

        // Edge Cases
        ("Input with only whitespace", false, vec![" "]),
        ("Input with mixed whitespace", false, vec![" \t\n "]),
        ("Input with Unicode characters", true, vec!["こんにちは"]),
        ("Input with escape sequences", true, vec!["escape\\nsequence"]),
        ("Input with comments", true, vec!["# this is a comment\nvalid_input"]),

        // Error Handling
        ("Malformed input", false, vec!["malformed{"]),
        ("Incomplete input", false, vec!["incomplete"]),
        ("Input with syntax errors", false, vec!["syntax error"]),
        ("Input with semantic errors", false, vec!["semantic error"]),
        ("Input with invalid characters", false, vec!["invalid\x00char"]),
        ("Input with mismatched delimiters", false, vec!["mismatched{"]),

        // Special Characters
        ("Input with special characters", true, vec!["!@#$%^&*()"]),
        ("Input with quotation marks", true, vec!["'single' \"double\""]),
        ("Input with backslashes", true, vec!["back\\slash"]),

        // Numeric Values
        ("Integer values", true, vec!["123", "-123", "0"]),
        ("Floating-point values", true, vec!["123.456", "-123.456", "0.0"]),
        ("Scientific notation", true, vec!["1.23e10", "-1.23e-10"]),
        ("Very large numbers", true, vec!["12345678901234567890"]),
        ("Very small numbers", true, vec!["0.000000000123456789"]),

        // String Values
        ("Empty strings", true, vec!["\"\""]),
        ("Strings with spaces", true, vec!["\"a string with spaces\""]),
        ("Strings with escape sequences", true, vec!["\"escape\\tsequence\""]),
        ("Multi-line strings", true, vec!["\"\"\"multi\nline\nstring\"\"\""]),

        // Boolean Values
        ("True/False values", true, vec!["true", "false"]),
        ("Case sensitivity testing", false, vec!["True", "False"]),

        // Null/None Values
        ("Null or None values", true, vec!["null"]),

        // Arrays/Lists
        ("Empty arrays", true, vec!["[]"]),
        ("Arrays with single element", true, vec!["[1]"]),
        ("Arrays with multiple elements", true, vec!["[1, 2, 3]"]),
        ("Nested arrays", true, vec!["[[1, 2], [3, 4]]"]),

        // Objects/Dictionaries
        ("Empty objects", true, vec!["{}"]),
        ("Objects with single key-value pair", true, vec!["{\"key\": \"value\"}"]),
        ("Objects with multiple key-value pairs", true, vec!["{\"key1\": \"value1\", \"key2\": \"value2\"}"]),
        ("Nested objects", true, vec!["{\"outer\": {\"inner\": \"value\"}}"]),

        // Encoding
        ("Different character encodings", true, vec!["utf8_string"]),
        ("Byte Order Mark (BOM) handling", true, vec!["\u{feff}bom_handling"]),

        // Localization
        ("Input in different languages", true, vec!["bonjour", "hola", "你好"]),
        ("Input with different locale-specific formatting", true, vec!["1,000.00", "1.000,00"]),
    ];

    for (description, should_parse, cases) in test_cases {
        println!("description: {}", description);
        println!("should_parse: {}", should_parse);
        for case in cases {
            println!("case: {}", case);
            if should_parse {
                assert_parses!(combinator, case);
            } else {
                assert_fails!(combinator, case);
            }
        }
    }
}