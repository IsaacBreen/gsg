use crate::*;

pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait CombinatorTrait {
    fn parse(&self, right_data: RightData, input: &[u8]) -> UnambiguousParseResults;
    fn rotate_right<'a>(&'a self) -> Choice<Seq<Box<dyn CombinatorTrait + 'a>>>;
}

// Non-greedy choice
pub struct Choice<T> {
    pub children: Vec<T>,
}

pub struct Seq<T> {
    pub children: Vec<T>,
}

pub struct EatU8 {
    pub u8: u8,
}

impl<T: 'static> AsAny for Choice<T> { fn as_any(&self) -> &dyn std::any::Any { self } }
impl<T: 'static> AsAny for Seq<T> { fn as_any(&self) -> &dyn std::any::Any { self } }
impl AsAny for EatU8 { fn as_any(&self) -> &dyn std::any::Any { self } }

impl<T: CombinatorTrait> CombinatorTrait for Choice<T> {
    fn parse(&self, right_data: RightData, input: &[u8]) -> UnambiguousParseResults {
        for (i, child) in self.children.iter().enumerate() {
            let parse_result = child.parse(right_data.clone(), input);
            match parse_result {
                Ok(new_right_data) => {
                    for other_child in self.children[i + 1..].iter() {
                        let other_parse_result = other_child.parse(right_data.clone(), input);
                        match other_parse_result {
                            Ok(_) | Err(UnambiguousParseError::Ambiguous) => {
                                return Err(UnambiguousParseError::Ambiguous);
                            },
                            Err(UnambiguousParseError::Incomplete) => {
                                return Err(UnambiguousParseError::Incomplete);
                            }
                            Err(UnambiguousParseError::Fail) => {
                                continue;
                            },
                        }
                    };
                    return Ok(new_right_data);
                }
                Err(UnambiguousParseError::Fail) => {
                    continue;
                }
                Err(UnambiguousParseError::Ambiguous | UnambiguousParseError::Incomplete) => {
                    return parse_result;
                }
            }
        }
        Err(UnambiguousParseError::Fail)
    }

    fn rotate_right<'a>(&'a self) -> Choice<Seq<Box<dyn CombinatorTrait + 'a>>> {
        todo!()
    }
}

impl<T: CombinatorTrait> CombinatorTrait for Seq<T> {
    fn parse(&self, mut right_data: RightData, input: &[u8]) -> UnambiguousParseResults {
        for child in self.children.iter() {
            let parse_result = child.parse(right_data.clone(), input);
            match parse_result {
                Ok(new_right_data) => {
                    right_data = new_right_data;
                }
                Err(_) => {
                    return parse_result;
                }
            }
        }
        Ok(right_data)
    }

    fn rotate_right<'a>(&'a self) -> Choice<Seq<Box<dyn CombinatorTrait + 'a>>> {
        todo!()
    }
}

impl CombinatorTrait for EatU8 {
    fn parse(&self, right_data: RightData, input: &[u8]) -> UnambiguousParseResults {
        match input.get(0) {
            Some(byte) if *byte == self.u8 => Ok(right_data),
            Some(_) => Err(UnambiguousParseError::Fail),
            None => Err(UnambiguousParseError::Incomplete),
        }
    }

    fn rotate_right<'a>(&'a self) -> Choice<Seq<Box<dyn CombinatorTrait + 'a>>> {
        todo!()
    }
}