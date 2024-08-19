use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, ParseResultTrait, RightData, U8Set, VecX, VecY};
use std::rc::Rc;

#[derive(Debug)]
pub struct Choice {
    pub children: Vec<Rc<dyn CombinatorTrait>>,
    pub greedy: bool,
}

#[derive(Debug)]
pub struct ChoiceParser<'a> {
    pub children: Vec<Rc<dyn CombinatorTrait>>,
    pub greedy: bool,
    pub parsers: VecX<Parser<'a>>,
    pub parse_results: VecX<ParseResults>,
    pub finished: bool,
}

impl CombinatorTrait for Choice {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn apply(&self, f: &mut dyn FnMut(&dyn CombinatorTrait)) {
        for child in self.children.iter() {
            child.apply(f);
        }
    }

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        for child in self.children.iter() {
            let result = child.one_shot_parse(right_data.clone(), bytes);
            if result.is_ok() {
                return result;
            }
        }
        UnambiguousParseResults::Err(UnambiguousParseError::Fail)
    }

    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let mut parsers = VecX::new();
        let mut parse_results = VecX::new();
        for child in self.children.iter() {
            let (parser, parse_result) = child.parse(right_data.clone(), bytes);
            parsers.push(parser);
            parse_results.push(parse_result);
        }
        (
            Parser::ChoiceParser(ChoiceParser {
                children: self.children.clone(),
                greedy: self.greedy,
                parsers,
                parse_results,
                finished: false,
            }),
            ParseResults::new(VecY::new(), false),
        )
    }
}

impl ParserTrait for ChoiceParser<'_> {
    fn get_u8set(&self) -> U8Set {
        let mut u8set = U8Set::new();
        for parser in self.parsers.iter() {
            u8set = u8set.union(parser.get_u8set());
        }
        u8set
    }

    fn parse(&mut self, bytes: &[u8]) -> ParseResults {
        let mut all_results = VecY::new();
        let mut any_unfinished = false;
        let mut i = 0;
        while i < self.parsers.len() {
            let parser = &mut self.parsers[i];
            let parse_results = &mut self.parse_results[i];
            let ParseResults {
                right_data_vec: mut new_right_data_vec,
                done: new_done,
            } = parser.parse(bytes);
            if !new_right_data_vec.is_empty() {
                all_results.append(&mut new_right_data_vec);
                if self.greedy {
                    break;
                }
            }
            if new_done {
                self.parsers.swap_remove(i);
                self.parse_results.swap_remove(i);
            } else {
                any_unfinished = true;
                i += 1;
            }
        }
        ParseResults::new(all_results, !any_unfinished)
    }
}

pub fn choice(children: Vec<Combinator>) -> Choice {
    Choice {
        children: children.into_iter().map(|child| Rc::new(child) as Rc<dyn CombinatorTrait>).collect(),
        greedy: false,
    }
}

pub fn choice_greedy(children: Vec<Combinator>) -> Choice {
    Choice {
        children: children.into_iter().map(|child| Rc::new(child) as Rc<dyn CombinatorTrait>).collect(),
        greedy: true,
    }
}

// impl From<Choice> for Combinator {
//     fn from(value: Choice) -> Self {
//         Combinator::Choice(value)
//     }
// }
