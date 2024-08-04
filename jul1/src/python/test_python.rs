use std::path::Path;
use std::time::Instant;

use crate::{eat_string, NAME, non_breaking_space, opt, python_file, python_literal, seq, STRING, whitespace, WS};
use crate::utils::{assert_fails, assert_fails_default, assert_fails_fast, assert_parses, assert_parses_default, assert_parses_fast, assert_parses_fast_with_tolerance};

#[test]
fn test_trivial_x() {
    let combinator = python_file();
    assert_parses_fast(&combinator, "x\n");
}

#[test]
fn test_simple_import() {
    let combinator = python_file();
    assert_parses_fast(&combinator, "import x\n");
}

#[test]
fn test_trivial_ws() {
    let combinator = seq!(NAME(), WS(), python_literal("="));
    assert_parses_default(&combinator, "x =");
}

#[test]
fn test_trivial_adjacent_literals() {
    let combinator = non_breaking_space();
    assert_parses_default(&combinator, " ");
}

#[test]
fn test_trivial_match() {
    let combinator = seq!(python_literal("match"), WS(), python_literal("x"), opt(WS()), python_literal(":"));
    assert_parses_default(&combinator, "match x:");
}

#[test]
fn test_simple() {
    println!("beginning test_simple");
    let combinator = python_file();

    assert_parses_default(&combinator, "1");
    assert_parses_default(&combinator, "11");
    assert_parses_default(&combinator, "1111");
    assert_parses_default(&combinator, "11111111");
    assert_parses_default(&combinator, "1111111111111111");

    assert_parses_default(&combinator, "x=12\nx=2\nx");

    assert_parses_default(&combinator, "x = 12\nx = 2\nx");
    assert_parses_default(&combinator, "x = (\n12\n+\n2\n)\nx");

    assert_fails_default(&combinator, "x = 12\n+\n2\n");
}

