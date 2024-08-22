// src/combinators/choice.rs
// src/combinators/choice.rs
use std::any::Any;
use std::rc::Rc;

use crate::{CombinatorTrait, eps, ParseResults, ParserTrait, profile_internal, Squash, U8Set, VecX, UnambiguousParseResults, UnambiguousParseError, BaseCombinatorTrait};
use crate::parse_state::{RightData, ParseResultTrait};

#[derive(Debug)]
pub struct Choice {
    pub(crate) children: VecX<Box<dyn CombinatorTrait<Parser = Box<dyn ParserTrait>>>>,
    pub(crate) greedy: bool,
}

#[derive(Debug)]
pub struct ChoiceParser<'a> {
    pub(crate) parsers: Vec<Box<dyn ParserTrait + 'a>>,
    pub(crate) greedy: bool,
}

impl CombinatorTrait for Choice {
    type Parser = ChoiceParser<'a>;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        if self.greedy {
            for parser in self.children.iter() {
                let parse_result = parser.one_shot_parse(right_data.clone(), bytes);
                match parse_result {
                    Ok(right_data) => {
                        return Ok(right_data);
                    }
                    Err(UnambiguousParseError::Incomplete | UnambiguousParseError::Ambiguous) => {
                        return parse_result;
                    }
                    Err(UnambiguousParseError::Fail) => {}
                }
            }
            Err(UnambiguousParseError::Fail)
        } else {
            for (i, parser) in self.children.iter().enumerate() {
                let parse_result = parser.one_shot_parse(right_data.clone(), bytes);
                match parse_result {
                    Ok(right_data) => {
                        for parser2 in self.children[i+1..].iter() {
                            let parse_result2 = parser2.one_shot_parse(right_data.clone(), bytes);
                            match parse_result2 {
                                Ok(_) => {
                                    return Err(UnambiguousParseError::Ambiguous);
                                }
                                Err(UnambiguousParseError::Incomplete | UnambiguousParseError::Ambiguous) => {
                                    return parse_result2;
                                }
                                Err(UnambiguousParseError::Fail) => {}
                            }
                        }
                        return Ok(right_data);
                    }
                    Err(UnambiguousParseError::Incomplete | UnambiguousParseError::Ambiguous) => {
                        return parse_result;
                    }
                    Err(UnambiguousParseError::Fail) => {}
                }
            };
            Err(UnambiguousParseError::Fail)
        }
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let mut parsers = Vec::new();
        let mut combined_results = ParseResults::empty_finished();

        for child in self.children.iter() {
            let (parser, parse_results) = child.parse(right_data.clone(), bytes);
            if !parse_results.done() {
                parsers.push(parser);
            }
            let discard_rest = self.greedy && parse_results.succeeds_decisively();
            combined_results.merge_assign(parse_results);
            if discard_rest {
                break;
            }
        }

        (
            ChoiceParser { parsers, greedy: self.greedy },
            combined_results
        )
    }
}

impl BaseCombinatorTrait for Choice {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        for child in self.children.iter() {
            f(child);
        }
    }
}

impl ParserTrait for ChoiceParser<'_> {
    fn get_u8set(&self) -> U8Set {
        let mut u8set = U8Set::none();
        for parser in &self.parsers {
            u8set |= parser.get_u8set();
        }
        u8set
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut parse_result = ParseResults::empty_finished();
        let mut discard_rest = false;

        self.parsers.retain_mut(|mut parser| {
            if discard_rest {
                return false;
            }
            let parse_results = parser.parse(bytes);
            discard_rest = self.greedy && parse_results.succeeds_decisively();
            let done = parse_results.done();
            parse_result.merge_assign(parse_results);
            !done
        });
        parse_result
    }

}

pub fn _choice(v: Vec<Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>>>) -> impl CombinatorTrait {
    Choice {
        children: v.into_iter().collect(),
        greedy: false,
    }
}

pub fn _choice_greedy(v: Vec<Box<dyn CombinatorTrait<Parser=Box<dyn ParserTrait>>>>) -> impl CombinatorTrait {
    profile_internal("choice", Choice {
        children: v.into_iter().collect(),
        greedy: true,
    })
}

// #[macro_export]
// macro_rules! choice {
//     ($($expr:expr),+ $(,)?) => {
//         $crate::_choice(vec![$($expr.into()),+])
//     };
// }
//
// #[macro_export]
// macro_rules! choice_greedy {
//     ($($expr:expr),+ $(,)?) => {
//         $crate::_choice_greedy(vec![$($expr.into()),+])
//     };
// }

// impl From<Choice> for Combinator {
//     fn from(value: Choice) -> Self {
//
//     }
//