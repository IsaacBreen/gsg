use std::collections::HashSet;
use std::iter::FromIterator;
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

fn process(c: u8, its: &mut Vec<CombinatorIterator>) -> ParserIterationResult {
    let mut final_result = ParserIterationResult {
        u8set: U8Set::none(),
        is_complete: false,
    };

    its.retain_mut(|it| {
        if let Some(Ok(result)) = it.borrow_mut().next() {
            final_result = final_result | result;
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
        let mut b_result = ParserIterationResult {
            u8set: U8Set::none(),
            is_complete: false,
        };

        if a_result.is_complete {
            let b_it = b(d.clone());
            b_result = b_it.borrow_mut().next().unwrap().unwrap();
            b_its.push(b_it);
            a_result.is_complete = false;
        }

        let it = std::iter::from_fn(move || {
            if a_its.is_empty() && b_its.is_empty() {
                None
            } else {
                let a_result = process(0, &mut a_its);
                let b_result = process(0, &mut b_its);
                if a_result.is_complete {
                    let b_it = b(d.clone());
                    let new_b_result = b_it.borrow_mut().next().unwrap().unwrap();
                    b_its.push(b_it);
                    Some(Ok(a_result | new_b_result))
                } else {
                    Some(Ok(a_result | b_result))
                }
            }
        });

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

        let it = std::iter::once(Ok(result)).chain(std::iter::from_fn(move || {
            if its.is_empty() {
                None
            } else {
                let result = process(0, &mut its);
                if result.is_complete {
                    let new_a_it = a(d.clone());
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

// Test functions would go here, but they would need to be adapted for Rust's testing framework