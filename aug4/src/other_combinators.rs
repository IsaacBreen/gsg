use crate::{Choice, CombinatorTrait, RightData, Seq, UnambiguousParseResults};
use crate::helper_traits::AsAny;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Seq2<L, R> {
    pub l: L,
    pub r: R,
}

impl<L: CombinatorTrait, R: CombinatorTrait> AsAny for Seq2<L, R> { fn as_any(&self) -> &dyn std::any::Any where Self: 'static { self } }

impl<L: CombinatorTrait, R: CombinatorTrait> CombinatorTrait for Seq2<L, R> {
    fn parse(&self, right_data: RightData, input: &[u8]) -> UnambiguousParseResults {
        let parse_result = self.l.parse(right_data.clone(), input);
        match parse_result {
            Ok(new_right_data) => {
                self.r.parse(new_right_data, input)
            }
            Err(_) => parse_result,
        }
    }

    fn rotate_right<'a>(&'a self) -> Choice<Seq<&'a dyn CombinatorTrait>> {
        let mut rot = self.l.rotate_right();
        for child in rot.children.iter_mut() {
            child.children.push(&self.r);
        }
        rot
    }
}