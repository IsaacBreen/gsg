use std::path::Path;
use std::time::Instant;

use crate::_07_python::python_grammar::{python_file, python_literal, FSTRING_END, FSTRING_START, NAME, STRING, WS};
use crate::utils::{assert_fails, assert_fails_default, assert_fails_fast, assert_parses, assert_parses_default, assert_parses_fast, assert_parses_one_shot_with_result, assert_parses_tight, profile_parse};
use crate::{cache_context, choice, choice_greedy, eat, opt, seq, strong_ref, symbol, IntoDyn, UnambiguousParseError};

#[test]
fn test_trivial_x() {
    let combinator = python_file();
    // assert_parses_fast(&combinator, "x\n");
    assert_parses_one_shot_with_result(&combinator, "x\n", Err(UnambiguousParseError::Incomplete));
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
fn test_trivial_match() {
    let combinator = seq!(python_literal("match"), WS(), python_literal("x"), opt(WS()), python_literal(":"));
    assert_parses_default(&combinator, "match x:");
}

#[test]
fn test_name() {
    let combinator = cache_context(seq!(crate::_07_python::python_grammar::NAME(), crate::_07_python::python_grammar::NEWLINE()));
    assert_parses_fast(&combinator, "x\n");
    assert_parses_fast(&combinator, "xy\n");
    assert_parses_fast(&combinator, "match\n");
    assert_parses_fast(&combinator, "id\n");
    assert_parses_fast(&combinator, "_\n");
    assert_parses_fast(&combinator, "_abc123\n");

    assert_fails_fast(&combinator, "1\n");
    assert_fails_fast(&combinator, "1x\n");
    assert_fails_fast(&combinator, "if\n");
    assert_fails_fast(&combinator, "for\n");
}

#[test]
fn test_pass() {
    let combinator = python_file();
    // let combinator = cache_context(crate::python::python_grammar::simple_stmts());
    // let combinator = cache_context(seq!(&simple_stmt, &crate::python::python_grammar::NEWLINE));
    // let combinator = cache_context(seq!(python_literal("pass"), &crate::python::python_grammar::NEWLINE));
    // let combinator = cache_context(seq!(cached(python_literal("pass")), &crate::python::python_grammar::NEWLINE).compile());
    assert_parses_fast(&combinator, "pass\n");
}

#[test]
fn test_simple_assignment() {
    let combinator = python_file();
    // let combinator = cache_context(seq!(&assignment, &crate::python::python_grammar::NEWLINE)).compile();
    // let combinator = cache_context(seq!(&crate::python::python_grammar::NAME, python_literal("="), &crate::python::python_grammar::NAME, &crate::python::python_grammar::NEWLINE)).compile();
    // let combinator = cache_context(seq!(&crate::python::python_tokenizer::NAME, python_literal("="), &crate::python::python_tokenizer::NAME, &crate::python::python_grammar::NEWLINE)).compile();
    assert_parses_fast(&combinator, "x=x\n");

    // let combinator = cache_context(seq!(&crate::python::python_grammar::NAME, &crate::python::python_grammar::NEWLINE)).compile();
    // assert_parses_fast(&combinator, "x\n");
}

#[test]
fn test_simple() {
    println!("beginning test_simple");
    let combinator = python_file();

    assert_parses_default(&combinator, "1");
    assert_parses_default(&combinator, "11");
    assert_parses_default(&combinator, "111");
    assert_parses_default(&combinator, "1111");
    assert_parses_fast(&combinator, "1111\n");
    assert_parses_default(&combinator, "11111111");
    assert_parses_default(&combinator, "1111111111111111");

    assert_parses_fast(&combinator, "x=12\n");

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
    let path = Path::new("../_04_tests/test_input.py");
    let file = std::fs::read_to_string(path).unwrap();
    let combinator = python_file();
    assert_parses(&combinator, &file, "Actual Python file");
}

#[test]
fn test_test_input_fast() {
    let path = Path::new("../_04_tests/test_input.py");
    let file = std::fs::read_to_string(path).unwrap();
    let combinator = python_file();
    assert_parses_fast(&combinator, &file);
}

#[test]
fn test_test_input_one_shot() {
    let path = Path::new("../_04_tests/test_input.py");
    let file = std::fs::read_to_string(path).unwrap();
    let combinator = python_file();
    assert_parses_one_shot_with_result(&combinator, &file, Err(UnambiguousParseError::Incomplete));
}

#[test]
fn test_test_input2() {
    let path = Path::new("../_04_tests/test_input2.py");
    let file = std::fs::read_to_string(path).unwrap();
    let combinator = python_file();
    assert_parses_default(&combinator, &file);
}


#[test]
fn test_test_input_fast2() {
    let path = Path::new("../_04_tests/test_input2.py");
    let file = std::fs::read_to_string(path).unwrap();
    let combinator = python_file();
    assert_parses_fast(&combinator, &file);
}

#[test]
fn test_actual_python_file() {
    let combinator = python_file();

    let test_cases = [
        // ("Simple string", "x = 12\nx = 2\nx"),
        // ("dump_python_gram.py", include_str!("dump_python_gram.py")),
        ("grammar_analysis.py", include_str!("grammar_analysis.py")),
        // ("test_input.py", include_str!("test_inputs/test_input.py")),
        // ("test_input2.py", include_str!("test_inputs/test_input2.py")),
        // ("test_input3.py", include_str!("test_inputs/test_input3.py")),
    ];

    for (name, content) in test_cases.iter() {
        let start = Instant::now();
        assert_parses(&combinator, content, name);
        let duration = start.elapsed();
        println!("{:<25} parsed in {:?}", name, duration);
        assert_parses_tight(&combinator, content, name);
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
        // ("Simple string", "x = 12\nx = 2\nx"),
        // ("dump_python_gram.py", include_str!("dump_python_gram.py")),
        ("grammar_analysis.py", include_str!("grammar_analysis.py")),
        // ("test_input.py", include_str!("test_inputs/test_input.py")),
        // ("test_input2.py", include_str!("test_inputs/test_input2.py")),
        // ("test_input3.py", include_str!("test_inputs/test_input3.py")),
        // ("test_input_simplified.py", include_str!("test_inputs/test_input_simplified.py")),
    ];

    for (name, content) in test_cases.iter() {
        let start = Instant::now();
        assert_parses_fast(&combinator, content);
        let duration = start.elapsed();
        println!("{:<25} parsed in {:?}", name, duration);
    }
}

#[test]
fn test_actual_python_file_one_shot() {
    let combinator = python_file();

    let test_cases = [
        // ("Simple string", "x = 12\nx = 2\nx"),
        // ("Simple import", "from x import y\n\nimport z\nimport a.b.c\n"),
        // ("dump_python_gram.py", include_str!("dump_python_gram.py")),
        ("grammar_analysis.py", include_str!("grammar_analysis.py")),
        // ("test_input.py", include_str!("test_inputs/test_input.py")),
        // ("test_input2.py", include_str!("test_inputs/test_input2.py")),
        // ("test_input3.py", include_str!("test_inputs/test_input3.py")),
        // ("test_input_simplified.py", include_str!("test_inputs/test_input_simplified.py")),
    ];

    for (name, content) in test_cases.iter() {
        let start = Instant::now();
        assert_parses_one_shot_with_result(&combinator, content, Err(UnambiguousParseError::Incomplete));
        let duration = start.elapsed();
        println!("{:<25} parsed in {:?}", name, duration);
    }}


#[test]
fn profile_python_file() {
    let combinator = python_file();

    let profile_files = [
        // "src/tests/test_input.py",
        "src/python/dump_python_gram.py",
        // "src/python/grammar_analysis.py",
    ];

    for profile_file in profile_files.iter() {
        let path = Path::new(profile_file);
        let file = std::fs::read_to_string(path).unwrap();
        profile_parse(&combinator, &file);
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
    let path = Path::new("../_04_tests/test_input.py");
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

    assert_parses_default(&combinator, "raise x\n");
    assert_parses_fast(&combinator, "raise x\n");

    assert_parses_default(&combinator, "raise ValueError('x')\n");
    assert_parses_fast(&combinator, "raise ValueError('x')\n");

    assert_parses_default(&combinator, "raise ValueError(f'{x}')\n");
    assert_parses_fast(&combinator, "raise ValueError(f'{x}')\n");
}

#[test]
fn test_parse_fstring() {
    let combinator = python_file();

    let s = "f'x'\n";
    assert_parses_default(&combinator, s);
    assert_parses_fast(&combinator, s);

    let s = "f'{x}'\n";
    assert_parses_default(&combinator, s);
    assert_parses_fast(&combinator, s);
}

#[test]
fn test_parse_fstring_distilled() {
    let NAME = NAME();
    let FSTRING_START = FSTRING_START();
    let FSTRING_END = FSTRING_END();
    let fstring = seq!(FSTRING_START, FSTRING_END);
    let combinator = seq!(choice_greedy!(NAME, fstring), eat(';'));

    let s = "f'';";
    assert_parses_default(&combinator, s);
    assert_parses_fast(&combinator, s);
}

#[test]
fn test_another_fstring_issue() {
    let combinator = python_file();

    let s = "f'Unknown item type: {type(item)}'\n";
    assert_parses_default(&combinator, s);
    assert_parses_fast(&combinator, s);
}

#[test]
fn test_yet_another_fstring_issue() {
    let combinator = python_file();

    let s = "f'{f'{1}'}'\n";
    // assert_parses_default(&combinator, s);
    assert_parses_fast(&combinator, s);
}

#[test]
fn test_yet_another_fstring_issue_distilled() {
    let FSTRING_START = FSTRING_START();
    let FSTRING_END = FSTRING_END();
    let NUMBER = symbol(eat("1"));
    let mut fstring = strong_ref();
    fstring.set(seq!(FSTRING_START, eat('{'), choice!(&NUMBER, &fstring), eat('}'), FSTRING_END).into_dyn());
    let combinator = symbol(&fstring);

    let s = "f'{f'{1}'}'";
    assert_parses_default(&combinator, s);
    assert_parses_fast(&combinator, s);
}
