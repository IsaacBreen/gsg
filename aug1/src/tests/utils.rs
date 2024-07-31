use crate::{Combinator, CombinatorTrait, ParseResults, ParserTrait, ParseState};

pub fn assert_parses(combinator: impl Into<Combinator>, input: &str) {
    let combinator: Combinator = combinator.into();
    let ParseResults { parsers: mut parsers, mut continuations, mut states } = combinator.init_parser(ParseState::default());
    for c in input.bytes() {
        let mut new_parsers = vec![];
        for parser in parsers.drain(..) {
            let parse_results = parser.step(c);
            new_parsers.extend(parse_results.parsers);
            states.extend(parse_results.states);
        }
        parsers = new_parsers;
    }
    println!("{:?}", states);
    println!("{:?}", continuations);
    println!("{:?}", parsers);
    assert_eq!(states, vec![ParseState::default()]);
    assert_eq!(continuations, vec![]);
    assert_eq!(parsers, vec![]);
}