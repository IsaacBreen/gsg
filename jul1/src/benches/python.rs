use std::path::Path;
use std::time::Instant;
use jun3::python_file;
use jun3::utils::{assert_parses, assert_parses_fast};

fn main() {
    let path = Path::new("src/tests/test_input.py");
    let file = std::fs::read_to_string(path).unwrap();
    let combinator = python_file();
    assert_parses_fast(&combinator, &file);
}