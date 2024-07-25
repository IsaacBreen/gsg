use std::any::Any;
use std::rc::Rc;

use crate::{Choice2, CombinatorTrait, Eps, IntoCombinator, opt, ParseResults, ParserTrait, seq, Seq2, Squash, Stats};
use crate::parse_state::{RightData, UpData};

#[derive(PartialEq)]
pub struct Repeat1<A>
where
    A: CombinatorTrait,
{
    a: Rc<A>,
}

pub struct Repeat1Parser<A>
where
    A: CombinatorTrait,
{
    pub(crate) a: Rc<A>,
    pub(crate) a_parsers: Vec<A::Parser>,
    right_data: RightData,
}

impl<A> CombinatorTrait for Repeat1<A>
where
    A: CombinatorTrait,
{
    type Parser = Repeat1Parser<A>;

    fn parser(&self, right_data: RightData) -> (Self::Parser, ParseResults) {
        let (a, mut parse_results) = self.a.parser(right_data.clone());
        assert!(parse_results.right_data_vec.is_empty());
        // parse_results.right_data_vec.clear();
        // parse_results.cut |= parse_results.right_data_vec.is_empty();
        let a_parsers = if !parse_results.right_data_vec.is_empty() || !parse_results.up_data_vec.is_empty() {
            vec![a]
        } else {
            vec![]
        };
        (Repeat1Parser { a: self.a.clone(), a_parsers, right_data }, parse_results)
    }
}

impl<A> ParserTrait for Repeat1Parser<A>
where
    A: CombinatorTrait,
{
    fn step(&mut self, c: u8) -> ParseResults {
        let mut right_data_as = vec![];
        let mut up_data_as = vec![];
        let mut any_cut = false;
        let mut new_parsers = vec![];

        for mut a_parser in self.a_parsers.drain(..) {
            let ParseResults { right_data_vec: right_data_a, up_data_vec: up_data_a, cut } = a_parser.step(c);
                if cut && !any_cut {
                    // Clear any parsers and up data up to this point, but not right data
                    new_parsers.clear();
                    up_data_as.clear();
                    any_cut = true;
                }
            if cut || !any_cut {
                if !right_data_a.is_empty() || !up_data_a.is_empty() {
                    new_parsers.push(a_parser);
                }
                up_data_as.extend(up_data_a);
            }
            right_data_as.extend(right_data_a);
        }

        right_data_as.squash();

        for right_data_a in right_data_as.clone() {
            let (a_parser, ParseResults { right_data_vec: mut right_data_a, up_data_vec: up_data_a, mut cut }) = self.a.parser(right_data_a);
            assert!(right_data_a.is_empty());
            // right_data_a.clear();
            // cut |= right_data_a.is_empty();
            if cut && !any_cut {
                // Clear any parsers and up data up to this point, but not right data
                new_parsers.clear();
                up_data_as.clear();
                any_cut = true;
            }
            if cut || !any_cut {
                up_data_as.extend(up_data_a);
                new_parsers.push(a_parser);
            }
            right_data_as.extend(right_data_a);
        }

        // right_data_as.squash();

        self.a_parsers = new_parsers;

        ParseResults {
            right_data_vec: right_data_as,
            up_data_vec: up_data_as,
            cut: any_cut,
        }
            // .squashed()
    }

    fn iter_children<'a>(&'a self) -> Box<dyn Iterator<Item=&'a dyn ParserTrait> + 'a> {
        Box::new(self.a_parsers.iter().map(|a| a as &dyn ParserTrait))
    }

    fn iter_children_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item=&'a mut dyn ParserTrait> + 'a> {
        Box::new(self.a_parsers.iter_mut().map(|a| a as &mut dyn ParserTrait))
    }

    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            let a_eq = CombinatorTrait::dyn_eq(&self.a, &other.a.into_box_dyn());
            let a_parsers_eq = self.a_parsers.iter().zip(other.a_parsers.iter()).all(|(a, b)| a.dyn_eq(b));
            let right_data_eq = self.right_data == other.right_data;
            a_eq && a_parsers_eq && right_data_eq
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn repeat1<A>(a: A) -> Repeat1<A::Output>
where
    A: IntoCombinator,
{
    Repeat1 { a: Rc::new(a.into_combinator()) }
}

pub fn repeat0<A>(a: A) -> Choice2<Repeat1<A::Output>, Eps>
where
    A: IntoCombinator,
{
    opt(repeat1(a.into_combinator()))
}

pub fn seprep1<A, B>(a: A, b: B) -> Seq2<Rc<<A as IntoCombinator>::Output>, Choice2<Repeat1<Seq2<<B as IntoCombinator>::Output, Rc<<A as IntoCombinator>::Output>>>, Eps>>
where
    A: IntoCombinator,
    B: IntoCombinator,
{
    let a = Rc::new(a.into_combinator());
    seq!(a.clone(), repeat0(seq!(b, a.clone())))
}

pub fn seprep0<A, B>(a: A, b: B) -> Seq2<Choice2<Repeat1<Seq2<Rc<<A as IntoCombinator>::Output>, <B as IntoCombinator>::Output>>, Eps>, Rc<<A as IntoCombinator>::Output>>
where
    A: IntoCombinator,
    B: IntoCombinator,
{
    let a = Rc::new(a.into_combinator());
    seq!(opt(repeat1(seq!(a.clone(), b))), a)
}