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
    assert_parses!(combinator, "token1 token2 token3", "Input with all possible valid tokens/constructs");
    // assert_fails!(combinator, "repeat repeat repeat", "Input with repeated elements");
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