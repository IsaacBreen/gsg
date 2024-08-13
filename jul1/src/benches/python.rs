use std::time::Instant;
use jun3::python_file;
use jun3::utils::assert_parses;

fn main() {
    let combinator = python_file();

    let test_cases = [
        // ("Simple string", "x = 12\nx = 2\nx"),
        ("dump_python_gram.py", include_str!("../python/dump_python_gram.py")),
        // ("remove_left_recursion.py", include_str!("../python/remove_left_recursion.py")),
        // ("test_input.py", include_str!("../tests/test_input.py")),
    ];

    for (name, content) in test_cases.iter() {
        let start = Instant::now();
        assert_parses(&combinator, content, name);
        let duration = start.elapsed();
        println!("{:<25} parsed in {:?}", name, duration);
    }
}