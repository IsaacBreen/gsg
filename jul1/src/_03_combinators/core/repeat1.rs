
// src/_03_combinators/core/repeat1.rs
// src/combinators/repeat1.rs
use crate::{seq, BaseCombinatorTrait, DynCombinatorTrait, UnambiguousParseError, UnambiguousParseResults, RightData, UpData, OneShotUpData};

use crate::_01_parse_state::{ParseResultTrait, RightDataGetters};
use crate::{profile_internal, vecy, CombinatorTrait, IntoCombinator, Opt, ParseResults, ParserTrait, Squash, U8Set, VecY};

#[derive(Debug)]
pub struct Repeat1<T> {
    pub(crate) a: T,
    pub(crate) greedy: bool,
    // pub(crate) _phantom: std::marker::PhantomData<&'a T>,
}

#[derive(Debug)]
pub struct Repeat1Parser<'a, T> where T: CombinatorTrait {
    // TODO: store a_parsers in a Vec<Vec<Parser>> where the index of each inner vec is the repetition count of those parsers. That way, we can easily discard earlier parsers when we get a decisively successful parse result.
    pub(crate) a: &'a T,
    pub(crate) a_parsers: Vec<T::Parser<'a>>,
    pub(crate) position: usize,
    pub(crate) greedy: bool,
}

impl<T: CombinatorTrait> DynCombinatorTrait for Repeat1<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<'b, T: CombinatorTrait > CombinatorTrait for Repeat1<T> {
    type Parser<'a> = Repeat1Parser<'a, T> where Self: 'a;
    type Output = Vec<T::Output>;
    type PartialOutput = Vec<T::PartialOutput>;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let mut right_data = right_data;
        let start_position = right_data.get_fields1().position;
        let mut prev_parse_result = Err(UnambiguousParseError::Fail);
        loop {
            let offset = right_data.get_fields1().position - start_position;
            let parse_result = self.a.one_shot_parse(right_data.clone(), &bytes[offset..]);
            match parse_result {
                Ok(one_shot_up_data) => {
                    if !self.greedy && prev_parse_result.is_ok() {
                        return Err(UnambiguousParseError::Ambiguous);
                    }
                    right_data = one_shot_up_data.clone().just_right_data();
                    prev_parse_result = Ok(one_shot_up_data);
                }
                Err(UnambiguousParseError::Fail) => {
                    return prev_parse_result;
                }
                Err(UnambiguousParseError::Ambiguous | UnambiguousParseError::Incomplete) => {
                    return parse_result;
                }
            }
        }
    }

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        // return self.old_parse(right_data, bytes);
        let mut right_data = right_data;
        let start_position = right_data.get_fields1().position;
        let mut prev_parse_result = Err(UnambiguousParseError::Fail);
        loop {
            let offset = right_data.get_fields1().position - start_position;
            let parse_result = self.a.one_shot_parse(right_data.clone(), &bytes[offset..]);
            match parse_result {
                Ok(one_shot_up_data) => {
                    if !self.greedy && prev_parse_result.is_ok() {
                        let (parser, mut parse_results_rest) = self.old_parse(right_data.clone(), &bytes[offset..]);
                        let prev_right_data = parse_results_rest.up_data_vec.pop().unwrap().just_right_data();
                        parse_results_rest.up_data_vec.push(UpData::new(prev_right_data));
                        return (parser, parse_results_rest);
                    }
                    right_data = one_shot_up_data.clone().just_right_data();
                    prev_parse_result = Ok(one_shot_up_data);
                }
                Err(UnambiguousParseError::Fail) => {
                    if let Ok(one_shot_up_data) = prev_parse_result {
                        return (Repeat1Parser {
                            a: &self.a,
                            a_parsers: vec![],
                            position: start_position + bytes.len(),
                            greedy: self.greedy
                        }, ParseResults::new_single(UpData::new(one_shot_up_data.just_right_data()), true));
                    } else {
                        return (Repeat1Parser {
                            a: &self.a,
                            a_parsers: vec![],
                            position: start_position + bytes.len(),
                            greedy: self.greedy
                        }, ParseResults::empty_finished());
                    }
                }
                Err(UnambiguousParseError::Ambiguous) => {
                    let (parser, mut parse_results_rest) = self.old_parse(right_data, &bytes[offset..]);
                    if let Ok(one_shot_up_data) = prev_parse_result {
                        parse_results_rest.up_data_vec.push(UpData::new(one_shot_up_data.just_right_data()));
                    }
                    return (parser, parse_results_rest);
                }
                Err(UnambiguousParseError::Incomplete) => {
                    let (parser, mut parse_results_rest) = self.old_parse(right_data, &bytes[offset..]);
                    if let Ok(one_shot_up_data) = prev_parse_result {
                        parse_results_rest.up_data_vec.push(UpData::new(one_shot_up_data.just_right_data()));
                    }
                    return (parser, parse_results_rest);
                }
            }
        }
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let start_position = right_data.get_fields1().position;
        let (parser, parse_results) = self.a.parse(right_data, bytes);
        if parse_results.done() && parse_results.up_data_vec.is_empty() {
            // Shortcut
            return (Repeat1Parser {
                a: &self.a,
                a_parsers: vec![],
                position: start_position + bytes.len(),
                greedy: self.greedy
            }, parse_results);
        } else if parse_results.up_data_vec.is_empty() {
            return (Repeat1Parser {
                a: &self.a,
                a_parsers: vec![parser],
                position: start_position + bytes.len(),
                greedy: self.greedy
            }, ParseResults::new(vecy![], false));
        }
        let mut parsers = if parse_results.done() {
            vec![]
        } else {
            vec![parser]
        };
        let mut all_prev_succeeded_decisively = parse_results.succeeds_decisively();
        let mut up_data_vec = parse_results.up_data_vec;

        let mut next_up_data = up_data_vec.clone();
        while next_up_data.len() > 0 {
            for up_data in std::mem::take(&mut next_up_data) {
                #[cfg(feature = "debug")]
                let new_right_data2 = new_right_data.clone();
                let offset = up_data.get_fields1().position - start_position;
                let (parser, parse_results) = self.a.parse(up_data.just_right_data(), &bytes[offset..]);
                if !parse_results.done() {
                    parsers.push(parser);
                }
                all_prev_succeeded_decisively &= parse_results.succeeds_decisively();
                if self.greedy && all_prev_succeeded_decisively {
                    up_data_vec.clear();
                    parsers.clear();
                }
                // if !(self.greedy && parse_results.succeeds_decisively()) && parse_results.up_data_vec.len() > 0 && up_data_vec.len() > 0 {
                //     println!("parse_results: {:?}", parse_results);
                // }
                #[cfg(feature = "debug")]
                for UpData { right_data } in &parse_results.up_data_vec {
                    if &new_right_data2 == right_data {
                        panic!("Repeat1Parser::old_parse: loop detected. new_right_data == right_data. This can happen if you repeat a parser that can match the empty string?");
                    }
                }
                next_up_data.extend(parse_results.up_data_vec);
            }
            if !up_data_vec.is_empty() && !next_up_data.is_empty() {
                let end_pos = start_position + bytes.len();
                let pos1 = up_data_vec[0].get_fields1().position;
                let pos2 = next_up_data[0].get_fields1().position;
                if end_pos < pos1 + 1000 || end_pos < pos2 + 1000 {
                    up_data_vec.clear();
                }
            }
            up_data_vec.extend(next_up_data.clone());
        }

        let done = parsers.is_empty();

        (
            Repeat1Parser {
                a: &self.a,
                a_parsers: parsers,
                position: start_position + bytes.len(),
                greedy: self.greedy
            },
            ParseResults::new(up_data_vec, done)
        )
    }
}