#[test]
fn test_gpt4_suggestions_0() {
    println!("beginning test_simple");
    let combinator = python_file();

    // Input Size
    assert_parses(&combinator, "", "Empty input");
    assert_parses(&combinator, "a", "Very small input");
    assert_parses(&combinator, "a".repeat(100), "Medium-sized input");
    // assert_parses(&combinator, "a".repeat(10000), "Large input");

    // Input Content
    assert_parses(&combinator, "valid_input", "Valid, well-formed input");
    assert_fails(&combinator, "repeat repeat repeat", "Input with repeated elements");
    assert_parses(&combinator, "nested(start(inner))", "Input with nested structures");

    // Edge Cases
    assert_parses(&combinator, " ", "Input with only whitespace. A valid completion is ' \n...'");
    assert_parses(&combinator, " \t\n ", "Input with mixed whitespace");
    assert_parses(&combinator, "def こんにちは(x):", "Input with Unicode characters");
    assert_fails(&combinator, "1 + \n2", "Input with unescaped newline");
    assert_parses(&combinator, "1 +\\\nsequence", "Arithmetic expression with escaped newline");
    assert_parses(&combinator, "(1 + \n2) * 3", "Arithmetic expression with unescaped newline inside parentheses");
    assert_parses(&combinator, "# this is a comment\nvalid_input", "Input with comments");

    // Error Handling
    // assert_fails(&combinator, "x {", "Malformed input");
    assert_parses(&combinator, "def f():", "Incomplete input");
    assert_fails(&combinator, "syntax error", "Input with syntax errors");
    assert_fails(&combinator, "semantic error", "Input with semantic errors");
    assert_fails(&combinator, "invalid\x00char", "Input with invalid characters");
    // assert_fails(&combinator, "x {", "Input with mismatched delimiters");

    // Special Characters
    assert_fails(&combinator, "!@#$%^&*()", "Input with special characters");
    assert_parses(&combinator, "'single'", "Input with single-quoted string");
    assert_parses(&combinator, "\"double\"", "Input with double-quoted string");
    assert_fails(&combinator, "back\\slash", "Input with backslashes");

    // Numeric Values
    assert_parses(&combinator, "123", "Integer values");
    assert_parses(&combinator, "-123", "Integer values");
    assert_parses(&combinator, "0", "Integer values");
    assert_parses(&combinator, "123.456", "Floating-point values");
    assert_parses(&combinator, "-123.456", "Floating-point values");
    assert_parses(&combinator, "0.0", "Floating-point values");
    assert_parses(&combinator, "1.23e10", "Scientific notation");
    assert_parses(&combinator, "-1.23e-10", "Scientific notation");
    assert_parses(&combinator, "12345678901234567890", "Very large numbers");
    assert_parses(&combinator, "0.000000000123456789", "Very small numbers");

    // String Values
    assert_parses(&combinator, "\"\"", "Empty strings");
    assert_parses(&combinator, "\"a string with spaces\"", "Strings with spaces");
    assert_parses(&combinator, "\"escape\\tsequence\"", "Strings with escape sequences");
    assert_parses(&combinator, "\"\"\"multi\nline\nstring\"\"\"", "Multi-line strings");

    // Boolean Values
    assert_parses(&combinator, "true", "True/False values");
    assert_parses(&combinator, "True", "Case sensitivity testing");

    // Null/None Values
    assert_parses_default(&combinator, "null");

    // Arrays/Lists
    assert_parses(&combinator, "[]", "Empty arrays");
    assert_parses(&combinator, "[1]", "Arrays with single element");
    assert_parses(&combinator, "[1, 2, 3]", "Arrays with multiple elements");
    assert_parses(&combinator, "[[1, 2], [3, 4]]", "Nested arrays");

    // Objects/Dictionaries
    assert_parses(&combinator, "{}", "Empty objects");
    assert_parses(&combinator, "{\"key\": \"value\"}", "Objects with single key-value pair");
    assert_parses(&combinator, "{\"key1\": \"value1\", \"key2\": \"value2\"}", "Objects with multiple key-value pairs");
    assert_parses(&combinator, "{\"outer\": {\"inner\": \"value\"}}", "Nested objects");

    // Encoding
    assert_parses_default(&combinator, "utf8_string");

    // Localization
    assert_parses(&combinator, "bonjour", "Input in different languages");  // lol
    assert_parses(&combinator, "1,000.00", "Input with different locale-specific formatting");
}

// #[ignore]
#[test]
fn test_test_input() {
    let path = Path::new("src/tests/test_input.py");
    let file = std::fs::read_to_string(path).unwrap();
    let combinator = python_file();
    assert_parses(&combinator, &file, "Actual Python file");
}

#[test]
fn test_test_input_fast() {
    let path = Path::new("src/tests/test_input.py");
    let file = std::fs::read_to_string(path).unwrap();
    let combinator = python_file();
    assert_parses_fast(&combinator, &file);
}

#[test]
fn test_actual_python_file() {
    let combinator = python_file();

    let test_cases = [
        ("Simple string", "x = 12\nx = 2\nx"),
        ("dump_python_gram.py", include_str!("../python/dump_python_gram.py")),
        ("remove_left_recursion.py", include_str!("../python/remove_left_recursion.py")),
        ("test_input.py", include_str!("../tests/test_input.py")),
    ];

    for (name, content) in test_cases.iter() {
        let start = Instant::now();
        assert_parses(&combinator, content, name);
        let duration = start.elapsed();
        println!("{:<25} parsed in {:?}", name, duration);
    }
}

#[test]
fn test_fails_fast() {
    let combinator = python_file();

    let test_casts = [
        ("double ampersand", "&&"),
        ("double identifier", "x x"),
    ];

    for (name, content) in test_casts.iter() {
        let start = Instant::now();
        assert_fails_fast(&combinator, content);
        let duration = start.elapsed();
        println!("{:<25} parsed in {:?}", name, duration);
    }
}

