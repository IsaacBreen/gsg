use std::collections::{BTreeSet, HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Symbol {
    Terminal(char),
    NonTerminal(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Production {
    lhs: Symbol,
    rhs: Vec<Symbol>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Item {
    production: Production,
    dot_position: usize,
}

fn closure(items: &BTreeSet<Item>, productions: &[Production]) -> BTreeSet<Item> {
    todo!()
}

fn goto(items: &BTreeSet<Item>, symbol: &Symbol, productions: &[Production]) -> BTreeSet<Item> {
        todo!()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Action {
    Shift(usize),
    Reduce(usize),
    Accept,
}

fn generate_parse_table(productions: &[Production], start_symbol: &str) -> HashMap<BTreeSet<Item>, HashMap<Symbol, BTreeSet<(Action, BTreeSet<Item>)>>> {
        todo!()
}

fn parse(input: &str, parse_table: &HashMap<BTreeSet<Item>, HashMap<Symbol, BTreeSet<(Action, BTreeSet<Item>)>>>, productions: &[Production], start_symbol: &str) -> Result<(), String> {
    todo!()
}

fn nt(name: &str) -> Symbol {
    Symbol::NonTerminal(name.to_string())
}

fn term(c: char) -> Symbol {
    Symbol::Terminal(c)
}

#[cfg(test)]
mod glalr_tests {
    use super::*;

    #[test]
    fn test_parse_simple_expression() {
        let productions = vec![

        ];

        let parse_table = generate_parse_table(&productions, "S");

        assert!(parse("i+i*i", &parse_table, &productions, "S").is_ok());
        assert!(parse("i+i", &parse_table, &productions, "S").is_ok());
        assert!(parse("i*i", &parse_table, &productions, "S").is_ok());
        assert!(parse("i", &parse_table, &productions, "S").is_ok());
        assert!(parse("(i+i)*i", &parse_table, &productions, "S").is_ok());

        assert!(parse("i+", &parse_table, &productions, "S").is_err());
        assert!(parse("i++i", &parse_table, &productions, "S").is_err());
        assert!(parse("", &parse_table, &productions, "S").is_err());
        assert!(parse(")", &parse_table, &productions, "S").is_err());
    }
}