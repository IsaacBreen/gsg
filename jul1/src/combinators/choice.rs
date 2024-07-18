use crate::{CombinatorTrait, IntoCombinator, ParserTrait, Stats};
use crate::parse_state::{RightData, UpData};

pub struct Choice2<A, B>
where
    A: CombinatorTrait,
    B: CombinatorTrait,
{
    a: A,
    b: B,
}

pub struct Choice2Parser<ParserA, ParserB>
where
    ParserA: ParserTrait,
    ParserB: ParserTrait,
{
    pub(crate) a: Option<ParserA>,
    pub(crate) b: Option<ParserB>,
}

impl<A, B> CombinatorTrait for Choice2<A, B>
where
    A: CombinatorTrait,
    B: CombinatorTrait,
{
    type Parser = Choice2Parser<A::Parser, B::Parser>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (a, right_data_a, up_data_a) = self.a.parser(right_data.clone());
        let (b, right_data_b, up_data_b) = self.b.parser(right_data);
        (
            Choice2Parser { a: Some(a), b: Some(b) },
            right_data_a.into_iter().chain(right_data_b).collect(),
            up_data_a.into_iter().chain(up_data_b).collect()
        )
    }
}

impl<A, B> ParserTrait for Choice2Parser<A, B>
where
    A: ParserTrait,
    B: ParserTrait,
{
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        let (mut right_data, mut up_data) = (vec![], vec![]);
        if let Some(a) = &mut self.a {
            let (mut right_data_a, mut up_data_a) = a.step(c);
            if right_data_a.is_empty() && up_data_a.is_empty() {
                self.a = None;
            } else {
                right_data.append(&mut right_data_a);
                up_data.append(&mut up_data_a);
            }
        }
        if let Some(b) = &mut self.b {
            let (mut right_data_b, mut up_data_b) = b.step(c);
            if right_data_b.is_empty() && up_data_b.is_empty() {
                self.b = None;
            } else {
                right_data.append(&mut right_data_b);
                up_data.append(&mut up_data_b);
            }
        }
        (right_data, up_data)
    }
    fn collect_stats(&self, stats: &mut Stats) {
        self.a.as_ref().map(|a| a.collect_stats(stats));
        self.b.as_ref().map(|b| b.collect_stats(stats));
        stats.active_parser_type_counts.entry("Choice2Parser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

pub fn choice2<A, B>(a: A, b: B) -> Choice2<A::Output, B::Output>
where
    A: IntoCombinator,
    B: IntoCombinator,
{
    Choice2 { a: a.into_combinator(), b: b.into_combinator() }
}

#[macro_export]
macro_rules! choice {
    ($a1:expr $(,)?) => {$a1};
    ($a1:expr, $a2:expr $(,)?) => {$crate::choice2($a1, $a2)};
    ($a1:expr, $a2:expr, $a3:expr $(,)?) => {$crate::choice2($a1, $crate::choice2($a2, $a3))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr $(,)?) => {$crate::choice2($crate::choice2($a1, $a2), $crate::choice2($a3, $a4))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr $(,)?) => {$crate::choice2($crate::choice2($a1, $a2), $crate::choice2($a3, $crate::choice2($a4, $a5)))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr $(,)?) => {$crate::choice2($crate::choice2($a1, $crate::choice2($a2, $a3)), $crate::choice2($a4, $crate::choice2($a5, $a6)))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr $(,)?) => {$crate::choice2($crate::choice2($a1, $crate::choice2($a2, $a3)), $crate::choice2($crate::choice2($a4, $a5), $crate::choice2($a6, $a7)))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr $(,)?) => {$crate::choice2($crate::choice2($crate::choice2($a1, $a2), $crate::choice2($a3, $a4)), $crate::choice2($crate::choice2($a5, $a6), $crate::choice2($a7, $a8)))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr $(,)?) => {$crate::choice2($crate::choice2($crate::choice2($a1, $a2), $crate::choice2($a3, $a4)), $crate::choice2($crate::choice2($a5, $a6), $crate::choice2($a7, $crate::choice2($a8, $a9))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr $(,)?) => {$crate::choice2($crate::choice2($crate::choice2($a1, $a2), $crate::choice2($a3, $crate::choice2($a4, $a5))), $crate::choice2($crate::choice2($a6, $a7), $crate::choice2($a8, $crate::choice2($a9, $a10))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr $(,)?) => {$crate::choice2($crate::choice2($crate::choice2($a1, $a2), $crate::choice2($a3, $crate::choice2($a4, $a5))), $crate::choice2($crate::choice2($a6, $crate::choice2($a7, $a8)), $crate::choice2($a9, $crate::choice2($a10, $a11))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr $(,)?) => {$crate::choice2($crate::choice2($crate::choice2($a1, $crate::choice2($a2, $a3)), $crate::choice2($a4, $crate::choice2($a5, $a6))), $crate::choice2($crate::choice2($a7, $crate::choice2($a8, $a9)), $crate::choice2($a10, $crate::choice2($a11, $a12))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr $(,)?) => {$crate::choice2($crate::choice2($crate::choice2($a1, $crate::choice2($a2, $a3)), $crate::choice2($a4, $crate::choice2($a5, $a6))), $crate::choice2($crate::choice2($a7, $crate::choice2($a8, $a9)), $crate::choice2($crate::choice2($a10, $a11), $crate::choice2($a12, $a13))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr $(,)?) => {$crate::choice2($crate::choice2($crate::choice2($a1, $crate::choice2($a2, $a3)), $crate::choice2($crate::choice2($a4, $a5), $crate::choice2($a6, $a7))), $crate::choice2($crate::choice2($a8, $crate::choice2($a9, $a10)), $crate::choice2($crate::choice2($a11, $a12), $crate::choice2($a13, $a14))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr, $a15:expr $(,)?) => {$crate::choice2($crate::choice2($crate::choice2($a1, $crate::choice2($a2, $a3)), $crate::choice2($crate::choice2($a4, $a5), $crate::choice2($a6, $a7))), $crate::choice2($crate::choice2($crate::choice2($a8, $a9), $crate::choice2($a10, $a11)), $crate::choice2($crate::choice2($a12, $a13), $crate::choice2($a14, $a15))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr, $a15:expr, $a16:expr $(,)?) => {$crate::choice2($crate::choice2($crate::choice2($crate::choice2($a1, $a2), $crate::choice2($a3, $a4)), $crate::choice2($crate::choice2($a5, $a6), $crate::choice2($a7, $a8))), $crate::choice2($crate::choice2($crate::choice2($a9, $a10), $crate::choice2($a11, $a12)), $crate::choice2($crate::choice2($a13, $a14), $crate::choice2($a15, $a16))))}
}