#[test]
fn test_actual_python_file_fast() {
    let combinator = python_file();

    let test_cases = [
        ("Simple string", "x = 12\nx = 2\nx"),
        ("dump_python_gram.py", include_str!("../python/dump_python_gram.py")),
        ("remove_left_recursion.py", include_str!("../python/remove_left_recursion.py")),
        ("test_input.py", include_str!("../tests/test_input.py")),
    ];

    for (name, content) in test_cases.iter() {
        let start = Instant::now();
        assert_parses_fast_with_tolerance(&combinator, content, 20);
        let duration = start.elapsed();
        println!("{:<25} parsed in {:?}", name, duration);
    }
}

#[test]
fn test_lots_of_lines() {
    let combinator = python_file();

    let s = "a\n".repeat(1000);
    assert_parses_default(&combinator, &s);
    assert_parses_fast(&combinator, &s);
}

#[test]
fn test_simple_2() {
    let combinator = python_file();
    // assert_parses_default(&combinator, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    assert_parses_default(&combinator, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n");
    assert_parses_fast(&combinator, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n");
}

#[test]
fn test_indents() {
    let combinator = python_file();
    assert_fails(&combinator, "def f():\npass\n", "Indentation error");
    assert_parses(&combinator, "def f():\n pass\n", "One space indentation");
    assert_parses(&combinator, "def f():\n  pass\n", "Two space indentation");
    assert_fails(&combinator, "def f():\n if True:\n pass\n", "Indentation error");
    assert_parses(&combinator, "def f():\n if True:\n  pass\n", "Two space indentation");
}

#[test]
fn test_explosion_please() {
    let combinator = python_file();
    let path = Path::new("src/tests/test_input.py");
    let s = std::fs::read_to_string(path).unwrap();
    assert_parses(&combinator, &s, "Test input");
}

#[test]
fn test_string_problem() {
    let combinator = python_file();

    // let s = "def f():\n  return\n"#;
    // assert_parses(&combinator, s, "String");

    // let s = "return \"choice!(\\n    \" + \",\\n    \".join(alt_to_rust(alt) for alt in rhs.alts) + \"\\n)\n";
    let s = "return \"choice!(\\n    \" + \",\\n    \".join(alt_to_rust(alt) for alt in rhs.alts) + \"\\n)\n";
    assert_parses(&combinator, s, "String");

    // let s = "return\n";
    // assert_parses(&combinator, s, "String");
}

#[test]
fn test_string() {
    let combinator = STRING();

    let s = "f'{x}'\n";
    // assert_parses_default(&combinator, s);
    assert_parses_fast(&combinator, s);

    // todo: Escpae sequences in both f-strings and regular strings
}

#[test]
fn test_debug_fail_case() {
    let combinator = python_file();
    assert_parses_default(&combinator, "f(xx=1)\n");
    assert_parses_fast(&combinator, "f(xx=1)\n");
}

#[test]
fn test_debug_import() {
    let combinator = python_file();
    assert_parses_default(&combinator, "import x\n");
    assert_parses_fast(&combinator, "import x\n");
}

#[test]
fn test_debug_raise() {
    let combinator = python_file();

    // assert_parses_default(&combinator, "raise x\n");
    // assert_parses_fast(&combinator, "raise x\n");

    // assert_parses_default(&combinator, "raise ValueError('x')\n");
    assert_parses_fast(&combinator, "raise ValueError('x')\n");

    // assert_parses_default(&combinator, "raise ValueError(f'{x}')\n");
    // assert_parses_fast(&combinator, "raise ValueError(f'{x}')\n");
}

#[test]
fn test_parse_fstring() {
    let combinator = python_file();

    let s = "f''\n";
    // assert_parses_default(&combinator, s);
    assert_parses_fast(&combinator, s);

    // let s = "f'{x}'\n";
    // assert_parses_default(&combinator, s);
    // assert_parses_fast(&combinator, s);
}