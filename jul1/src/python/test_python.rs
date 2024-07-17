use crate::{assert_parses, python_file};

#[test]
fn test_simple() {
    println!("beginning test_simple");
    let combinator = python_file();
    // assert_parses!(combinator, "x = 12");
    assert_parses!(combinator, "xxxxxxxxxxxxxxxx");
}