impl<'a, T: BaseCombinatorTrait> BaseCombinatorTrait for Repeat1<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(&self.a);
    }
}

impl<'a, T> ParserTrait for Repeat1Parser<'a, T> where T: CombinatorTrait {
    fn get_u8set(&self) -> U8Set {
        if self.a_parsers.is_empty() {
            U8Set::none()
        } else {
            let mut u8set = U8Set::none();
            for p in &self.a_parsers {
                u8set = u8set | p.get_u8set();
            }
            u8set
        }
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut up_data_as = VecY::new();
        // let mut up_data_as: BTreeMap<usize, RightData, RightDataGettersSquasher> = BTreeMap::new();

        self.a_parsers.retain_mut(|mut a_parser| {
            let parse_results = a_parser.parse(bytes);
            // up_data_as.entry(parse_results.up_data_vec.len()).or_default().extend(parse_results.up_data_vec);
            up_data_as.extend(parse_results.up_data_vec);
            !parse_results.done
        });

        up_data_as.squash();

        let mut i = 0;
        while i < up_data_as.len() {
            let up_data_a = up_data_as[i].clone();
            let offset = up_data_a.get_fields1().position - self.position;
            let (a_parser, parse_results) = self.a.parse(up_data_a.just_right_data(), &bytes[offset..]);
            if !parse_results.done() {
                self.a_parsers.push(a_parser);
            }
            // up_data_as.entry(i).or_default().extend(parse_results.up_data_vec);
            up_data_as.extend(parse_results.up_data_vec);
            i += 1;
        }

        self.position += bytes.len();

        ParseResults::new(up_data_as, self.a_parsers.is_empty())
    }
}

pub fn Repeat1<T: IntoCombinator>(a: T)-> impl CombinatorTrait {
    // profile_internal("repeat1", Repeat1 {
    //     a: a.into_combinator(),
    //     greedy: false,
    // })
    Repeat1 {
        a: a.into_combinator(),
        greedy: false,
        // _phantom: std::marker::PhantomData,
    }
}

pub fn repeat1<T: IntoCombinator>(a: T)-> impl CombinatorTrait {
    Repeat1 {
        a: a.into_combinator(),
        greedy: false,
    }
}

pub fn repeat1_greedy<T: IntoCombinator>(a: T)-> impl CombinatorTrait {
    profile_internal("repeat1_greedy", Repeat1 {
        a: a.into_combinator(),
        greedy: true,
        // _phantom: std::marker::PhantomData,
    })
}

pub fn repeat0<T: IntoCombinator>(a: T)-> impl CombinatorTrait {
    Opt { inner: Repeat1 { a: a.into_combinator(), greedy: false }, greedy: false }
}

pub fn repeat0_greedy<T: IntoCombinator>(a: T)-> impl CombinatorTrait {
    Opt { inner: Repeat1 { a: a.into_combinator(), greedy: true }, greedy: true }
}

pub fn seprep1<T: IntoCombinator + Clone, U: IntoCombinator>(a: T, b: U)-> impl CombinatorTrait {
    seq!(a.clone(), repeat0(seq!(b, a)))
}