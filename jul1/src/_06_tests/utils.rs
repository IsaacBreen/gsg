use crate::{clear_profile_data, RightData, UnambiguousParseResults};
use std::panic::{catch_unwind, AssertUnwindSafe};

use kdam::tqdm;

use crate::{profile, CombinatorTrait, CombinatorTraitExt, ParseResultTrait, ParseResults, ParserTrait, ParserTraitExt, Squash, GLOBAL_PROFILE_DATA};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{Duration, Instant};

const VERBOSE: bool = false;

pub fn assert_parses<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, desc: &str) {
    clear_profile_data();
    let input = input.to_string();
    println!("beginning assert_parses {}", desc);

    let mut timings = Vec::new();

    let (mut parser, mut parse_results) = profile!("parser", combinator.parser(RightData::default()));

    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();

    profile!("assert_parses main loop", {
        for (line_number, line) in tqdm!(lines.iter().enumerate(), animation = "fillup", position = 0) {
            let line_start = Instant::now();

            let line = if line_number != num_lines - 1 {
                format!("{}\n", line)
            } else {
                line.to_string()
            };
            let bytes = line.bytes().collect::<Vec<_>>();

            for (char_number, byte) in bytes.iter().cloned().enumerate() {
                parse_results.squash();
                assert!(parser.get_u8set().contains(byte),
                    "byte {:?} is not in any up_data: {:?}. Line: {:?}, Char: {:?}, Text: {:?}, u8set: {:?}",
                    byte as char, parse_results, line_number, char_number, line, parser.get_u8set());

                if line_number == lines.len() - 1 && char_number == bytes.len() - 1 {
                    timings.push((line.to_string(), Instant::now() - line_start));
                    break;
                }

                if VERBOSE {
                    println!("line:char: {line_number}:{char_number}");
                    println!("line: {line:?}");
                }

                parse_results = catch_unwind(AssertUnwindSafe(|| parser.step(byte)))
                    .expect(format!("Parser.step: Error at byte: {} on line: {} at char: {}", byte as char, line_number, char_number).as_str());

                parse_results.squash();

                assert!(!parse_results.right_data_vec.is_empty() || !parser.get_u8set().is_empty(),
                    "Parser didn't return any data at byte: {} on line: {} at char: {}", byte as char, line_number, char_number);
                assert!(!parse_results.done(),
                    "Parser finished prematurely at byte: {} on line: {} at char: {}", byte as char, line_number, char_number);
            }

            timings.push((line, Instant::now() - line_start));
        }
    });

    print_profile_results();

    print_timing_results(timings);
}

pub fn assert_parses_tight<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, desc: &str) {
    clear_profile_data();
    let input = input.to_string();
    println!("beginning assert_parses_tight {}", desc);

    let (mut parser, mut parse_results) = combinator.parser(RightData::default());

    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();

    let start = Instant::now();

    profile!("assert_parses_tight main loop", {
        for (line_number, line) in lines.iter().enumerate() {
            let line = if line_number != num_lines - 1 {
                format!("{}\n", line)
            } else {
                line.to_string()
            };
            let bytes = line.bytes().collect::<Vec<_>>();

            for (char_number, byte) in bytes.iter().cloned().enumerate() {
                if line_number == lines.len() - 1 && char_number == bytes.len() - 1 {
                    break;
                }
                parse_results = parser.step(byte);
            }
        }
    });

    println!("assert_parses_tight took {:?}", start.elapsed());
}

pub fn assert_parses_default<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    assert_parses(combinator, input, "Parser failed unexpectedly");
}

pub fn profile_parse<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    println!("beginning profile_parse");

    let (mut parser, mut parse_results) = combinator.parser(RightData::default());

    for byte in tqdm!(input.to_string().bytes(), animation = "fillup", position = 0) {
        parser.step(byte);
    }

    print_profile_results();
}


