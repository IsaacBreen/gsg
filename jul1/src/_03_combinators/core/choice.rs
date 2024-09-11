use crate::_01_parse_state::{ParseResultTrait, RightData, RightDataGetters, UpData};
use crate::{profile_internal, BaseCombinatorTrait, CombinatorTrait, DynCombinatorTrait, ParseResults, ParserTrait, U8Set, UnambiguousParseError, UnambiguousParseResults, VecX, OneShotUpData};

#[derive(Debug)]
pub struct Choice<'a, Output: 'a> {
    pub(crate) children: VecX<Box<dyn DynCombinatorTrait<Output = Output> + 'a>>,
    pub(crate) greedy: bool,
}

#[derive(Debug)]
pub struct ChoiceParser<'a, Output: 'a> {
    pub(crate) parsers: Vec<Box<dyn ParserTrait<Output = Output> + 'a>>,
    pub(crate) greedy: bool,
}

impl<Output: 'static> DynCombinatorTrait<Output = Output> for Choice<'_, Output> {
    fn parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> (Box<dyn ParserTrait<Output = Output> + 'a>, ParseResults<Output>) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &'a [u8]) -> UnambiguousParseResults<Output> {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<Output: 'static> CombinatorTrait<Output = Output> for Choice<'_, Output> {
    type Parser<'a> = ChoiceParser<'a, Output> where Self: 'a;

    fn one_shot_parse<'b>(&self, right_data: RightData, bytes: &'b [u8]) -> UnambiguousParseResults<Output> where Output: 'b {
        if self.greedy {
            for parser in self.children.iter() {
                let parse_result = parser.one_shot_parse(right_data.clone(), bytes);
                match parse_result {
                    Ok(one_shot_up_data) => {
                        return Ok(OneShotUpData::new(one_shot_up_data.just_right_data(), one_shot_up_data.output));
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
                    Ok(one_shot_up_data) => {
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
                        return Ok(OneShotUpData::new(one_shot_up_data.just_right_data(), one_shot_up_data.output));
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

    fn parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Output>) where Output: 'b {
        let mut parsers = Vec::new();
        let mut combined_results = ParseResults::empty_finished();

        for child in &self.children {
            let (parser, parse_results) = child.as_ref().parse_dyn(right_data.clone(), bytes);
            if !parse_results.done {
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

    fn old_parse<'a, 'b>(&'a self, right_data: RightData, bytes: &'b [u8]) -> (Self::Parser<'a>, ParseResults<Output>) where Output: 'b {
        self.parse(right_data, bytes)
    }
}

impl<Output: 'static> BaseCombinatorTrait for Choice<'_, Output> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        for child in self.children.iter() {
            f(child);
        }
    }
}

impl<Output: 'static> ParserTrait<Output = Output> for ChoiceParser<'_, Output> {
    fn get_u8set(&self) -> U8Set {
        let mut u8set = U8Set::none();
        for parser in &self.parsers {
            u8set |= parser.get_u8set();
        }
        u8set
    }

    fn parse<'b>(&mut self, bytes: &'b [u8]) -> ParseResults<Output> where Output: 'b {
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

pub fn _choice<'a, Output: 'static>(v: Vec<Box<dyn DynCombinatorTrait<Output = Output> + 'a>>) -> impl CombinatorTrait<Output = Output> + 'a {
    Choice {
        children: v.into_iter().collect(),
        greedy: false,
    }
}

pub fn _choice_greedy<'a, Output: 'static>(v: Vec<Box<dyn DynCombinatorTrait<Output = Output> + 'a>>) -> impl CombinatorTrait<Output = Output> + 'a {
    profile_internal("choice", Choice {
        children: v.into_iter().collect(),
        greedy: true,
    })
}