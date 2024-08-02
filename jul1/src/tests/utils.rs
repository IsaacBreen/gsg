use std::panic::{AssertUnwindSafe, catch_unwind};

use kdam::tqdm;

use crate::{CombinatorTrait, ParseResults, ParserTrait, RightData, Squash};

use std::time::Instant;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::iter;

const VERBOSE: bool = false;

pub fn assert_parses<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, desc: &str) {
    let mut input = input.to_string();
    println!("beginning assert_parses {}", desc);

    let mut timings: Vec<(String, std::time::Duration)> = Vec::new();

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

        for (char_number, byte) in bytes.iter().cloned().enumerate() {
            let byte_is_in_some_up_data = up_data.iter().any(|up_data| up_data.u8set.contains(byte));
            assert!(byte_is_in_some_up_data, "byte {:?} is not in any up_data: {:?}. Line: {:?}, Char: {:?}, Text: {:?}", byte as char, up_data, line_number + 1, char_number + 1, line);

            if line_number == lines.len() - 1 && char_number == bytes.len() - 1 {
                timings.push((line.to_string(), Instant::now() - line_start));
                break 'outer;
            }

            if VERBOSE {
                // Print useful info
                println!("line:char: {line_number}:{char_number}");
                println!("line: {line:?}");
                let stats = parser.stats();
                println!("Stats:");
                println!("{}", stats);
            }

            let ParseResults {
                right_data_vec: right_data,
                up_data_vec: new_up_data,
                done,
            } = catch_unwind(AssertUnwindSafe(|| parser.step(byte).squashed())).expect(format!("Parser.step: Error at byte: {} on line: {} at char: {}", byte as char, line_number + 1, char_number + 1).as_str());

            up_data = new_up_data;
            up_data.squash();

            assert!(!right_data.is_empty() || !up_data.is_empty(), "Parser didn't return any data at byte: {} on line: {} at char: {}", byte as char, line_number + 1, char_number + 1);
            assert!(!done, "Parser finished prematurely at byte: {} on line: {} at char: {}", byte as char, line_number + 1, char_number + 1);
        }

        timings.push((line.to_string(), Instant::now() - line_start));
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

    if VERBOSE {
        println!("Execution time profile:");
        for (desc, duration) in timing_vec.clone() {
            let duration_secs = duration.as_secs_f64();
            let time_per_char = duration_secs / desc.len() as f64 * 1000.0;
            let emphasis = if duration > threshold { " * " } else { "   " };
            let bold = if duration > threshold { "\x1b[1m" } else { "" };
            let reset = if bold.is_empty() { "" } else { "\x1b[0m" };
            println!("{}{:<15}{:<10}{}{:?}{}s",
                     emphasis,
                     format!("{:.3}ms/char", time_per_char),
                     format!("{:.3}s", duration_secs),
                     bold,
                     desc,
                     reset,
            );
        }
    }

    // Save to CSV
    let mut csv_file = BufWriter::new(File::create("timings.csv").unwrap());
    csv_file.write_all("index,text,duration\n".as_bytes()).unwrap();
    for (i, (line, duration)) in timing_vec.iter().enumerate() {
        // Escape quotes and newlines in the text
        let line = line.replace("\"", "\"\"");
        let line = line.replace("\n", "\\n");
        csv_file.write_all(format!("{},\"{}\",{}\n", i, line, duration.as_secs_f64()).as_bytes()).unwrap();
    }
    if VERBOSE {
        println!("Saved timings to timings.csv");
    }
}

pub fn assert_parses_default<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    assert_parses(combinator, input, "Parser failed unexpectedly");
}

pub fn assert_parses_fast<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    let bytes = input.to_string().bytes().collect::<Vec<_>>();
    let (parser, mut parse_results) = combinator.parser_with_steps(RightData::default(), &bytes);
    parse_results.squash();
    // todo: uncomment this for unambiguous parses
    // let [right_data] = parse_results.right_data_vec.as_slice() else { panic!("Expected one right data, but found {:?}", parse_results.right_data_vec) };
    // Get the right data with the highest position
    // Ensure the parser finished with right data at the end
    assert!(parse_results.right_data_vec.iter().max_by_key(|right_data| right_data.position).expect(format!("Expected at least one right data. parse_results: {:?}", parse_results).as_str()).position == bytes.len(), "Expected parser to finish with right data at the end position {}. parse_results: {:?}", bytes.len(), parse_results);
}

pub fn assert_parses_fast_with_tolerance<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, tolerance: usize) {
    let bytes = input.to_string().bytes().collect::<Vec<_>>();
    let (parser, mut parse_results) = combinator.parser_with_steps(RightData::default(), &bytes);
    parse_results.squash();
    // todo: uncomment this for unambiguous parses
    // let [right_data] = parse_results.right_data_vec.as_slice() else { panic!("Expected one right data, but found {:?}", parse_results.right_data_vec) };
    // Get the right data with the highest position
    // Ensure the parser is still going or that it finished with right data at the end (within tolerance)
    assert!(parse_results.right_data_vec.iter().max_by_key(|right_data| right_data.position).expect(format!("Expected at least one right data. parse_results: {:?}", parse_results).as_str()).position >= bytes.len().saturating_sub(tolerance), "Expected parser to finish with right data at the end position {}. parse_results: {:?}", bytes.len(), parse_results);
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

pub fn assert_fails_fast<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    let (mut parser, _) = combinator.parser(RightData::default());
    let bytes = input.to_string().bytes().collect::<Vec<_>>();
    let parse_results = parser.steps(&bytes);
    assert!(parse_results.done && parse_results.right_data_vec.iter().max_by_key(|right_data| right_data.position).map_or(true, |right_data| right_data.position == bytes.len()), "Expected parser to fail at the end. parse_results: {:?}", parse_results);
}