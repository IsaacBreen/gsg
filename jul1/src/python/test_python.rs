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