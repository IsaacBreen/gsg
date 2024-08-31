use crate::{choice_greedy, eat_bytes, eat_char_choice, eps, mutate_right_data, negative_lookahead, seq, CombinatorTrait, DownData, IntoDyn, ParseResultTrait, ParseResults, ParserTrait, U8Set, UpData, VecY};
// src/combinators/indent.rs
use crate::{BaseCombinatorTrait, DynCombinatorTrait, RightData, RightDataGetters, UnambiguousParseResults};
use aliasable::boxed::AliasableBox;
use std::fmt::Debug;
use std::mem::transmute;

#[derive(Debug)]
pub enum IndentCombinator {
    Dent,
    Indent,
    Dedent,
    AssertNoDedents,
}

#[derive(Debug)]
pub enum IndentCombinatorParser<'a> {
    DentParser(OwningParser<'a, Box<dyn DynCombinatorTrait + 'a>>),
    IndentParser(Option<RightData>),
    Done,
}

#[derive(Debug)]
pub struct OwningParser<'a, T: CombinatorTrait + 'a + ?Sized> {
    combinator: AliasableBox<T>,
    pub(crate) parser: Option<T::Parser<'a>>,
}

impl<'a, T> OwningParser<'a, T> where T: CombinatorTrait {
    pub fn init(
        combinator: T,
        right_data: RightData,
        bytes: &[u8],
    ) -> (OwningParser<'a, T>, ParseResults) {
        let mut owning_parser = OwningParser {
            combinator: AliasableBox::from_unique(Box::new(combinator)),
            parser: None,
        };

        let (parser, parse_results) = unsafe {
            // Create the parser using the combinator
            let (parser, parse_results) = owning_parser.combinator.parse(DownData::new(right_data), bytes);

            // Transmute the parser's lifetime to 'static
            let parser = transmute(parser);

            // Set the parser in the OwningParser
            owning_parser.parser = Some(parser);

            (owning_parser.parser.as_mut().unwrap(), parse_results)
        };

        (owning_parser, parse_results)
    }
}

impl<'a, T> ParserTrait for OwningParser<'a, T> where T: CombinatorTrait {
    fn get_u8set(&self) -> U8Set {
        self.parser.as_ref().unwrap().get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.parser.as_mut().unwrap().parse(bytes)
    }
}

