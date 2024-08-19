use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Result};
use std::ops::AddAssign;

use crate::{BruteForceParser, CacheContextParser, ChoiceParser, EatStringParser, EatU8Parser, ExcludeBytestringsParser, GLOBAL_CACHE, IndentCombinatorParser, match_parser, Parser, ProfiledParser, Repeat1Parser, SeqParser, TaggedParser, U8Set};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Stats {
    pub active_parser_type_counts: BTreeMap<String, usize>,
    pub active_symbols: BTreeMap<String, usize>,
    pub active_tags: BTreeMap<String, usize>,
    pub active_string_matchers: BTreeMap<String, usize>,
    pub active_u8_matchers: BTreeMap<U8Set, usize>,
    pub stats_by_tag: BTreeMap<String, Vec<Stats>>,
}

impl Stats {
    pub fn accumulate_tags(&self) -> Stats {
        let mut stats: Stats = self.clone();
        for (tag, other_stats_vec) in self.stats_by_tag.iter() {
            for other_stats in other_stats_vec.iter() {
                let other_stats = other_stats.accumulate_tags();
                for (name, count) in other_stats.active_parser_type_counts.iter() {
                    stats.active_parser_type_counts.entry(name.clone()).or_default().add_assign(*count);
                }
                for (name, count) in other_stats.active_symbols.iter() {
                    stats.active_symbols.entry(name.clone()).or_default().add_assign(*count);
                }
                for (name, count) in other_stats.active_tags.iter() {
                    stats.active_tags.entry(name.clone()).or_default().add_assign(*count);
                }
                for (name, count) in other_stats.active_string_matchers.iter() {
                    stats.active_string_matchers.entry(name.clone()).or_default().add_assign(*count);
                }
                for (name, count) in other_stats.active_u8_matchers.iter() {
                    stats.active_u8_matchers.entry(name.clone()).or_default().add_assign(*count);
                }
            }
        }
        stats
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut lines = vec![];

        // Overview section
        lines.push("Total Stats".to_string());
        lines.push("══════════════".to_string());
        lines.push("".to_string());
        let accumulated_stats = self.accumulate_tags();
        let mut total_blocks = vec![
            create_block("Parser Types", accumulated_stats.total_active_parsers(), &accumulated_stats.active_parser_type_counts),
            create_block("Tags", accumulated_stats.total_active_tags(), &accumulated_stats.active_tags),
            create_block("Symbols", accumulated_stats.total_active_symbols(), &accumulated_stats.active_symbols),
            create_block("String Matchers", accumulated_stats.total_active_string_matchers(), &accumulated_stats.active_string_matchers),
            create_block("U8 Matchers", accumulated_stats.total_active_u8_matchers(), &accumulated_stats.active_u8_matchers),
        ];
        total_blocks.retain(|b| !b.is_empty());
        lines.extend(join_vecs_horizontally_with_separator(&total_blocks, "   "));
        lines.push("".to_string());

        lines.push("Top Level Stats".to_string());
        lines.push("══════════════".to_string());
        lines.push("".to_string());
        let mut overview_blocks = vec![
            create_block("Parser Types", self.total_active_parsers(), &self.active_parser_type_counts),
            create_block("Tags", self.total_active_tags(), &self.active_tags),
            create_block("Symbols", self.total_active_symbols(), &self.active_symbols),
            create_block("String Matchers", self.total_active_string_matchers(), &self.active_string_matchers),
            create_block("U8 Matchers", self.total_active_u8_matchers(), &self.active_u8_matchers),
        ];
        overview_blocks.retain(|b| !b.is_empty());
        lines.extend(join_vecs_horizontally_with_separator(&overview_blocks, "   "));
        lines.push("".to_string());

        lines.push("Nested Stats".to_string());
        lines.push("════════════".to_string());
        lines.push("".to_string());

        let mut blocks_for_nested_tags = vec![];
        for block in create_nested_stats(&self.stats_by_tag) {
            let mut lines = vec![];
            for line in block {
                lines.push(format!("│ {}", line));
            }
            blocks_for_nested_tags.push(lines);
        }

        if !blocks_for_nested_tags.is_empty() {
            lines.extend(join_vecs_vertically_with_separator(&blocks_for_nested_tags, vec![String::new()]));
            lines.push("".to_string());
        }

        write!(f, "{}", lines.join("\n"))
    }
}

