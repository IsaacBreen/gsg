use crate::*;

pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait CombinatorTrait {
    fn parse(&self, right_data: RightData, input: &[u8]) -> UnambiguousParseResults;
    fn rotate_right<'a>(&'a self) -> Choice<Seq<Box<dyn CombinatorTrait + 'a>>>;
}

impl CombinatorTrait for Box<dyn CombinatorTrait> {
    fn parse(&self, right_data: RightData, input: &[u8]) -> UnambiguousParseResults {
        self.as_ref().parse(right_data, input)
    }

    fn rotate_right<'a>(&'a self) -> Choice<Seq<Box<dyn CombinatorTrait + 'a>>> {
        self.as_ref().rotate_right()
    }
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
        let start_position = right_data.position();
        for child in self.children.iter() {
            let offset = right_data.position() - start_position;
            let parse_result = child.parse(right_data.clone(), &input[offset..]);
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
    fn parse(&self, mut right_data: RightData, input: &[u8]) -> UnambiguousParseResults {
        match input.get(0) {
            Some(byte) if *byte == self.u8 => {
                right_data.advance(1);
                Ok(right_data)
            },
            Some(_) => Err(UnambiguousParseError::Fail),
            None => Err(UnambiguousParseError::Incomplete),
        }
    }

    fn rotate_right<'a>(&'a self) -> Choice<Seq<Box<dyn CombinatorTrait + 'a>>> {
        todo!()
    }
}

pub fn eat_u8(u8: u8) -> EatU8 {
    EatU8 { u8 }
}

#[macro_export]
macro_rules! choice {
    ($($combinator:expr),* $(,)?) => {
        $crate::combinator::Choice {
            children: vec![$(Box::new($combinator) as Box<dyn CombinatorTrait>),*]
        }
    };
}

#[macro_export]
macro_rules! seq {
    ($($combinator:expr),* $(,)?) => {
        $crate::combinator::Seq {
            children: vec![$(Box::new($combinator) as Box<dyn CombinatorTrait>),*]
        }
    };
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use super::*;

    macro_rules! assert_parse_result_matches {
        ($combinator:expr, $input:expr, $expected_result:pat) => {
            assert_matches!($combinator.parse(RightData::new(), $input), $expected_result);
        };
    }

    #[test]
    fn test_eat_u8() {
        let combinator = eat_u8(b'a');
        assert_parse_result_matches!(combinator, b"a", Ok(_));
        assert_parse_result_matches!(combinator, b"b", Err(UnambiguousParseError::Fail));
        assert_parse_result_matches!(combinator, b"", Err(UnambiguousParseError::Incomplete));
    }

    #[test]
    fn test_choice() {
        let combinator = choice!(
            eat_u8(b'a'),
            eat_u8(b'b')
        );
        assert_parse_result_matches!(combinator, b"a", Ok(_));
        assert_parse_result_matches!(combinator, b"b", Ok(_));
        assert_parse_result_matches!(combinator, b"c", Err(UnambiguousParseError::Fail));

        let combinator = choice!(
            eat_u8(b'a'),
            eat_u8(b'a'),
            eat_u8(b'b')
        );
        assert_parse_result_matches!(combinator, b"a", Err(UnambiguousParseError::Ambiguous));
        assert_parse_result_matches!(combinator, b"b", Ok(_));
    }

    #[test]
    fn test_seq() {
        let combinator = seq!(
            eat_u8(b'a'),
            eat_u8(b'b')
        );
        assert_parse_result_matches!(combinator, b"ab", Ok(_));
        assert_parse_result_matches!(combinator, b"ba", Err(UnambiguousParseError::Fail));
        assert_parse_result_matches!(combinator, b"a", Err(UnambiguousParseError::Incomplete));
    }

    #[test]
    fn test_seq_choice_seq() {
        let combinator = seq!(choice!(eat_u8(b'a'), seq!(eat_u8(b'a'), eat_u8(b'b'))), eat_u8(b'c'));
        assert_parse_result_matches!(combinator, b"ac", Ok(_));
        // "abc" is ambiguous according to the inner combinator `choice!(eat_u8(b'a'), seq!(eat_u8(b'a'), eat_u8(b'b')))`.
        // So, even though *we* can tell that ambiguity gets resolved by reading the final "c", the inner choice combinator
        // can't tell that, and it returns `Err(UnambiguousParseError::Ambiguous)`.
        assert_parse_result_matches!(combinator, b"abc", Err(UnambiguousParseError::Ambiguous));
        // "ab" is a similar story.
        assert_parse_result_matches!(combinator, b"ab", Err(UnambiguousParseError::Ambiguous));
        assert_parse_result_matches!(combinator, b"bc", Err(UnambiguousParseError::Fail));
    }
}