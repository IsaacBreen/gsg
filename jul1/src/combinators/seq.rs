use std::rc::Rc;

use crate::*;
use crate::parse_state::{RightData, UpData};

pub struct Seq2<A, B>
where
    A: CombinatorTrait,
    B: CombinatorTrait,
{
    a: A,
    b: Rc<B>,
}

pub struct Seq2Parser<B, ParserA>
where
    ParserA: ParserTrait,
    B: CombinatorTrait,
{
    pub(crate) a: Option<ParserA>,
    pub(crate) bs: Vec<B::Parser>,
    b: Rc<B>,
    right_data: RightData,
}

impl<A, B> CombinatorTrait for Seq2<A, B>
where
    A: CombinatorTrait,
    B: CombinatorTrait,
{
    type Parser = Seq2Parser<B, A::Parser>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, Vec<RightData>, Vec<UpData>) {
        let (a, right_data_a, up_data_a) = self.a.parser(right_data.clone());
        let (mut bs, mut right_data_bs, mut up_data_bs) = (vec![], vec![], vec![]);
        for right_data_b in right_data_a {
            let (b, right_data_b, up_data_b) = self.b.parser(right_data_b);
            bs.push(b);
            right_data_bs.extend(right_data_b);
            up_data_bs.extend(up_data_b);
        }
        let parser = Seq2Parser {
            a: Some(a),
            bs,
            b: self.b.clone(),
            right_data,
        };
        (parser, right_data_bs, up_data_bs.into_iter().chain(up_data_a).collect())
    }
}

