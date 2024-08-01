use std::fmt::{Result};
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use crate::{Parser, U8Set, SeqParser, ChoiceParser, EatU8Parser, EatStringParser, CacheContextParser, FrameStackOpParser, SymbolParser, TaggedParser, Repeat1Parser, WithNewFrameParser, IndentCombinatorParser, match_parser};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Stats {
    pub active_parser_type_counts: BTreeMap<String, usize>,
    pub active_symbols: BTreeMap<String, usize>,
    pub active_tags: BTreeMap<String, usize>,
    pub active_string_matchers: BTreeMap<String, usize>,
    pub active_u8_matchers: BTreeMap<U8Set, usize>,
    pub stats_by_tag: BTreeMap<String, Vec<Stats>>,
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Stats Overview")?;
        writeln!(f, "│")?;

        self.format_section(f, "Active Parser Types", &self.active_parser_type_counts, self.total_active_parsers())?;
        self.format_section(f, "Active Tags", &self.active_tags, self.total_active_tags())?;
        self.format_section(f, "Active Symbols", &self.active_symbols, self.total_active_symbols())?;
        self.format_section(f, "Active String Matchers", &self.active_string_matchers, self.total_active_string_matchers())?;
        self.format_section(f, "Active U8 Matchers", &self.active_u8_matchers, self.total_active_u8_matchers())?;

        if !self.stats_by_tag.is_empty() {
            writeln!(f, "Stats by Tag")?;
            writeln!(f, "│")?;

            let mut tags: Vec<_> = self.stats_by_tag.iter().collect();
            tags.sort_by_key(|(tag, _)| *tag);

            for (i, (tag, stats_vec)) in tags.iter().enumerate() {
                let is_last = i == tags.len() - 1;
                let prefix = if is_last { "└─ " } else { "├─ " };
                writeln!(f, "{}{}",  prefix, tag)?;

                for (j, stats) in stats_vec.iter().enumerate() {
                    let is_last_stat = j == stats_vec.len() - 1;
                    let stat_prefix = if is_last {
                        if is_last_stat { "   " } else { "   │" }
                    } else {
                        if is_last_stat { "│  " } else { "│  │" }
                    };

                    for (k, line) in stats.to_string().lines().enumerate() {
                        if k == 0 {
                            writeln!(f, "{}  {}", stat_prefix, line)?;
                        } else {
                            writeln!(f, "{}     {}", stat_prefix, line)?;
                        }
                    }

                    if !is_last_stat {
                        writeln!(f, "{}  │", stat_prefix)?;
                    }
                }

                if !is_last {
                    writeln!(f, "│")?;
                }
            }
        }

        Ok(())
    }
}

impl Stats {
    fn format_section<T: Display>(&self, f: &mut Formatter, title: &str, items: &BTreeMap<T, usize>, total: usize) -> Result {
        writeln!(f, "├─ {} (Total: {})", title, total)?;

        let mut sorted_items: Vec<_> = items.iter().collect();
        sorted_items.sort_by_key(|(_, &count)| Reverse(count));

        for (i, (name, count)) in sorted_items.iter().enumerate() {
            let is_last = i == sorted_items.len() - 1;
            let prefix = if is_last { "   └─ " } else { "   ├─ " };

            let name_str = name.to_string();
            let display_name = if name_str.len() > 80 {
                format!("{}...", &name_str[..77])
            } else {
                name_str
            };

            writeln!(f, "{}{}:{} {}", prefix, display_name, " ".repeat(20_usize.saturating_sub(display_name.len())), count)?;
        }

        writeln!(f, "│")
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

impl Parser {
    pub fn stats(&self) -> Stats {
        let mut stats = Stats::default();
        self.collect_stats(&mut stats, None);
        stats
    }

    fn collect_stats(&self, stats: &mut Stats, current_tag: Option<&String>) {
        match self {
            Parser::SeqParser(SeqParser { children, .. }) => {
                children.iter().for_each(|(_, parsers)| {
                    parsers.iter().for_each(|p| p.collect_stats(stats, current_tag));
                });
            }
            Parser::ChoiceParser(ChoiceParser { parsers }) => {
                parsers.iter().for_each(|p| p.collect_stats(stats, current_tag));
            }
            Parser::EatU8Parser(EatU8Parser { u8set, .. }) => {
                stats.active_u8_matchers.entry(u8set.clone()).or_default().add_assign(1);
            }
            Parser::EatStringParser(EatStringParser { string, .. }) => {
                stats.active_string_matchers.entry(String::from_utf8_lossy(string).to_string()).or_default().add_assign(1);
            }
            Parser::CacheContextParser(CacheContextParser { inner, cache_data_inner, .. }) => {
                inner.collect_stats(stats, current_tag);
                for entry in cache_data_inner.borrow().entries.iter() {
                    entry.borrow().parser.as_ref().map(|p| p.collect_stats(stats, current_tag));
                }
            }
            Parser::FrameStackOpParser(FrameStackOpParser { a: inner, .. }) |
            Parser::SymbolParser(SymbolParser { inner, .. }) => inner.collect_stats(stats, current_tag),
            Parser::TaggedParser(TaggedParser { inner, tag }) => {
                let mut tag_stats = Stats::default();
                inner.collect_stats(&mut tag_stats, Some(tag));
                stats.stats_by_tag.entry(tag.clone()).or_default().push(tag_stats);
                stats.active_tags.entry(tag.clone()).or_default().add_assign(1);
            }
            Parser::Repeat1Parser(Repeat1Parser { a_parsers, .. }) => {
                a_parsers.iter().for_each(|p| p.collect_stats(stats, current_tag));
            }
            Parser::WithNewFrameParser(WithNewFrameParser { a, .. }) => {
                a.as_ref().map(|a| a.collect_stats(stats, current_tag));
            }
            Parser::IndentCombinatorParser(IndentCombinatorParser::DentParser(parser)) => parser.collect_stats(stats, current_tag),
            _ => {}
        }
        stats.active_parser_type_counts.entry(self.type_name()).or_default().add_assign(1);
    }

    fn type_name(&self) -> String {
        match_parser!(self, inner => std::any::type_name_of_val(&inner)).to_string()
    }
}