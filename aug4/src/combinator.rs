use std::fmt::Debug;
use crate::*;
use crate::helper_traits::{AsAny};

pub trait CombinatorTrait: Debug + AsAny {
    fn parse(&self, right_data: RightData, input: &[u8]) -> UnambiguousParseResults;
    fn rotate_right<'a>(&'a self) -> Choice<Seq<&'a dyn CombinatorTrait>>;
}

pub trait IntoBoxDynCombinator {
    fn into_dyn<'a>(self) -> Box<dyn CombinatorTrait + 'a> where Self: 'a;
}

impl<T: CombinatorTrait> IntoBoxDynCombinator for T {
    fn into_dyn<'a>(self) -> Box<dyn CombinatorTrait + 'a> where Self: 'a { Box::new(self) }
}

impl CombinatorTrait for Box<dyn CombinatorTrait> {
    fn parse(&self, right_data: RightData, input: &[u8]) -> UnambiguousParseResults {
        self.as_ref().parse(right_data, input)
    }

    fn rotate_right<'a>(&'a self) -> Choice<Seq<&'a dyn CombinatorTrait>> {
        self.as_ref().rotate_right()
    }
}

impl<T: CombinatorTrait + ?Sized> CombinatorTrait for &T {
    fn parse(&self, right_data: RightData, input: &[u8]) -> UnambiguousParseResults {
        (*self).parse(right_data, input)
    }

    fn rotate_right<'b>(&'b self) -> Choice<Seq<&'b dyn CombinatorTrait>> {
        (*self).rotate_right()
    }
}

impl<'a> PartialEq for &'a dyn CombinatorTrait {
    fn eq(&self, other: &Self) -> bool {
        let self_ptr = std::ptr::addr_of!(**self);
        let other_ptr = std::ptr::addr_of!(**other);
        std::ptr::addr_eq(self_ptr, other_ptr)
    }
}

// Non-greedy choice
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Choice<T: CombinatorTrait> {
    pub children: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Seq<T: CombinatorTrait> {
    pub children: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EatU8 {
    pub u8: u8,
}

impl<T: CombinatorTrait> AsAny for Choice<T> { fn as_any(&self) -> &dyn std::any::Any where Self: 'static { self } }
impl<T: CombinatorTrait> AsAny for Seq<T> { fn as_any(&self) -> &dyn std::any::Any where Self: 'static { self } }
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

    fn rotate_right<'a>(&'a self) -> Choice<Seq<&'a dyn CombinatorTrait>> {
        let mut new_children: Vec<Seq<_>> = vec![];
        for child in self.children.iter() {
            new_children.extend(child.rotate_right().children);
        }
        Choice { children: new_children }
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

    fn rotate_right<'a>(&'a self) -> Choice<Seq<&'a dyn CombinatorTrait>> {
        if let Some(first) = self.children.first() {
            let mut rot = first.rotate_right();
            for seq in rot.children.iter_mut() {
                // TODO: we can make this more efficient by defining a PartialSeq type that stores a reference to self
                //  and a child index and starts parsing at that child.
                for child in self.children.iter().skip(1) {
                    seq.children.push(child);
                }
            }
            rot
        } else {
            Choice { children: vec![seq!()] }
        }
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

    fn rotate_right<'a>(&'a self) -> Choice<Seq<&'a dyn CombinatorTrait>> {
        Choice { children: vec![seq!(self)] }
    }
}

pub fn eat_u8(u8: u8) -> EatU8 {
    EatU8 { u8 }
}

#[macro_export]
macro_rules! choice {
    ($($combinator:expr),* $(,)?) => {
        $crate::combinator::Choice {
            children: vec![$($combinator),*]
        }
    };
}

#[macro_export]
macro_rules! seq {
    ($($combinator:expr),* $(,)?) => {
        $crate::combinator::Seq {
            children: vec![$($combinator),*]
        }
    };
}

#[macro_export]
macro_rules! choice_dyn {
    ($($combinator:expr),* $(,)?) => {
        $crate::choice!($($combinator.into_dyn()),*)
    };
}

#[macro_export]
macro_rules! seq_dyn {
    ($($combinator:expr),* $(,)?) => {
        $crate::seq!($($combinator.into_dyn()),*)
    };
}

#[cfg(test)]
mod test_parse {
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
        let combinator = seq_dyn!(choice_dyn!(eat_u8(b'a'), seq_dyn!(eat_u8(b'a'), eat_u8(b'b'))), eat_u8(b'c'));
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

#[cfg(test)]
mod test_rotate_right {
    use super::*;

    #[test]
    fn test_ref_dyn_eq() {
        let a = eat_u8(b'a');
        let a_ref1 = &a as &dyn CombinatorTrait;
        let a_ref2 = &a as &dyn CombinatorTrait;
        assert_eq!(format!("{:?}", a_ref1), format!("{:?}", a_ref2));
    }

    #[test]
    fn test_eat_u8() {
        let combinator = eat_u8(b'a');
        let expected = choice!(seq!(&combinator as &dyn CombinatorTrait));
        assert_eq!(format!("{:?}", combinator.rotate_right()), format!("{:?}", expected));
    }

    #[test]
    fn test_choice() {
        let a = eat_u8(b'a');
        let b = eat_u8(b'b');
        let combinator = choice!(&a as &dyn CombinatorTrait, &b as &dyn CombinatorTrait);
        let expected = choice!(seq!(&a as &dyn CombinatorTrait), seq!(&b as &dyn CombinatorTrait));
        assert_eq!(format!("{:?}", combinator.rotate_right()), format!("{:?}", expected));
    }

    #[test]
    fn test_seq() {
        let a = eat_u8(b'a');
        let b = eat_u8(b'b');
        let combinator = seq!(&a as &dyn CombinatorTrait, &b as &dyn CombinatorTrait);
        let expected = choice!(seq!(&a as &dyn CombinatorTrait, &b as &dyn CombinatorTrait));
        assert_eq!(format!("{:?}", combinator.rotate_right()), format!("{:?}", expected));
    }

    // TODO: test more complicated cases
}