fn join_vecs_vertically_with_separator(vecs: &[Vec<String>], separator: Vec<String>) -> Vec<String> {
    vecs.iter()
        .flat_map(|v| v.iter().chain(&separator))
        .cloned()
        .collect::<Vec<_>>()
}

fn join_vecs_horizontally_with_separator(vecs: &[Vec<String>], separator: &str) -> Vec<String> {
    let max_lines = vecs.iter().map(|v| v.len()).max().unwrap_or(0);
    let padded_vecs: Vec<Vec<String>> = vecs.iter().map(|v| pad_lines_into_square(pad_lines_vertically(v, max_lines))).collect();
    let mut lines = padded_vecs[0].clone();
    for padded_vec in padded_vecs.iter().skip(1) {
        for i in 0..max_lines {
            lines[i] = format!("{}{}{}", lines[i], separator, padded_vec[i]);
        }
    }
    lines
}

fn pad_lines_vertically(lines: &[String], max_lines: usize) -> Vec<String> {
    let mut padded_lines = lines.to_vec();
    while padded_lines.len() < max_lines {
        padded_lines.push(String::new());
    }
    padded_lines
}

fn pad_lines_into_square(lines: Vec<String>) -> Vec<String> {
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    lines.iter().map(|l| format!("{:<width$}", l, width = max_width)).collect()
}

fn create_block(title: &str, total: usize, items: &BTreeMap<impl ToString, usize>) -> Vec<String> {
    if items.is_empty() {
        return vec![];
    }
    let mut lines = vec![title.to_string()];
    let mut items: Vec<_> = items.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));
    for (key, value) in items.iter().take(3) {
        const PADDING: usize = 32;
        lines.push(format!("▪ {:<PADDING$} {:>3}", truncate(&key.to_string(), PADDING), value));
    }
    lines
}

fn create_nested_stats(stats_by_tag: &BTreeMap<String, Vec<Stats>>) -> Vec<Vec<String>> {
    let mut blocks = vec![];

    for (tag, stats_vec) in stats_by_tag {
        for stats in stats_vec.iter() {
            let mut lines = vec![];
            lines.push(tag.to_string());

            let mut inner_blocks_for_stats = vec![
                create_block("Parser Types", stats.total_active_parsers(), &stats.active_parser_type_counts),
                create_block("Tags", stats.total_active_tags(), &stats.active_tags),
                create_block("Symbols", stats.total_active_symbols(), &stats.active_symbols),
                create_block("String Matchers", stats.total_active_string_matchers(), &stats.active_string_matchers),
                create_block("U8 Matchers", stats.total_active_u8_matchers(), &stats.active_u8_matchers),
            ];
            inner_blocks_for_stats.retain(|b| !b.is_empty());

            lines.extend(join_vecs_horizontally_with_separator(&inner_blocks_for_stats, "   "));

            // Add a vertical pipe at the start of each line
            let mut inner_blocks_for_tags = create_nested_stats(&stats.stats_by_tag);
            for mut block in inner_blocks_for_tags.iter_mut() {
                for line in block.iter_mut() {
                    *line = format!("│ {}", line);
                }
            }

            if !inner_blocks_for_tags.is_empty() {
                lines.push("".to_string());
                lines.extend(join_vecs_vertically_with_separator(&inner_blocks_for_tags, vec![String::new()]));
            }

            blocks.push(lines);
        }
    }

    blocks
}

fn truncate(s: &str, max_chars: usize) -> String {
    const JUST_RETURN_LAST_PART: bool = true;
    if s.len() > max_chars {
        if JUST_RETURN_LAST_PART {
            return s.split("::").last().unwrap().to_string();
        }

        // First, split by "::" and prioritize keeping the left-most parts
        let mut parts: Vec<&str> = s.split("::").collect();
        let mut truncated = parts.last().unwrap().to_string();
        for part in parts.iter().rev().skip(1) {
            let final_truncated = format!("...::{}::{}", part, truncated);
            if final_truncated.len() > max_chars {
                break;
            } else {
                truncated = format!("{}::{}", part, truncated);
            }
        }

        // Add ellipsis
        truncated = format!("...::{}", truncated);

        // If the string is still too long, truncate end
        if truncated.len() > max_chars {
            truncated.truncate(max_chars);
            truncated.push_str("...");
        }
        truncated
    } else {
        s.to_string()
    }
}

impl Stats {
    pub fn total_active_parsers(&self) -> usize {
        self.active_parser_type_counts.values().sum()
    }

    pub fn total_active_symbols(&self) -> usize {
        self.active_symbols.values().sum()
    }