impl<ParserA, B> ParserTrait for Seq2Parser<B, ParserA>
where
    ParserA: ParserTrait,
    B: CombinatorTrait,
{
    fn step(&mut self, c: u8) -> (Vec<RightData>, Vec<UpData>) {
        let (right_data_a, up_data_a) = self.a.as_mut().map(|a| a.step(c)).unwrap_or((vec![], vec![]));
        if right_data_a.is_empty() && up_data_a.is_empty() {
            self.a = None;
        }
        let (mut right_data_bs, mut up_data_bs) = (vec![], vec![]);
        self.bs.retain_mut(|b| {
            let (right_data_b, up_data_b) = b.step(c);
            if right_data_b.is_empty() && up_data_b.is_empty() {
                false
            } else {
                right_data_bs.extend(right_data_b);
                up_data_bs.extend(up_data_b);
                true
            }
        });
        for right_data_b in right_data_a {
            let (b, right_data_b, up_data_b) = self.b.parser(right_data_b);
            self.bs.push(b);
            right_data_bs.extend(right_data_b);
            up_data_bs.extend(up_data_b);
        }
        (right_data_bs, up_data_bs.into_iter().chain(up_data_a).collect())
    }

    fn collect_stats(&self, stats: &mut Stats) {
        self.a.as_ref().map(|a| a.collect_stats(stats));
        for b in &self.bs {
            b.collect_stats(stats);
        }
        stats.active_parser_type_counts.entry("Seq2Parser".to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
}

pub fn seq2<A, B>(a: A, b: B) -> Seq2<A::Output, B::Output>
where
    A: IntoCombinator,
    B: IntoCombinator,
{
    Seq2 { a: a.into_combinator(), b: Rc::new(b.into_combinator()) }
}

impl<T> From<Vec<T>> for Seq2<Box<DynCombinator>, Box<DynCombinator>>
where
    T: IntoCombinator,
{
    fn from(mut v: Vec<T>) -> Self {
        fn helper<T>(mut v: Vec<T>) -> Box<DynCombinator>
        where
            T: IntoCombinator,
        {
            if v.len() == 1 {
                v.into_iter().next().unwrap().into_combinator().into_boxed()
            } else {
                let rest = v.split_off(v.len() / 2);
                seq2(helper(v), helper(rest)).into_boxed()
            }
        }
        assert!(v.len() >= 2);
        let rest = v.split_off(v.len() / 2);
        seq2(helper(v), helper(rest))
    }
}

pub fn seq_from_vec<T>(v: Vec<T>) -> Box<DynCombinator>
where
    T: IntoCombinator,
{
    let mut v = v;
    if v.len() == 1 {
        v.into_iter().next().unwrap().into_combinator().into_boxed()
    } else {
        let rest = v.split_off(v.len() / 2);
        seq2(seq_from_vec(v), seq_from_vec(rest)).into_boxed()
    }
}

#[macro_export]
macro_rules! seq {
    ($a1:expr $(,)?) => {$a1};
    ($a1:expr, $a2:expr $(,)?) => {$crate::seq2($a1, $a2)};
    ($a1:expr, $a2:expr, $a3:expr $(,)?) => {$crate::seq2($a1, $crate::seq2($a2, $a3))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr $(,)?) => {$crate::seq2($crate::seq2($a1, $a2), $crate::seq2($a3, $a4))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr $(,)?) => {$crate::seq2($crate::seq2($a1, $a2), $crate::seq2($a3, $crate::seq2($a4, $a5)))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr $(,)?) => {$crate::seq2($crate::seq2($a1, $crate::seq2($a2, $a3)), $crate::seq2($a4, $crate::seq2($a5, $a6)))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr $(,)?) => {$crate::seq2($crate::seq2($a1, $crate::seq2($a2, $a3)), $crate::seq2($crate::seq2($a4, $a5), $crate::seq2($a6, $a7)))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr $(,)?) => {$crate::seq2($crate::seq2($crate::seq2($a1, $a2), $crate::seq2($a3, $a4)), $crate::seq2($crate::seq2($a5, $a6), $crate::seq2($a7, $a8)))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr $(,)?) => {$crate::seq2($crate::seq2($crate::seq2($a1, $a2), $crate::seq2($a3, $a4)), $crate::seq2($crate::seq2($a5, $a6), $crate::seq2($a7, $crate::seq2($a8, $a9))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr $(,)?) => {$crate::seq2($crate::seq2($crate::seq2($a1, $a2), $crate::seq2($a3, $crate::seq2($a4, $a5))), $crate::seq2($crate::seq2($a6, $a7), $crate::seq2($a8, $crate::seq2($a9, $a10))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr $(,)?) => {$crate::seq2($crate::seq2($crate::seq2($a1, $a2), $crate::seq2($a3, $crate::seq2($a4, $a5))), $crate::seq2($crate::seq2($a6, $crate::seq2($a7, $a8)), $crate::seq2($a9, $crate::seq2($a10, $a11))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr $(,)?) => {$crate::seq2($crate::seq2($crate::seq2($a1, $crate::seq2($a2, $a3)), $crate::seq2($a4, $crate::seq2($a5, $a6))), $crate::seq2($crate::seq2($a7, $crate::seq2($a8, $a9)), $crate::seq2($a10, $crate::seq2($a11, $a12))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr $(,)?) => {$crate::seq2($crate::seq2($crate::seq2($a1, $crate::seq2($a2, $a3)), $crate::seq2($a4, $crate::seq2($a5, $a6))), $crate::seq2($crate::seq2($a7, $crate::seq2($a8, $a9)), $crate::seq2($crate::seq2($a10, $a11), $crate::seq2($a12, $a13))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr $(,)?) => {$crate::seq2($crate::seq2($crate::seq2($a1, $crate::seq2($a2, $a3)), $crate::seq2($crate::seq2($a4, $a5), $crate::seq2($a6, $a7))), $crate::seq2($crate::seq2($a8, $crate::seq2($a9, $a10)), $crate::seq2($crate::seq2($a11, $a12), $crate::seq2($a13, $a14))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr, $a15:expr $(,)?) => {$crate::seq2($crate::seq2($crate::seq2($a1, $crate::seq2($a2, $a3)), $crate::seq2($crate::seq2($a4, $a5), $crate::seq2($a6, $a7))), $crate::seq2($crate::seq2($crate::seq2($a8, $a9), $crate::seq2($a10, $a11)), $crate::seq2($crate::seq2($a12, $a13), $crate::seq2($a14, $a15))))};
    ($a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr, $a8:expr, $a9:expr, $a10:expr, $a11:expr, $a12:expr, $a13:expr, $a14:expr, $a15:expr, $a16:expr $(,)?) => {$crate::seq2($crate::seq2($crate::seq2($crate::seq2($a1, $a2), $crate::seq2($a3, $a4)), $crate::seq2($crate::seq2($a5, $a6), $crate::seq2($a7, $a8))), $crate::seq2($crate::seq2($crate::seq2($a9, $a10), $crate::seq2($a11, $a12)), $crate::seq2($crate::seq2($a13, $a14), $crate::seq2($a15, $a16))))}
}
