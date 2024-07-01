use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
struct U8Set(HashSet<u8>);

impl U8Set {
    fn none() -> Self {
        U8Set(HashSet::new())
    }

    fn from_chars(chars: &str) -> Self {
        U8Set(HashSet::from_iter(chars.bytes()))
    }
}

impl std::ops::BitOr for U8Set {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        U8Set(self.0.union(&other.0).cloned().collect())
    }
}

impl std::ops::BitAnd for U8Set {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        U8Set(self.0.intersection(&other.0).cloned().collect())
    }
}

#[derive(Clone, Debug)]
struct ParserIterationResult {
    u8set: U8Set,
    is_complete: bool,
}

impl std::ops::BitOr for ParserIterationResult {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        ParserIterationResult {
            u8set: self.u8set | other.u8set,
            is_complete: self.is_complete | other.is_complete,
        }
    }
}

impl std::ops::BitAnd for ParserIterationResult {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        ParserIterationResult {
            u8set: self.u8set & other.u8set,
            is_complete: self.is_complete & other.is_complete,
        }
    }
}

type Data = ();
type Combinator = Rc<dyn Fn(Data) -> CombinatorIterator>;
type CombinatorIterator = Rc<RefCell<dyn Iterator<Item = Result<ParserIterationResult, u8>>>>;

fn process(_c: u8, its: &mut Vec<CombinatorIterator>) -> ParserIterationResult {
    let mut final_result = ParserIterationResult {
        u8set: U8Set::none(),
        is_complete: false,
    };

    its.retain_mut(|it| {
        if let Some(Ok(result)) = it.borrow_mut().next() {
            final_result = final_result.clone() | result;
            true
        } else {
            false
        }
    });

    final_result
}

fn seq2(a: Combinator, b: Combinator) -> Combinator {
    Rc::new(move |d: Data| {
        let a_it = a(d.clone());
        let mut a_its = vec![a_it.clone()];
        let mut b_its = vec![];

        let mut a_result = a_it.borrow_mut().next().unwrap().unwrap();

        if a_result.is_complete {
            let b_it = b(d.clone());
            let b_result = b_it.borrow_mut().next().unwrap().unwrap();
            b_its.push(b_it);
            a_result.is_complete = false;
            a_result = a_result | b_result;
        }

        // Remove the unused a_clone
        let b_clone = b.clone();
        let d_clone = d.clone();

        let it = std::iter::once(Ok(a_result)).chain(std::iter::from_fn(move || {
            if a_its.is_empty() && b_its.is_empty() {
                None
            } else {
                let a_result = process(0, &mut a_its);
                let b_result = process(0, &mut b_its);
                if a_result.is_complete {
                    let b_it = b_clone(d_clone.clone());
                    let new_b_result = b_it.borrow_mut().next().unwrap().unwrap();
                    b_its.push(b_it);
                    Some(Ok(a_result | new_b_result))
                } else {
                    Some(Ok(a_result | b_result))
                }
            }
        }));

        Rc::new(RefCell::new(it))
    })
}

fn seq(parsers: Vec<Combinator>) -> Combinator {
    parsers.into_iter().reduce(seq2).unwrap()
}

fn choice(parsers: Vec<Combinator>) -> Combinator {
    Rc::new(move |d: Data| {
        let active_parsers: Vec<CombinatorIterator> = parsers.iter().map(|p| p(d.clone())).collect();
        let first_results: Vec<ParserIterationResult> = active_parsers
            .iter()
            .map(|p| p.borrow_mut().next().unwrap().unwrap())
            .collect();

        let initial_result = first_results.into_iter().reduce(|a, b| a | b).unwrap();
        let mut active_parsers = active_parsers;

        let it = std::iter::once(Ok(initial_result)).chain(std::iter::from_fn(move || {
            if active_parsers.is_empty() {
                None
            } else {
                Some(Ok(process(0, &mut active_parsers)))
            }
        }));

        Rc::new(RefCell::new(it))
    })
}

fn eat_u8(value: u8) -> Combinator {
    Rc::new(move |_: Data| {
        let value = value;
        let it = vec![
            Ok(ParserIterationResult {
                u8set: U8Set::from_chars(&std::str::from_utf8(&[value]).unwrap()),
                is_complete: false,
            }),
            Ok(ParserIterationResult {
                u8set: U8Set::none(),
                is_complete: true,
            }),
        ].into_iter();

        Rc::new(RefCell::new(it))
    })
}

