use std::panic::catch_unwind;
use kdam::tqdm;
use crate::{CombinatorTrait, RightData, ParseResults, ParserTrait};

pub fn assert_parses<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, desc: &str) {
    let mut input = input.to_string();
    println!("beginning assert_parses {}", desc);
    let (mut parser, ParseResults { up_data_vec: mut up_data, .. }) = T::parser(&combinator, RightData::default());
    println!("constructed parser");

    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();
    'outer: for (line_number, line) in tqdm!(lines.iter().enumerate(), animation = "fillup", position = 0) {
        // Add newline back in
        let mut line = format!("{}", line);
        if line_number != num_lines - 1 {
            line = format!("{}\n", line);
        }
        let bytes = line.bytes().collect::<Vec<_>>();
        for (char_number, byte) in tqdm!(bytes.iter().cloned().enumerate(), animation = "fillup", position = 1) {
            println!("byte: {:?}\n\n\n\n", byte as char);
            let byte_is_in_some_up_data = up_data.iter().any(|up_data| up_data.u8set.contains(byte));
            assert!(byte_is_in_some_up_data, "byte {:?} is not in any up_data: {:?}", byte as char, up_data);

            if line_number == lines.len() - 1 && char_number == bytes.len() - 1 {
                break 'outer;
            }

            let ParseResults {
                right_data_vec: right_data,
                up_data_vec: new_up_data,
                cut,
                done,
            } = parser.step(byte);

            up_data = new_up_data;

            println!();
            println!("line:char: {line_number}:{char_number}");
            println!("line: {line:?}");
            println!("byte: {:?}", byte as char);
            // println!("up_data: {up_data:?}");
            println!("Stats:");
            println!("{}", parser.stats());

            if cut {
                println!("cut!");
                println!()
            }
            assert!(!right_data.is_empty() || !up_data.is_empty(), "Parser failed at byte: {} on line: {} at char: {}", byte as char, line_number + 1, char_number + 1);
        }
    }
}

pub fn assert_parses_default<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    assert_parses(combinator, input, "Parser failed unexpectedly");
}

pub fn assert_fails<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, desc: &str) {
    let mut input = input.to_string();
    println!("beginning assert_fails {}", desc);
    let (mut parser, ParseResults { up_data_vec: mut up_data, .. }) = T::parser(&combinator, RightData::default());
    println!("constructed parser");

    let mut result = Ok(());

    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();
    'outer: for (line_number, line) in tqdm!(lines.iter().enumerate(), animation = "fillup", position = 0) {
        // Add newline back in
        let mut line = format!("{}", line);
        if line_number != num_lines - 1 {
            line = format!("{}\n", line);
        }
        let bytes = line.bytes().collect::<Vec<_>>();
        for (char_number, byte) in tqdm!(bytes.iter().cloned().enumerate(), animation = "fillup", position = 1) {
            println!("byte: {:?}\n\n\n\n", byte as char);
            let byte_is_in_some_up_data = up_data.iter().any(|up_data| up_data.u8set.contains(byte));
            // assert!(byte_is_in_some_up_data, "byte {:?} is not in any up_data: {:?}", byte as char, up_data);
            if !byte_is_in_some_up_data {
                println!("byte {:?} is not in any up_data: {:?}", byte as char, up_data);
                return;
            }

            if line_number == lines.len() - 1 && char_number == bytes.len() - 1 {
                break 'outer;
            }

            let ParseResults {
                right_data_vec: right_data,
                up_data_vec: new_up_data,
                cut,
                done,
            } = parser.step(byte);

            up_data = new_up_data;

            println!();
            println!("line:char: {line_number}:{char_number}");
            println!("line: {line:?}");
            println!("byte: {:?}", byte as char);
            // println!("up_data: {up_data:?}");
            println!("Stats:");
            println!("{}", parser.stats());

            if cut {
                println!("cut!");
                println!()
            }
            if !right_data.is_empty() || !up_data.is_empty() {
                result = Err(format!(
                    "Parser succeeded at byte: {} on line: {} at char: {}",
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

    assert!(result.is_err(), "{}", desc);
}

pub fn assert_fails_default<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    assert_fails(combinator, input, "Parser succeeded unexpectedly");
}