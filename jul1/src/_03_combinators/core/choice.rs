use crate::_01_parse_state::{ParseResultTrait, RightData, RightDataGetters, UpData};
use crate::{profile_internal, BaseCombinatorTrait, CombinatorTrait, DynCombinatorTrait, ParseResults, ParserTrait, U8Set, UnambiguousParseError, UnambiguousParseResults, VecX, DownData, OneShotUpData};

#[derive(Debug)]
pub struct Choice<'a> {
    pub(crate) children: VecX<Box<dyn DynCombinatorTrait + 'a>>,
    pub(crate) greedy: bool,
}

#[derive(Debug)]
pub struct ChoiceParser<'a> {
    pub(crate) parsers: Vec<Box<dyn ParserTrait + 'a>>,
    pub(crate) greedy: bool,
}

impl DynCombinatorTrait for Choice<'_> {
    fn parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> (Box<dyn ParserTrait + 'a>, ParseResults) {
        let (parser, parse_results) = self.parse(down_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(down_data, bytes)
    }
}

impl CombinatorTrait for Choice<'_> {
    type Parser<'a> = ChoiceParser<'a> where Self: 'a;
    type Output = Box<dyn std::any::Any>;
    type PartialOutput = Box<dyn std::any::Any>;


    fn one_shot_parse(&self, down_data: DownData, bytes: &[u8]) -> UnambiguousParseResults {
        if self.greedy {
            for parser in self.children.iter() {
                let parse_result = parser.one_shot_parse(down_data.clone(), bytes);
                match parse_result {
                    Ok(OneShotUpData { right_data }) => {
                        return Ok(OneShotUpData::new(right_data));
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
                let parse_result = parser.one_shot_parse(down_data.clone(), bytes);
                match parse_result {
                    Ok(OneShotUpData { right_data }) => {
                        for parser2 in self.children[i+1..].iter() {
                            let parse_result2 = parser2.one_shot_parse(down_data.clone(), bytes);
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
                        return Ok(OneShotUpData::new(right_data));
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

    fn old_parse(&self, down_data: DownData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let mut parsers = Vec::new();
        let mut combined_results = ParseResults::empty_finished();

        for child in &self.children {
            let (parser, parse_results) = child.as_ref().parse_dyn(down_data.clone(), bytes);
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
}

impl BaseCombinatorTrait for Choice<'_> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
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

pub fn _choice<'a>(v: Vec<Box<dyn DynCombinatorTrait + 'a>>) -> impl CombinatorTrait + 'a {
    Choice {
        children: v.into_iter().collect(),
        greedy: false,
    }
}

pub fn _choice_greedy<'a>(v: Vec<Box<dyn DynCombinatorTrait + 'a>>) -> impl CombinatorTrait + 'a {
    profile_internal("choice", Choice {
        children: v.into_iter().collect(),
        greedy: true,
    })
}