pub fn assert_parses_fast<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    let bytes = input.to_string().bytes().collect::<Vec<_>>();
    let start = Instant::now();
    let (parser, mut parse_results) = profile!("assert_parses_fast parse", {
        combinator.parse(RightData::default(), &bytes)
    });
    let duration = start.elapsed();
    println!("assert_parses_fast parse took {:?}", duration);
    parse_results.squash();

    let max_position = parse_results.right_data_vec
        .iter()
        .max_by_key(|right_data| right_data.right_data_inner.fields1.position)
        .expect("Expected at least one right data.")
        .right_data_inner.fields1.position;

    let (line_number, char_number) = calculate_line_and_char_number(&bytes, max_position);

    print_profile_results();

    parse_results.squash();

    println!("max_position: {max_position}, line_number: {line_number}, char_number: {char_number}");
    assert!(parse_results.right_data_vec
        .iter()
        .max_by_key(|right_data| right_data.right_data_inner.fields1.position)
        .expect("Expected at least one right data.")
        .right_data_inner.fields1.position == bytes.len(),
        "Expected parser to finish with right data at the end position {}. parse_results: {:?}", bytes.len(), parse_results);
}

pub fn assert_parses_fast_with_tolerance<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, tolerance: usize) {
    let bytes = input.to_string().bytes().collect::<Vec<_>>();
    let (parser, mut parse_results) = combinator.parse(RightData::default(), &bytes);
    parse_results.squash();

    let max_position = parse_results.right_data_vec
        .iter()
        .max_by_key(|right_data| right_data.right_data_inner.fields1.position)
        .expect("Expected at least one right data.")
        .right_data_inner.fields1.position;

    let (line_number, char_number) = calculate_line_and_char_number(&bytes, max_position);

    print_profile_results();

    assert!(parse_results.right_data_vec
        .iter()
        .max_by_key(|right_data| right_data.right_data_inner.fields1.position)
        .expect("Expected at least one right data.")
        .right_data_inner.fields1.position >= bytes.len().saturating_sub(tolerance),
        "Expected parser to finish with right data at the end position {}. parse_results: {:?}", bytes.len(), parse_results);
}

pub fn assert_parses_one_shot<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    let bytes = input.to_string().bytes().collect::<Vec<_>>();
    let start = Instant::now();
    let parse_results = profile!("assert_parses_fast parse", {
        combinator.one_shot_parse(RightData::default(), &bytes)
    });
    let right_data = parse_results.expect("Error parsing input.");
    let duration = start.elapsed();
    println!("assert_parses_fast parse took {:?}", duration);

    let max_position = right_data.right_data_inner.fields1.position;
    let (line_number, char_number) = calculate_line_and_char_number(&bytes, max_position);

    print_profile_results();

    println!("max_position: {max_position}, line_number: {line_number}, char_number: {char_number}");

    assert!(right_data.right_data_inner.fields1.position == bytes.len(),
        "Expected parser to finish with right data at the end position {}. right_data: {:?}", bytes.len(), right_data);
}

pub fn assert_parses_one_shot_with_result<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, expected_result: UnambiguousParseResults) {
    let bytes = input.to_string().bytes().collect::<Vec<_>>();
    let start = Instant::now();
    let parse_results = profile!("assert_parses_fast parse", {
        combinator.one_shot_parse(RightData::default(), &bytes)
    });
    let duration = start.elapsed();
    println!("assert_parses_fast parse took {:?}", duration);

    print_profile_results();

    assert_eq!(parse_results, expected_result, "Expected parse result {:?}, but got {:?}", expected_result, parse_results);
}

pub fn assert_fails<T: CombinatorTrait, S: ToString>(combinator: &T, input: S, desc: &str) {
    let input = input.to_string();
    println!("beginning assert_fails {}", desc);
    let (mut parser, ParseResults { .. }) = combinator.parser(RightData::default());

    let lines = input.lines().collect::<Vec<_>>();
    let num_lines = lines.len();
    for (line_number, line) in tqdm!(lines.iter().enumerate(), animation = "fillup", position = 0) {
        let line = if line_number != num_lines - 1 {
            format!("{}\n", line)
        } else {
            line.to_string()
        };
        let bytes = line.bytes().collect::<Vec<_>>();
        for (char_number, byte) in tqdm!(bytes.iter().cloned().enumerate(), animation = "fillup", position = 1) {
            let u8set = parser.get_u8set();
            if !u8set.contains(byte) {
                println!("byte {:?} is not in the u8set: {:?}", byte as char, u8set);
                return;
            }

            if line_number == lines.len() - 1 && char_number == bytes.len() - 1 {
                break;
            }

            let ParseResults {
                right_data_vec: right_data,
                ..
            } = parser.step(byte).squashed();

            if !right_data.is_empty() || !parser.get_u8set().is_empty() {
                panic!("Parser succeeded at byte: {} on line: {} at char: {}", byte as char, line_number, char_number);
            }
        }
    }

    panic!("{}", desc);
}

