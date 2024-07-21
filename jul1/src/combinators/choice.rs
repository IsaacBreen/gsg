use std::any::Any;
use crate::{CombinatorTrait, DynCombinator, eps, fail, IntoCombinator, ParseResults, ParserTrait, Squash, Stats};
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

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        let (a, parse_results_a) = self.a.parser(right_data.clone());
        let (b, parse_results_b) = self.b.parser(right_data);
        (
            Choice2Parser { a: Some(a), b: Some(b) },
            parse_results_a.combine_inplace(parse_results_b).squashed()
        )
    }
}

impl<A, B> ParserTrait for Choice2Parser<A, B>
where
    A: ParserTrait + 'static,
    B: ParserTrait + 'static,
{
    fn step(&mut self, c: u8) -> ParseResults {
        let mut right_data = vec![];
        let mut up_data = vec![];
        let mut any_cut = false;

        if let Some(a) = &mut self.a {
            let ParseResults { right_data_vec: mut right_data_a, up_data_vec: mut up_data_a, cut } = a.step(c);
            any_cut = cut;
            if right_data_a.is_empty() && up_data_a.is_empty() {
                self.a = None;
            } else {
                right_data.append(&mut right_data_a);
                up_data.append(&mut up_data_a);
            }
        }

        if let Some(b) = &mut self.b {
            let ParseResults { right_data_vec: mut right_data_b, up_data_vec: mut up_data_b, cut } = b.step(c);
            if cut && !any_cut {
                // Clear the 'a' combinator and any up data from 'a' if 'b' cuts and 'a' didn't
                self.a = None;
                up_data.clear();
                any_cut = true;
            }
            if cut || !any_cut {
                if right_data_b.is_empty() && up_data_b.is_empty() {
                    self.b = None;
                } else {
                    up_data.append(&mut up_data_b);
                }
            }
            right_data.append(&mut right_data_b);
        }

        ParseResults {
            right_data_vec: right_data.squashed(),
            up_data_vec: up_data,
            cut: any_cut,
        }
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        Box::new(self.a.iter().map(|a| a as &dyn ParserTrait).chain(self.b.iter().map(|b| b as &dyn ParserTrait)))
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        Box::new(self.a.iter_mut().map(|a| a as &mut dyn ParserTrait).chain(self.b.iter_mut().map(|b| b as &mut dyn ParserTrait)))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn choice2<A, B>(a: A, b: B) -> Choice2<A::Output, B::Output>
where
    A: IntoCombinator,
    B: IntoCombinator,
{
    Choice2 { a: a.into_combinator(), b: b.into_combinator() }
}


impl<T> From<Vec<T>> for Choice2<Box<DynCombinator>, Box<DynCombinator>>
where
    T: IntoCombinator,
{
    fn from(mut v: Vec<T>) -> Self {
        fn helper<T>(mut v: Vec<T>) -> Box<DynCombinator>
        where
            T: IntoCombinator,
        {
            if v.len() == 1 {
                v.into_iter().next().unwrap().into_combinator().into_box_dyn()
            } else {
                let rest = v.split_off(v.len() / 2);
                choice2(helper(v), helper(rest)).into_box_dyn()
            }
        }
        assert!(v.len() >= 2);
        let rest = v.split_off(v.len() / 2);
        choice2(helper(v), helper(rest))
    }
}

pub fn choice_from_vec<T>(v: Vec<T>) -> Box<DynCombinator>
where
    T: IntoCombinator,
{
    let mut v = v;
    if v.len() == 0 {
        fail().into_box_dyn()
    } else if v.len() == 1 {
        v.into_iter().next().unwrap().into_combinator().into_box_dyn()
    } else {
        let rest = v.split_off(v.len() / 2);
        choice2(choice_from_vec(v), choice_from_vec(rest)).into_box_dyn()
    }
}

#[macro_export]
macro_rules! choice {
    ($a1:expr $(,)?) => {($a1)};
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
