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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn write_sorted<S: Clone + Display>(f: &mut Formatter, title: &str, items: &[(S, usize)]) -> std::fmt::Result {
            writeln!(f, "{}", title)?;
            let mut sorted_items = items.to_vec();
            sorted_items.sort_by(|a, b| b.1.cmp(&a.1));
            for (name, count) in sorted_items {
                let mut name = name.to_string();
                if name.len() > 80 {
                    name.truncate(80);
                    name.push_str("...");
                }
                writeln!(f, "    {}: {}", name, count)?;
            }
            writeln!(f, "")
        }

        write_sorted(f, "Active Parser Types:", self.active_parser_type_counts.clone().into_iter().collect::<Vec<_>>().as_slice())?;
        write_sorted(f, "Active Tags:", self.active_tags.clone().into_iter().collect::<Vec<_>>().as_slice())?;

        if !self.stats_by_tag.is_empty() {
            writeln!(f, "Stats by Tag:")?;
            for (tag, stats_vec) in &self.stats_by_tag {
                for (i, stats) in stats_vec.iter().enumerate() {
                    writeln!(f, "Tag {:?} ({}/{}):", tag, i + 1, stats_vec.len())?;
                    for line in stats.to_string().lines() {
                        writeln!(f, "    {}", line)?;
                    }
                }
            }
        }
        Ok(())
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