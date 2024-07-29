use std::panic::catch_unwind;

use kdam::tqdm;

use crate::{CombinatorTrait, ParseResults, ParserTrait, RightData, Squash};

use std::time::Instant;
use std::collections::HashMap;

pub fn assert_parses<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, desc: &str) {
    let mut input = input.to_string();
    println!("beginning assert_parses {}", desc);

    let mut timings: HashMap<String, std::time::Duration> = HashMap::new();

    let start = Instant::now();
    let (mut parser, ParseResults { up_data_vec: mut up_data, .. }) = T::parser(&combinator, RightData::default());

    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();

    'outer: for (line_number, line) in tqdm!(lines.iter().enumerate(), animation = "fillup", position = 0) {
        let line_start = Instant::now();

        // Add newline back in
        let mut line = format!("{}", line);
        if line_number != num_lines - 1 {
            line = format!("{}\n", line);
        }
        let bytes = line.bytes().collect::<Vec<_>>();

        for (char_number, byte) in tqdm!(bytes.iter().cloned().enumerate(), animation = "fillup", position = 1) {
            let char_start = Instant::now();

            let byte_is_in_some_up_data = up_data.iter().any(|up_data| up_data.u8set.contains(byte));
            assert!(byte_is_in_some_up_data, "byte {:?} is not in any up_data: {:?}", byte as char, up_data);

            if line_number == lines.len() - 1 && char_number == bytes.len() - 1 {
                timings.insert(line.to_string(), Instant::now() - line_start);
                break 'outer;
            }

            let step_start = Instant::now();
            let ParseResults {
                right_data_vec: right_data,
                up_data_vec: new_up_data,
                done,
            } = parser.step(byte).squashed();

            up_data = new_up_data;

            assert!(!right_data.is_empty() || !up_data.is_empty(), "Parser failed at byte: {} on line: {} at char: {}", byte as char, line_number + 1, char_number + 1);
        }

        timings.insert(line.to_string(), Instant::now() - line_start);
    }

    // Print timing results
    let mut timing_vec: Vec<(String, std::time::Duration)> = timings.into_iter().collect();

    // Get 90th percentile
    let mut timing_vec_sorted: Vec<(String, std::time::Duration)> = timing_vec.iter().cloned().collect();
    // timing_vec_sorted.sort_by(|(line_a, duration_a), (line_b, duration_b)| (duration_b.as_secs_f64() / line_b.len() as f64).partial_cmp(&(duration_a.as_secs_f64() / line_a.len() as f64)).unwrap());
    timing_vec_sorted.sort_by(|(line_a, duration_a), (line_b, duration_b)| {
        let time_per_char_a = duration_a.as_secs_f64() / line_a.len() as f64;
        let time_per_char_b = duration_b.as_secs_f64() / line_b.len() as f64;
        time_per_char_b.partial_cmp(&time_per_char_a).unwrap()
    });
    let threshold = timing_vec_sorted[timing_vec_sorted.len() / 10].1;

    println!("Execution time profile:");
    for (desc, duration) in timing_vec {
        let duration_secs = duration.as_secs_f64();
        let emphasis = if duration > threshold { " * " } else { "   " };
        let bold = if duration > threshold { "\x1b[1m" } else { "" };
        let reset = if bold.is_empty() { "" } else { "\x1b[0m" };
        println!("{}{:<10}{}{:?}{}s",
                 emphasis,
                 format!("{:.3}s", duration_secs),
                 bold,
                 desc,
                 reset,
        );
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
                done,
            } = parser.step(byte).squashed();

            up_data = new_up_data;

            println!();
            println!("line:char: {line_number}:{char_number}");
            println!("line: {line:?}");
            println!("byte: {:?}", byte as char);
            // println!("up_data: {up_data:?}");
            println!("Stats:");
            println!("{}", parser.stats());

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