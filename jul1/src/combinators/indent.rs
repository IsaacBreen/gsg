// src/combinators/indent.rs
use crate::{dumb_one_shot_parse, BaseCombinatorTrait, RightData, UnambiguousParseError, UnambiguousParseResults};
use std::mem::transmute;
use std::rc::Rc;
use aliasable::boxed::AliasableBox;
use crate::{choice, choice_greedy, CombinatorTrait, eat_byte_choice, eat_bytes, eat_char_choice, eps, mutate_right_data, negative_lookahead, ParseResults, ParserTrait, ParseResultTrait, seq, U8Set, VecX, VecY, IntoDyn};

#[derive(Debug)]
pub enum IndentCombinator {
    Dent,
    Indent,
    Dedent,
    AssertNoDedents,
}

#[derive(Debug)]
pub enum IndentCombinatorParser<'a> {
    DentParser(OwningParser<'a>),
    IndentParser(Option<RightData>),
    Done,
}

#[derive(Debug)]
pub struct OwningParser<'a> {
    combinator: AliasableBox<dyn CombinatorTrait<Parser = Box<dyn ParserTrait>> + 'a>,
    pub(crate) parser: Option<Box<dyn ParserTrait>>,
}

impl<'a> OwningParser<'a> {
    pub fn init(
        combinator: Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>> + 'a>,
        right_data: RightData,
        bytes: &[u8],
    ) -> (OwningParser<'a>, ParseResults) {
        let mut owning_parser = OwningParser {
            combinator: AliasableBox::from_unique(combinator),
            parser: None,
        };

        let (parser, parse_results) = unsafe {
            // Create the parser using the combinator
            let (parser, parse_results) = owning_parser.combinator.parse(right_data, bytes);

            // Transmute the parser's lifetime to 'static
            let parser = transmute(parser);

            // Set the parser in the OwningParser
            owning_parser.parser = Some(parser);

            (owning_parser.parser.as_mut().unwrap(), parse_results)
        };

        (owning_parser, parse_results)
    }
}

impl<'a> ParserTrait for OwningParser<'a> {
    fn get_u8set(&self) -> U8Set {
        self.parser.as_ref().unwrap().get_u8set()
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        self.parser.as_mut().unwrap().parse(bytes)
    }
}

impl<'a> CombinatorTrait for IndentCombinator {
    type Parser = IndentCombinatorParser<'a>;

    fn old_parse(&self, mut right_data: RightData, bytes: &[u8]) -> (Self::Parser, ParseResults) {
        let (parser, parse_results): (IndentCombinatorParser, ParseResults) = match &self {
            IndentCombinator::Dent if right_data.right_data_inner.fields1.dedents == 0 => {
                fn make_combinator<'a>(mut indents: &[Vec<u8>], total_indents: usize)-> Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>> + 'a> {
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
                                    right_data_inner.fields1.dedents = dedents;
                                    // Remove the last `dedents` indents from the indent stack
                                    let new_size = right_data_inner.fields2.indents.len() - dedents as usize;
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
                let combinator: Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>> + 'a> = make_combinator(&right_data.right_data_inner.fields2.indents, right_data.right_data_inner.fields2.indents.len());
                let (parser, parse_results) = OwningParser::init(combinator, right_data, bytes);
                (IndentCombinatorParser::DentParser(parser), parse_results)
            }
            IndentCombinator::Indent if right_data.right_data_inner.fields1.dedents == 0 => {
                if !bytes.is_empty() && bytes[0] != b' ' {
                    (IndentCombinatorParser::Done, ParseResultTrait::new(VecY::new(), true))
                } else {
                    // Consume as many spaces as possible
                    let mut i = 0;
                    while bytes.get(i) == Some(&b' ') {
                        i += 1;
                    }
                    let right_data_inner = right_data.get_inner_mut();
                    right_data_inner.fields1.position += i;
                    right_data_inner.get_fields2_mut().indents.push(bytes[0..i].to_vec());
                    (IndentCombinatorParser::IndentParser(Some(right_data.clone())), ParseResultTrait::new_single(right_data, i < bytes.len()))
                }
            }
            IndentCombinator::Dedent if right_data.right_data_inner.fields1.dedents > 0 => {
                right_data.get_inner_mut().fields1.dedents -= 1;
                // println!("Decremented dedents to {}", right_data.right_data_inner.dedents);
                (IndentCombinatorParser::Done, ParseResultTrait::new_single(right_data, true))
            }
            IndentCombinator::AssertNoDedents if right_data.right_data_inner.fields1.dedents == 0 => {
                (IndentCombinatorParser::Done, ParseResultTrait::new_single(right_data, true))
            }
            _ => (IndentCombinatorParser::Done, ParseResultTrait::empty_finished()),
        };
        (parser, parse_results)
    }

    fn one_shot_parse(&self, mut right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        match &self {
            IndentCombinator::Dent if right_data.right_data_inner.fields1.dedents == 0 => {
                fn make_combinator<'a>(mut indents: &[Vec<u8>], total_indents: usize)-> Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>> + 'a> {
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
                                    right_data_inner.fields1.dedents = dedents;
                                    // Remove the last `dedents` indents from the indent stack
                                    let new_size = right_data_inner.fields2.indents.len() - dedents as usize;
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
                let combinator = make_combinator(&right_data.right_data_inner.fields2.indents, right_data.right_data_inner.fields2.indents.len());
                combinator.one_shot_parse(right_data, bytes)
            }
            IndentCombinator::Indent if right_data.right_data_inner.fields1.dedents == 0 => {
                if !bytes.is_empty() && bytes[0] != b' ' {
                    ParseResultTrait::new(VecY::new(), true)
                } else {
                    // Consume as many spaces as possible
                    let mut i = 0;
                    while bytes.get(i) == Some(&b' ') {
                        i += 1;
                    }
                    let right_data_inner = right_data.get_inner_mut();
                    right_data_inner.fields1.position += i;
                    right_data_inner.get_fields2_mut().indents.push(bytes[0..i].to_vec());
                    ParseResultTrait::new_single(right_data, i < bytes.len())
                }
            }
            IndentCombinator::Dedent if right_data.right_data_inner.fields1.dedents > 0 => {
                right_data.get_inner_mut().fields1.dedents -= 1;
                // println!("Decremented dedents to {}", right_data.right_data_inner.dedents);
                ParseResultTrait::new_single(right_data, true)
            }
            IndentCombinator::AssertNoDedents if right_data.right_data_inner.fields1.dedents == 0 => {
                ParseResultTrait::new_single(right_data, true)
            }
            _ => ParseResultTrait::empty_finished(),
        }
    }
}

impl BaseCombinatorTrait for IndentCombinator {
    fn as_any(&self) -> &dyn std::any::Any {
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
                    right_data_vec.append(&mut parse_results.right_data_vec);
                    done = parse_results.done();
                    if done {
                        break;
                    }
                }
                IndentCombinatorParser::IndentParser(maybe_right_data) => {
                    if byte == b' ' {
                        let mut right_data = maybe_right_data.as_mut().unwrap();
                        let right_data_inner = right_data.get_inner_mut();
                        right_data_inner.fields1.position += 1;
                        right_data_inner.get_fields2_mut().indents.last_mut().unwrap().push(byte);
                        right_data_vec.push(right_data.clone());
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

pub fn with_indent(a: impl CombinatorTrait + 'static)-> impl CombinatorTrait {
    seq!(indent(), a, dedent())
}
//
// impl From<IndentCombinator> for Combinator {
//     fn from(value: IndentCombinator) -> Self {
//         Combinator::IndentCombinator(value)
//     }
// }