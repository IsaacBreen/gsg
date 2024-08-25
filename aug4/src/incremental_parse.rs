use crate::*;
use crate::parser::{ChoiceParser, ParserTrait, SeqParser};

pub struct IncrementalParser<'a> {
    pub parser: ChoiceParser<SeqParser<Box<dyn ParserTrait + 'a>, Box<dyn CombinatorTrait + 'a>>>,
}