impl DynCombinatorTrait for IndentCombinator {
    fn parse_dyn(&self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl CombinatorTrait for IndentCombinator {
    type Parser<'a> = IndentCombinatorParser<'a>;
    type Output = ();
    type PartialOutput = ();

    fn old_parse(&self, mut down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let (parser, parse_results): (IndentCombinatorParser, ParseResults) = match &self {
            IndentCombinator::Dent if down_data.get_fields1().dedents == 0 => {
                fn make_combinator<'a>(mut indents: &[Vec<u8>], total_indents: usize)-> Box<dyn DynCombinatorTrait + 'a> {
                    if indents.is_empty() {
                        eps().into_dyn()
                    } else {
                        let dedents = indents.len().try_into().unwrap();
                        choice_greedy!(
                            // Exit here and register dedents
                            seq!(
                                negative_lookahead(eat_char_choice(" \n\r")),
                                mutate_right_data(move |right_data: &mut RightData| {
                                    let right_data_inner = right_data.get_inner_mut();
                                    right_data_inner.get_fields1_mut().dedents = dedents;
                                    // Remove the last `dedents` indents from the indent stack
                                    let new_size = right_data_inner.get_fields2().indents.len() - dedents as usize;
                                    right_data_inner.get_fields2_mut().indents.truncate(new_size);
                                    // println!("Registering {} dedents. Right  {:?}", dedents, right_data);
                                    true
                                })
                            ),
                            // Or match the indent and continue
                            seq!(eat_bytes(&indents[0]), make_combinator(&indents[1..], total_indents))
                        ).into_dyn()
                    }
                }
                // println!("Made dent parser with right_data: {:?}", right_data);
                let combinator: Box<dyn DynCombinatorTrait> = make_combinator(&down_data.get_fields2().indents, down_data.get_fields2().indents.len());
                let (parser, parse_results) = OwningParser::init(combinator, down_data.just_right_data(), bytes);
                (IndentCombinatorParser::DentParser(parser), parse_results)
            }
            IndentCombinator::Indent if down_data.get_fields1().dedents == 0 => {
                if !bytes.is_empty() && bytes[0] != b' ' {
                    (IndentCombinatorParser::Done, ParseResultTrait::new(VecY::new(), true))
                } else {
                    // Consume as many spaces as possible
                    let mut i = 0;
                    while bytes.get(i) == Some(&b' ') {
                        i += 1;
                    }
                    down_data.get_fields1_mut().position += i;
                    down_data.get_fields2_mut().indents.push(bytes[0..i].to_vec());
                    (IndentCombinatorParser::IndentParser(Some(down_data.clone().just_right_data())), ParseResultTrait::new_single(UpData::new(down_data.just_right_data().clone()), i < bytes.len()))
                }
            }
            IndentCombinator::Dedent if down_data.get_fields1().dedents > 0 => {
                down_data.get_fields1_mut().dedents -= 1;
                // println!("Decremented dedents to {}", down_data.right_data.right_data_inner.dedents);
                (IndentCombinatorParser::Done, ParseResultTrait::new_single(UpData::new(down_data.clone().just_right_data()), true))
            }
            IndentCombinator::AssertNoDedents if down_data.get_fields1().dedents == 0 => {
                (IndentCombinatorParser::Done, ParseResultTrait::new_single(UpData::new(down_data.clone().just_right_data()), true))
            }
            _ => (IndentCombinatorParser::Done, ParseResultTrait::empty_finished()),
        };
        (parser, parse_results)
    }

    fn one_shot_parse(&self, mut down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        match &self {
            IndentCombinator::Dent if down_data.get_fields1().dedents == 0 => {
                fn make_combinator<'a>(mut indents: &[Vec<u8>], total_indents: usize)-> Box<dyn DynCombinatorTrait + 'a> {
                    if indents.is_empty() {
                        eps().into_dyn()
                    } else {
                        let dedents = indents.len().try_into().unwrap();
                        choice_greedy!(
                            // Exit here and register dedents
                            seq!(
                                negative_lookahead(eat_char_choice(" \n\r")),
                                mutate_right_data(move |right_data: &mut RightData| {
                                    let right_data_inner = right_data.get_inner_mut();
                                    right_data_inner.get_fields1_mut().dedents = dedents;
                                    // Remove the last `dedents` indents from the indent stack
                                    let new_size = right_data_inner.get_fields2().indents.len() - dedents as usize;
                                    right_data_inner.get_fields2_mut().indents.truncate(new_size);
                                    // println!("Registering {} dedents. Right  {:?}", dedents, right_data);
                                    true
                                })
                            ),
                            // Or match the indent and continue
                            seq!(eat_bytes(&indents[0]), make_combinator(&indents[1..], total_indents))
                        ).into_dyn()
                    }
                }
                // println!("Made dent parser with right_data: {:?}", right_data);
                let combinator = make_combinator(&down_data.get_fields2().indents, down_data.get_fields2().indents.len());
                combinator.one_shot_parse(down_data, bytes)
            }
            IndentCombinator::Indent if down_data.get_fields1().dedents == 0 => {
                if !bytes.is_empty() && bytes[0] != b' ' {
                    ParseResultTrait::new(VecY::new(), true)
                } else {
                    // Consume as many spaces as possible
                    let mut i = 0;
                    while bytes.get(i) == Some(&b' ') {
                        i += 1;
                    }
                    down_data.get_fields1_mut().position += i;
                    down_data.get_fields2_mut().indents.push(bytes[0..i].to_vec());
                    ParseResultTrait::new_single(UpData::new(down_data.clone().just_right_data()), i < bytes.len())
                }
            }
            IndentCombinator::Dedent if down_data.get_fields1().dedents > 0 => {
                down_data.get_fields1_mut().dedents -= 1;
                // println!("Decremented dedents to {}", down_data.right_data.right_data_inner.dedents);
                ParseResultTrait::new_single(UpData::new(down_data.just_right_data()), true)
            }
            IndentCombinator::AssertNoDedents if down_data.get_fields1().dedents == 0 => {
                ParseResultTrait::new_single(UpData::new(down_data.just_right_data()), true)
            }
            _ => ParseResultTrait::empty_finished(),
        }
    }
}

impl BaseCombinatorTrait for IndentCombinator {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
}

impl ParserTrait for IndentCombinatorParser<'_> {
    fn get_u8set(&self) -> U8Set {
        match self {
            IndentCombinatorParser::DentParser(parser) => parser.get_u8set(),
            IndentCombinatorParser::IndentParser(_) => U8Set::from_byte(b' '),
            IndentCombinatorParser::Done => U8Set::none(),
        }
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        if bytes.is_empty() {
            return ParseResultTrait::empty_unfinished();
        }

        let mut right_data_vec = VecY::new();
        let mut done = false;

        for &byte in bytes {
            match self {
                IndentCombinatorParser::DentParser(parser) => {
                    // let ParseResults { right_data_vec: mut new_right_data_vec, done: new_done } = parser.parse(&[byte]);
                    let mut parse_results = parser.parse(&[byte]);
                    right_data_vec.append(&mut parse_results.up_data_vec);
                    done = parse_results.done();
                    if done {
                        break;
                    }
                }
                IndentCombinatorParser::IndentParser(maybe_right_data) => {
                    if byte == b' ' {
                        let mut right_data = maybe_right_data.as_mut().unwrap();
                        let right_data_inner = right_data.get_inner_mut();
                        right_data_inner.get_fields1_mut().position += 1;
                        right_data_inner.get_fields2_mut().indents.last_mut().unwrap().push(byte);
                        right_data_vec.push(UpData::new(right_data.clone()));
                    } else {
                        maybe_right_data.take();
                        done = true;
                        break;
                    }
                }
                IndentCombinatorParser::Done => {
                    done = true;
                    break;
                }
            }
        }

        ParseResultTrait::new(right_data_vec, done)
    }
}

pub fn dent() -> IndentCombinator {
    IndentCombinator::Dent
}

pub fn indent() -> IndentCombinator {
    IndentCombinator::Indent
}

pub fn dedent() -> IndentCombinator {
    IndentCombinator::Dedent
}

pub fn assert_no_dedents() -> IndentCombinator {
    IndentCombinator::AssertNoDedents
}

pub fn with_indent(a: impl CombinatorTrait)-> impl CombinatorTrait {
    seq!(indent(), a, dedent())
}