    pub fn total_active_tags(&self) -> usize {
        self.active_tags.values().sum()
    }

    pub fn total_active_string_matchers(&self) -> usize {
        self.active_string_matchers.values().sum()
    }

    pub fn total_active_u8_matchers(&self) -> usize {
        self.active_u8_matchers.values().sum()
    }
}

impl Parser<'_> {
    pub fn stats(&self) -> Stats {
        let mut stats = Stats::default();
        self.collect_stats(&mut stats, None);
        stats
    }

    fn collect_stats(&self, stats: &mut Stats, current_tag: Option<&String>) {
        match self {
            Parser::SeqParser(SeqParser { parsers, .. }) => {
                parsers.iter().for_each(|(_, parser)| parser.collect_stats(stats, current_tag));
            }
            Parser::ChoiceParser(ChoiceParser { parsers, greedy }) => {
                parsers.iter().for_each(|p| p.collect_stats(stats, current_tag));
            }
            Parser::EatU8Parser(EatU8Parser { u8set, .. }) => {
                stats.active_u8_matchers.entry(u8set.clone()).or_default().add_assign(1);
            }
            Parser::EatStringParser(EatStringParser { string, .. }) => {
                stats.active_string_matchers.entry(String::from_utf8_lossy(string).to_string()).or_default().add_assign(1);
            }
            Parser::CacheContextParser(CacheContextParser { inner, .. }) => {
                inner.collect_stats(stats, current_tag);
                // for entry in GLOBAL_CACHE.with(|cache| cache.borrow().entries.iter()) {
                //     entry.borrow().parser.as_ref().map(|p| p.collect_stats(stats, current_tag));
                // }
            }
            Parser::Repeat1Parser(Repeat1Parser { a_parsers, .. }) => {
                a_parsers.iter().for_each(|p| p.collect_stats(stats, current_tag));
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::DentParser(parser)) => parser.parser.as_ref().unwrap().collect_stats(stats, current_tag),

            Parser::EpsParser(_) |
            Parser::FailParser(_) |
            Parser::CachedParser(_) |
            Parser::EatByteStringChoiceParser(_) => { },
            Parser::ExcludeBytestringsParser(ExcludeBytestringsParser { inner, .. }) => {
                inner.collect_stats(stats, current_tag);
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::IndentParser(_)) |
            Parser::IndentCombinatorParser(IndentCombinatorParser::Done) => {}
            Parser::ProfiledParser(ProfiledParser { inner, .. }) => {
                inner.collect_stats(stats, current_tag);
            },
            Parser::BruteForceParser(_) => {},
            Parser::ContinuationParser(_) => {},
            Parser::FastParserWrapper(_) => {}
            Parser::DynParser(_) => todo!(),
            Parser::OwningParser(_) => todo!(),
            Parser::TaggedParser(TaggedParser { inner, tag }) => {
                inner.collect_stats(stats, Some(tag));
            },
        }
        stats.active_parser_type_counts.entry(self.type_name()).or_default().add_assign(1);
    }

    fn type_name(&self) -> String {
        match_parser!(self, inner => std::any::type_name_of_val(&inner)).to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[ignore]
    #[test]
    fn test_stats_display() {
        let mut stats = Stats::default();

        // Top-level stats
        stats.active_parser_type_counts = BTreeMap::from([
            ("SeqParser".to_string(), 50),
            ("ChoiceParser".to_string(), 30),
            ("TaggedParser".to_string(), 20),
        ]);
        stats.active_tags = BTreeMap::from([
            ("expression".to_string(), 40),
            ("statement".to_string(), 25),
            ("declaration".to_string(), 15),
        ]);
        stats.active_symbols = BTreeMap::from([
            ("identifier".to_string(), 30),
            ("operator".to_string(), 20),
            ("keyword".to_string(), 10),
        ]);
        stats.active_string_matchers = BTreeMap::from([
            ("function".to_string(), 20),
            ("let".to_string(), 15),
            ("const".to_string(), 5),
        ]);
        stats.active_u8_matchers = BTreeMap::from([
            (U8Set::from_byte_range(b'a'..=b'z'), 10),
            (U8Set::from_byte_range(b'0'..=b'9'), 8),
            (U8Set::from_byte_range(b'A'..=b'Z'), 2),
        ]);

        // Nested stats
        let mut expression_stats = Stats::default();
        expression_stats.active_parser_type_counts = BTreeMap::from([
            ("ChoiceParser".to_string(), 60),
            ("SeqParser".to_string(), 40),
            ("TaggedParser".to_string(), 20),
        ]);
        expression_stats.active_tags = BTreeMap::from([
            ("binary_expr".to_string(), 40),
            ("unary_expr".to_string(), 25),
            ("literal".to_string(), 15),
        ]);

        let mut binary_expr_stats = Stats::default();
        binary_expr_stats.active_parser_type_counts = BTreeMap::from([
            ("SeqParser".to_string(), 30),
            ("ChoiceParser".to_string(), 20),
            ("TaggedParser".to_string(), 10),
        ]);
        binary_expr_stats.active_symbols = BTreeMap::from([
            ("operator".to_string(), 25),
            ("identifier".to_string(), 15),
        ]);
        binary_expr_stats.active_string_matchers = BTreeMap::from([
            ("+".to_string(), 15),
            ("-".to_string(), 10),
            ("*".to_string(), 5),
        ]);
        binary_expr_stats.active_u8_matchers = BTreeMap::from([
            (U8Set::from_byte_range(b'a'..=b'z'), 10),
            (U8Set::from_byte_range(b'0'..=b'9'), 8),
            (U8Set::from_byte_range(b'A'..=b'Z'), 2),
        ]);

        let mut nested_binary_stats = Stats::default();
        nested_binary_stats.active_parser_type_counts = BTreeMap::from([
            ("SeqParser".to_string(), 20),
            ("TaggedParser".to_string(), 10),
        ]);
        nested_binary_stats.active_symbols = BTreeMap::from([
            ("operator".to_string(), 15),
            ("identifier".to_string(), 5),
        ]);
        binary_expr_stats.stats_by_tag.insert("nested_binary".to_string(), vec![nested_binary_stats]);

        let mut unary_expr_stats = Stats::default();
        unary_expr_stats.active_parser_type_counts = BTreeMap::from([
            ("SeqParser".to_string(), 20),
            ("TaggedParser".to_string(), 10),
        ]);
        unary_expr_stats.active_symbols = BTreeMap::from([
            ("operator".to_string(), 15),
            ("identifier".to_string(), 5),
        ]);
        unary_expr_stats.active_u8_matchers = BTreeMap::from([
            (U8Set::from_byte(b'!'), 10),
            (U8Set::from_byte(b'-'), 5),
        ]);
        unary_expr_stats.active_tags = BTreeMap::from([
            ("prefix".to_string(), 7),
            ("postfix".to_string(), 3),
        ]);

        let mut literal_stats = Stats::default();
        literal_stats.active_string_matchers = BTreeMap::from([
            ("true".to_string(), 15),
            ("false".to_string(), 10),
            ("null".to_string(), 5),
        ]);

        expression_stats.stats_by_tag.insert("binary_expr".to_string(), vec![binary_expr_stats]);
        expression_stats.stats_by_tag.insert("unary_expr".to_string(), vec![unary_expr_stats]);
        expression_stats.stats_by_tag.insert("literal".to_string(), vec![literal_stats]);

        let mut statement_stats = Stats::default();
        statement_stats.active_parser_type_counts = BTreeMap::from([
            ("SeqParser".to_string(), 50),
            ("ChoiceParser".to_string(), 30),
            ("TaggedParser".to_string(), 10),
        ]);
        statement_stats.active_tags = BTreeMap::from([
            ("if_statement".to_string(), 30),
            ("for_loop".to_string(), 20),
            ("while_loop".to_string(), 10),
        ]);

        let mut if_statement_stats = Stats::default();
        if_statement_stats.active_parser_type_counts = BTreeMap::from([
            ("SeqParser".to_string(), 25),
            ("TaggedParser".to_string(), 15),
        ]);
        if_statement_stats.active_symbols = BTreeMap::from([
            ("keyword".to_string(), 15),
            ("operator".to_string(), 10),
        ]);
        if_statement_stats.active_string_matchers = BTreeMap::from([
            ("if".to_string(), 10),
            ("else".to_string(), 5),
        ]);
        statement_stats.stats_by_tag.insert("if_statement".to_string(), vec![if_statement_stats]);

        let mut declaration_stats = Stats::default();
        declaration_stats.active_parser_type_counts = BTreeMap::from([
            ("SeqParser".to_string(), 40),
            ("TaggedParser".to_string(), 20),
            ("ChoiceParser".to_string(), 10),
        ]);
        declaration_stats.active_symbols = BTreeMap::from([
            ("identifier".to_string(), 20),
            ("keyword".to_string(), 10),
        ]);

        let mut variable_decl_stats = Stats::default();
        variable_decl_stats.active_parser_type_counts = BTreeMap::from([
            ("SeqParser".to_string(), 15),
            ("TaggedParser".to_string(), 10),
        ]);
        variable_decl_stats.active_symbols = BTreeMap::from([
            ("identifier".to_string(), 12),
            ("operator".to_string(), 8),
        ]);
        variable_decl_stats.active_string_matchers = BTreeMap::from([
            ("let".to_string(), 10),
            ("const".to_string(), 5),
        ]);
        declaration_stats.stats_by_tag.insert("variable_decl".to_string(), vec![variable_decl_stats]);

        stats.stats_by_tag.insert("expression".to_string(), vec![expression_stats]);
        stats.stats_by_tag.insert("statement".to_string(), vec![statement_stats]);
        stats.stats_by_tag.insert("declaration".to_string(), vec![declaration_stats]);

        let expected_output = r#"Stats Overview
══════════════

Parser Types (100)    Tags (80)           Symbols (60)
▪ SeqParser     50    ▪ expression   40   ▪ identifier   30
▪ ChoiceParser  30    ▪ statement    25   ▪ operator     20
▪ TaggedParser  20    ▪ declaration  15   ▪ keyword      10

String Matchers (40)  U8 Matchers (20)
▪ "function"    20    ▪ [a-z]        10
▪ "let"         15    ▪ [0-9]         8
▪ "const"        5    ▪ [A-Z]         2

Nested Stats
════════════

expression (200)
│  Parser Types (120)    │ Tags (80)
│  ▪ ChoiceParser   60   │ ▪ binary_expr   40
│  ▪ SeqParser      40   │ ▪ unary_expr    25
│  ▪ TaggedParser   20   │ ▪ literal       15
│
├─ binary_expr (100)
│  │ Parser Types (60)   │ Symbols (40)        │ String Matchers (30)
│  │ ▪ SeqParser    30   │ ▪ operator     25   │ ▪ "+"             15
│  │ ▪ ChoiceParser 20   │ ▪ identifier   15   │ ▪ "-"             10
│  │ ▪ TaggedParser 10   │                     │ ▪ "*"              5
│  │
│  │ U8 Matchers (20)
│  │ ▪ [a-z]        10
│  │ ▪ [0-9]         8
│  │ ▪ [A-Z]         2
│  │
│  ├─ nested_binary (50)
│  │  Parser Types (30)   │ Symbols (20)
│  │  ▪ SeqParser    20   │ ▪ operator     15
│  │  ▪ TaggedParser 10   │ ▪ identifier    5
│
├─ unary_expr (50)
│  Parser Types (30)      │ Symbols (20)        │ U8 Matchers (15)
│  ▪ SeqParser      20    │ ▪ operator     15   │ ▪ [!]         10
│  ▪ TaggedParser   10    │ ▪ identifier    5   │ ▪ [-]          5
│
│ Tags (10)
│ ▪ prefix  7
│ ▪ postfix 3
│
├─ literal (30)
│  String Matchers (30)
│  ▪ "true"     15
│  ▪ "false"    10
│  ▪ "null"      5

statement (150)
│  Parser Types (90)   │ Tags (60)
│  ▪ SeqParser    50   │ ▪ if_statement  30
│  ▪ ChoiceParser 30   │ ▪ for_loop      20
│  ▪ TaggedParser 10   │ ▪ while_loop    10
│
├─ if_statement (80)
│  │ Parser Types (40)   │ Symbols (25)     │ String Matchers (15)
│  │ ▪ SeqParser    25   │ ▪ keyword   15   │ ▪ "if"     10
│  │ ▪ TaggedParser 15   │ ▪ operator  10   │ ▪ "else"    5

declaration (100)
│  Parser Types (70)   │ Symbols (30)
│  ▪ SeqParser    40   │ ▪ identifier    20
│  ▪ TaggedParser 20   │ ▪ keyword       10
│  ▪ ChoiceParser 10   │
│
├─ variable_decl (60)
│  │ Parser Types (25)   │ Symbols (20)     │ String Matchers (15)
│  │ ▪ SeqParser    15   │ ▪ identifier 12  │ ▪ "let"   10
│  │ ▪ TaggedParser 10   │ ▪ operator   8   │ ▪ "const"  5"#;

        println!("{}", stats);
        assert_eq!(stats.to_string(), expected_output);
    }
}