fn eat_u8_range(start: u8, end: u8) -> Combinator {
    Rc::new(move |_: Data| {
        let chars: String = (start..=end).map(|c| c as char).collect();
        let it = vec![
            Ok(ParserIterationResult {
                u8set: U8Set::from_chars(&chars),
                is_complete: false,
            }),
            Ok(ParserIterationResult {
                u8set: U8Set::none(),
                is_complete: true,
            }),
        ].into_iter();

        Rc::new(RefCell::new(it))
    })
}

fn eat_u8_range_complement(start: u8, end: u8) -> Combinator {
    Rc::new(move |_: Data| {
        let chars: String = (0..start).chain((end + 1)..=255).map(|c| c as char).collect();
        let it = vec![
            Ok(ParserIterationResult {
                u8set: U8Set::from_chars(&chars),
                is_complete: false,
            }),
            Ok(ParserIterationResult {
                u8set: U8Set::none(),
                is_complete: true,
            }),
        ].into_iter();

        Rc::new(RefCell::new(it))
    })
}

fn eat_string(value: &str) -> Combinator {
    seq(value.bytes().map(eat_u8).collect())
}

fn repeat(a: Combinator) -> Combinator {
    Rc::new(move |d: Data| {
        let a_it = a(d.clone());
        let mut result = a_it.borrow_mut().next().unwrap().unwrap();
        result.is_complete = true;
        let mut its = vec![a_it];

        let a_clone = a.clone();
        let d_clone = d.clone();

        let it = std::iter::once(Ok(result)).chain(std::iter::from_fn(move || {
            if its.is_empty() {
                None
            } else {
                let result = process(0, &mut its);
                if result.is_complete {
                    let new_a_it = a_clone(d_clone.clone());
                    let new_result = new_a_it.borrow_mut().next().unwrap().unwrap();
                    its.push(new_a_it);
                    Some(Ok(result | new_result))
                } else {
                    Some(Ok(result))
                }
            }
        }));

        Rc::new(RefCell::new(it))
    })
}

fn eps() -> Combinator {
    Rc::new(|_: Data| {
        Rc::new(RefCell::new(std::iter::once(Ok(ParserIterationResult {
            u8set: U8Set::none(),
            is_complete: true,
        }))))
    })
}

