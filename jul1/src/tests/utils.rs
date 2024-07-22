use kdam::tqdm;
use crate::{CombinatorTrait, RightData, ParseResults, ParserTrait};

pub fn assert_parses<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, desc: &str) {
    let input = input.to_string();
    println!("beginning assert_parses {}", desc);
    let (mut parser, _) = T::parser(&combinator, RightData::default());
    println!("constructed parser");

    let mut result = Ok(());

    for (line_number, line) in tqdm!(input.lines().enumerate(), animation = "fillup") {
        for (char_number, byte) in tqdm!(line.bytes().enumerate(), animation = "fillup") {
            let ParseResults {
                right_data_vec: right_data,
                up_data_vec: up_data,
                cut,
            } = parser.step(byte);

            println!("Stats:");
            println!("{}", parser.stats());
            if cut {
                println!("cut!");
                println!()
            }
            if right_data.is_empty() && up_data.is_empty() {
                result = Err(format!(
                    "Parser failed at byte: {} on line: {} at char: {}",
                    byte as char,
                    line_number + 1,
                    char_number + 1
                ));
                break;
            }
        }
        if result.is_err() {
            break;
        }
    }

    assert!(result.is_ok(), "{}", desc);
}

pub fn assert_parses_default<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    assert_parses(combinator, input, "Parser failed unexpectedly");
}

pub fn assert_fails<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, desc: &str) {
    let input = input.to_string();
    println!("beginning assert_fails {}", desc);
    let (mut parser, _) = T::parser(&combinator, RightData::default());
    println!("constructed parser");

    let mut result = Ok(());

    for (line_number, line) in tqdm!(input.lines().enumerate(), animation = "fillup") {
        for (char_number, byte) in tqdm!(line.bytes().enumerate(), animation = "fillup") {
            let ParseResults {
                right_data_vec: right_data,
                up_data_vec: up_data,
                cut,
            } = parser.step(byte);

            println!("Stats:");
            println!("{}", parser.stats());
            if cut {
                println!("cut!");
                println!()
            }
            if right_data.is_empty() && up_data.is_empty() {
                result = Err(format!(
                    "Parser failed at byte: {} on line: {} at char: {}",
                    byte as char,
                    line_number + 1,
                    char_number + 1
                ));
                break;
            }
        }
        if result.is_err() {
            break;
        }
    }

    match result {
        Ok(_) => assert!(false, "{}", desc),
        Err(_) => (),
    }
}

pub fn assert_fails_default<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    assert_fails(combinator, input, "Parser succeeded unexpectedly");
}