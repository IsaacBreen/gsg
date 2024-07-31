use crate::{Combinator, CombinatorTrait, ParseResults, ParserTrait, ParseState};

pub fn assert_parses(combinator: &(impl Into<Combinator> + Clone), bytes: &[u8]) {
    let combinator: Combinator = combinator.clone().into();
    let ParseResults { continuations: mut continuations, waiting_continuations: mut waiting_continuations, mut states } = combinator.init_parser(ParseState::default());
    for c in bytes {
        let mut new_continuations = vec![];
        let mut new_waiting_continuations = vec![];
        for continuation in continuations.drain(..) {
            let mut parse_results = continuation.head.step(*c);
            parse_results.extend_tail(&continuation.tail);
            new_continuations.extend(parse_results.continuations);
            new_waiting_continuations.extend(parse_results.waiting_continuations);
            states.extend(parse_results.states);
        }
        while let Some(waiting_continuation) = new_waiting_continuations.pop() {
            let mut parse_results = waiting_continuation.head.init_parser(waiting_continuation.state);
            parse_results.extend_tail(&waiting_continuation.tail);
            new_continuations.extend(parse_results.continuations);
            new_waiting_continuations.extend(parse_results.waiting_continuations);
            states.extend(parse_results.states);
        }
        continuations = new_continuations;
    }
    println!("{:?}", states);
    println!("{:?}", waiting_continuations);
    println!("{:?}", continuations);
    assert_eq!(states, vec![ParseState::default()]);
    assert_eq!(waiting_continuations, vec![]);
    assert_eq!(continuations, vec![]);
}