fn opt(a: Combinator) -> Combinator {
    choice(vec![a, eps()])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_parser(parser: Combinator, input: &str) -> bool {
        let it = parser(());  // Remove 'mut'
        let mut result = it.borrow_mut().next().unwrap().unwrap();

        for c in input.bytes() {
            if !result.u8set.0.contains(&c) {
                return false;
            }
            result = it.borrow_mut().next().unwrap().unwrap();
        }

        result.is_complete
    }

    #[test]
    fn test_eat_u8() {
        let parser = eat_u8(b'a');
        assert!(run_parser(parser.clone(), "a"));
        assert!(!run_parser(parser, "b"));
    }

    #[test]
    fn test_eat_u8_range() {
        let parser = eat_u8_range(b'0', b'9');
        assert!(run_parser(parser.clone(), "5"));
        assert!(!run_parser(parser.clone(), "a"));
    }

    #[test]
    fn test_eat_u8_range_complement() {
        let parser = eat_u8_range_complement(b'0', b'9');
        assert!(run_parser(parser.clone(), "a"));
        assert!(!run_parser(parser.clone(), "5"));
    }

    #[test]
    fn test_eat_string() {
        let parser = eat_string("hello");
        assert!(run_parser(parser.clone(), "hello"));
        assert!(!run_parser(parser.clone(), "world"));
    }

    #[test]
    fn test_seq() {
        let parser = seq(vec![eat_u8(b'a'), eat_u8(b'b'), eat_u8(b'c')]);
        assert!(run_parser(parser.clone(), "abc"));
        assert!(!run_parser(parser.clone(), "acb"));
    }

    #[test]
    fn test_choice() {
        let parser = choice(vec![eat_string("foo"), eat_string("bar")]);
        assert!(run_parser(parser.clone(), "foo"));
        assert!(run_parser(parser.clone(), "bar"));
        assert!(!run_parser(parser.clone(), "baz"));
    }

    #[test]
    fn test_repeat() {
        let parser = repeat(eat_u8(b'a'));
        assert!(run_parser(parser.clone(), ""));
        assert!(run_parser(parser.clone(), "a"));
        assert!(run_parser(parser.clone(), "aaa"));
        assert!(!run_parser(parser.clone(), "ab"));
    }

    #[test]
    fn test_opt() {
        let parser = seq(vec![
            eat_u8(b'a'),
            opt(eat_u8(b'b')),
            eat_u8(b'c'),
        ]);
        assert!(run_parser(parser.clone(), "abc"));
        assert!(run_parser(parser.clone(), "ac"));
        assert!(!run_parser(parser.clone(), "ab"));
    }

    #[test]
    fn test_eps() {
        let parser = eps();
        assert!(run_parser(parser.clone(), ""));
        assert!(!run_parser(parser.clone(), "a"));
    }

    // More complex parsing scenarios

    #[test]
    fn test_parse_number() {
        let digit = eat_u8_range(b'0', b'9');
        let number = seq(vec![
            opt(choice(vec![eat_u8(b'+'), eat_u8(b'-')])),
            repeat(digit),
        ]);
        assert!(run_parser(number.clone(), "123"));
        assert!(run_parser(number.clone(), "+456"));
        assert!(run_parser(number.clone(), "-789"));
        assert!(!run_parser(number.clone(), "12a"));
    }

    #[test]
    fn test_parse_identifier() {
        let letter = choice(vec![
            eat_u8_range(b'a', b'z'),
            eat_u8_range(b'A', b'Z'),
        ]);
        let digit = eat_u8_range(b'0', b'9');
        let identifier = seq(vec![
            letter.clone(),
            repeat(choice(vec![letter.clone(), digit])),
        ]);
        assert!(run_parser(identifier.clone(), "abc"));
        assert!(run_parser(identifier.clone(), "abc123"));
        assert!(run_parser(identifier.clone(), "A_b_C"));
        assert!(!run_parser(identifier.clone(), "123abc"));
    }

    #[test]
    fn test_parse_json_like() {
        let whitespace = repeat(choice(vec![
            eat_u8(b' '),
            eat_u8(b'\t'),
            eat_u8(b'\n'),
            eat_u8(b'\r'),
        ]));

        let digit = eat_u8_range(b'0', b'9');
        let number = seq(vec![
            opt(eat_u8(b'-')),
            choice(vec![
                eat_u8(b'0'),
                seq(vec![eat_u8_range(b'1', b'9'), repeat(digit.clone())]),
            ]),
            opt(seq(vec![
                eat_u8(b'.'),
                repeat(digit),
            ])),
        ]);

        let string_char = choice(vec![
            eat_u8_range_complement(b'"', b'"'),
            seq(vec![eat_u8(b'\\'), eat_u8(b'"')]),
        ]);
        let string = seq(vec![
            eat_u8(b'"'),
            repeat(string_char),
            eat_u8(b'"'),
        ]);

        let json_value = Rc::new(move |d| {
            choice(vec![
                string.clone(),
                number.clone(),
                eat_string("true"),
                eat_string("false"),
                eat_string("null"),
            ])(d)
        });

        let json_array = {
            let json_value = json_value.clone();  // Clone before moving into the closure
            Rc::new(move |d| {
                seq(vec![
                    eat_u8(b'['),
                    whitespace.clone(),
                    opt(seq(vec![
                        json_value.clone(),
                        repeat(seq(vec![
                            whitespace.clone(),
                            eat_u8(b','),
                            whitespace.clone(),
                            json_value.clone(),
                        ])),
                    ])),
                    whitespace.clone(),
                    eat_u8(b']'),
                ])(d)
            })
        };

        assert!(run_parser(json_value.clone(), "\"hello\""));
        assert!(run_parser(json_value.clone(), "42"));
        assert!(run_parser(json_value.clone(), "-3.14"));
        assert!(run_parser(json_value.clone(), "true"));
        assert!(run_parser(json_value.clone(), "false"));
        assert!(run_parser(json_value.clone(), "null"));

        assert!(run_parser(json_array.clone(), "[]"));
        assert!(run_parser(json_array.clone(), "[1, 2, 3]"));
        assert!(run_parser(json_array.clone(), "[\"a\", true, null]"));
        assert!(!run_parser(json_array.clone(), "[1, 2,]"));
    }
}