pub fn assert_fails_default<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    assert_fails(combinator, input, "Parser succeeded unexpectedly");
}

pub fn assert_fails_fast<T: CombinatorTrait, S: ToString>(combinator: &T, input: S) {
    let (mut parser, _) = combinator.parser(RightData::default());
    let bytes = input.to_string().bytes().collect::<Vec<_>>();
    let parse_results = parser.parse(&bytes);
    assert!(parse_results.done() && parse_results.right_data_vec.iter().max_by_key(|right_data| right_data.right_data_inner.fields1.position).map_or(true, |right_data| right_data.right_data_inner.fields1.position == bytes.len()), "Expected parser to fail at the end. parse_results: {:?}", parse_results);
}

// Helper functions to reduce code duplication

fn print_profile_results() {
    let profile_data = GLOBAL_PROFILE_DATA.lock().unwrap();
    let total_time = profile_data.timings.iter().map(|(_, duration)| *duration).sum::<Duration>();
    println!("Total time: {:?}", total_time);

    // Print profile results
    let mut profile_vec: Vec<(String, Duration)> = profile_data.timings.iter().map(|(tag, duration)| (tag.clone(), *duration)).collect::<Vec<_>>();
    profile_vec.sort_by(|(_, duration_a), (_, duration_b)| duration_b.partial_cmp(duration_a).unwrap());
    println!("Profile results:");
    for (tag, duration) in profile_vec.clone() {
        let percent = duration.as_secs_f64() / total_time.as_secs_f64() * 100.0;
        println!("{:>9} {:6.2}% {}", format!("{:.3?}", duration), percent, tag);
    }
    println!("Hit counts:");
    let total_hit_count = profile_data.hit_counts.values().sum::<usize>();
    let mut hit_counts = profile_data.hit_counts.iter().collect::<Vec<_>>();
    hit_counts.sort_by(|(_, hit_count_a), (_, hit_count_b)| hit_count_b.partial_cmp(hit_count_a).unwrap());
    for (tag, hit_count) in hit_counts.clone() {
        let percent = *hit_count as f64 / total_hit_count as f64 * 100.0;
        println!("{:>9} {:6.2}% {}", format!("{:.3?}", hit_count), percent, tag);
    }
    // Duration per hit
    println!("Duration per hit:");
    let mut duration_per_hit: HashMap<String, Duration> = HashMap::new();
    for (tag, hits) in profile_data.hit_counts.iter() {
        if let Some(duration) = profile_data.timings.get(tag) {
            duration_per_hit.insert(tag.clone(), *duration / *hits as u32);
        }
    }
    let mut duration_per_hit: Vec<(String, Duration)> = duration_per_hit.into_iter().collect::<Vec<_>>();
    duration_per_hit.sort_by(|(_, duration_a), (_, duration_b)| duration_b.partial_cmp(duration_a).unwrap());
    for (tag, duration) in duration_per_hit.clone() {
        let percent = duration.as_secs_f64() / total_time.as_secs_f64() * 100.0;
        println!("{:>9} {:6.2}% {}", format!("{:.3?}", duration), percent, tag);
    }
    drop(profile_data);
}

fn calculate_line_and_char_number(bytes: &[u8], max_position: usize) -> (usize, usize) {
    let mut line_number = 0;
    let mut char_number = 0;
    for byte in bytes[0..max_position].iter().cloned() {
        if byte == b'\n' {
            line_number += 1;
            char_number = 0;
        } else {
            char_number += 1;
        }
    }
    (line_number, char_number)
}

fn print_timing_results(timings: Vec<(String, Duration)>) {
    // Print timing results
    let mut timing_vec: Vec<(String, std::time::Duration)> = timings.into_iter().collect();

    // Get 90th percentile
    let mut timing_vec_sorted: Vec<(String, std::time::Duration)> = timing_vec.iter().cloned().collect();
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