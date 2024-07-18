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

    // Input Size
    assert_parses!(combinator, "", "Empty input");
    assert_parses!(combinator, "a", "Very small input");
    assert_parses!(combinator, "a".repeat(100), "Medium-sized input");
    // assert_parses!(combinator, "a".repeat(10000), "Large input");

    // Input Content
    assert_parses!(combinator, "valid_input", "Valid, well-formed input");
    assert_fails!(combinator, "repeat repeat repeat", "Input with repeated elements");
    assert_parses!(combinator, "nested(start(inner))", "Input with nested structures");

    // Edge Cases
    assert_parses!(combinator, " ", "Input with only whitespace. A valid completion is ' \n...'");
    assert_parses!(combinator, " \t\n ", "Input with mixed whitespace");
    assert_parses!(combinator, "def こんにちは(x):", "Input with Unicode characters");
    assert_fails!(combinator, "1 + \n2", "Input with unescaped newline");
    assert_parses!(combinator, "1 +\\\nsequence", "Arithmetic expression with escaped newline");
    assert_parses!(combinator, "(1 + \n2) * 3", "Arithmetic expression with unescaped newline inside parentheses");
    assert_parses!(combinator, "# this is a comment\nvalid_input", "Input with comments");

    // Error Handling
    // assert_fails!(combinator, "malformed{", "Malformed input");
    assert_parses!(combinator, "def f():", "Incomplete input");
    assert_fails!(combinator, "syntax error", "Input with syntax errors");
    assert_fails!(combinator, "semantic error", "Input with semantic errors");
    assert_fails!(combinator, "invalid\x00char", "Input with invalid characters");
    // assert_fails!(combinator, "mismatched{", "Input with mismatched delimiters");

    // Special Characters
    assert_fails!(combinator, "!@#$%^&*()", "Input with special characters");
    assert_parses!(combinator, "'single'", "Input with single-quoted string");
    assert_parses!(combinator, "\"double\"", "Input with double-quoted string");
    assert_fails!(combinator, "back\\slash", "Input with backslashes");

    // Numeric Values
    assert_parses!(combinator, "123", "Integer values");
    assert_parses!(combinator, "-123", "Integer values");
    assert_parses!(combinator, "0", "Integer values");
    assert_parses!(combinator, "123.456", "Floating-point values");
    assert_parses!(combinator, "-123.456", "Floating-point values");
    assert_parses!(combinator, "0.0", "Floating-point values");
    // assert_parses!(combinator, "1.23e10", "Scientific notation");
    // assert_parses!(combinator, "-1.23e-10", "Scientific notation");
    assert_parses!(combinator, "12345678901234567890", "Very large numbers");
    assert_parses!(combinator, "0.000000000123456789", "Very small numbers");

    // String Values
    assert_parses!(combinator, "\"\"", "Empty strings");
    assert_parses!(combinator, "\"a string with spaces\"", "Strings with spaces");
    assert_parses!(combinator, "\"escape\\tsequence\"", "Strings with escape sequences");
    assert_parses!(combinator, "\"\"\"multi\nline\nstring\"\"\"", "Multi-line strings");

    // Boolean Values
    assert_parses!(combinator, "true", "True/False values");
    assert_parses!(combinator, "True", "Case sensitivity testing");

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

    // Localization
    assert_parses!(combinator, "bonjour", "Input in different languages");  // lol
    assert_parses!(combinator, "1,000.00", "Input with different locale-specific formatting");
}