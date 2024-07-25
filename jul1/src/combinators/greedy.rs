use std::any::Any;
use crate::{choice, Choice2, CombinatorTrait, IntoCombinator, ParseResults, ParserTrait, Squash, Stats, U8Set};
use crate::parse_state::{RightData, UpData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GreedOrder {
    allowed_u8s: U8Set,
}

impl Default for GreedOrder {
    fn default() -> Self {
        GreedOrder {
            allowed_u8s: U8Set::all(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Greedy<A> {
    a: A,
}

pub struct GreedyParser<P> {
    parser: P,
}

impl<A> CombinatorTrait for Greedy<A>
where
    A: CombinatorTrait,
{
    type Parser = GreedyParser<A::Parser>;
    fn parser(&self, mut right_data: RightData) -> (Self::Parser, ParseResults) {
        let existing_greed_order = right_data.greed_order.clone();

        // Ensure any up data obeys the existing greed order.
        let (parser, mut parse_results) = self.a.parser(right_data);
        for up_data in &mut parse_results.up_data_vec {
            up_data.u8set = up_data.u8set.intersection(&existing_greed_order.allowed_u8s);
        }
        // Remove any up data that doesn't match anything.
        parse_results.up_data_vec.retain(|up_data| !up_data.u8set.is_empty());

        // Create a new greed order.
        // Get the union of u8sets from all up data. These are all possible next characters.
        let up_data_u8set = parse_results.up_data_vec.iter().map(|up_data| up_data.u8set).reduce(|u8set1, u8set2| u8set1.union(&u8set2)).unwrap_or_default();
        // Create a new greed order that disallows the current up data characters.
        let new_greed_order = GreedOrder { allowed_u8s: up_data_u8set.complement() };
        // Apply it to all right data.
        for right_data in &mut parse_results.right_data_vec {
            right_data.greed_order.allowed_u8s = right_data.greed_order.allowed_u8s.intersection(&new_greed_order.allowed_u8s);
        }
        // Remove any right data that totally disallows matching.
        parse_results.right_data_vec.retain(|right_data| !right_data.greed_order.allowed_u8s.is_empty());

        (GreedyParser { parser }, parse_results.squashed())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<P> ParserTrait for GreedyParser<P>
where
    P: ParserTrait + 'static,
{
    fn step(&mut self, c: u8) -> ParseResults {
        let mut parse_results = self.parser.step(c);

        // Create a new greed order.
        // Get the union of u8sets from all up data. These are all possible next characters.
        let up_data_u8set = parse_results.up_data_vec.iter().map(|up_data| up_data.u8set).reduce(|u8set1, u8set2| u8set1.union(&u8set2)).unwrap_or_default();
        // Create a new greed order that disallows the current up data characters.
        let new_greed_order = GreedOrder { allowed_u8s: up_data_u8set.complement() };
        // Apply it to all right data.
        for right_data in &mut parse_results.right_data_vec {
            right_data.greed_order.allowed_u8s = right_data.greed_order.allowed_u8s.intersection(&new_greed_order.allowed_u8s);
        }
        // Remove any right data that totally disallows matching.
        parse_results.right_data_vec.retain(|right_data| !right_data.greed_order.allowed_u8s.is_empty());

        parse_results.squashed()
    }

    fn dyn_eq(&self, other: &dyn ParserTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.parser.dyn_eq(&other.parser)
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// pub struct GreedyGuard<A> {
//     a: A,
// }
//
// pub struct GreedyGuardParser<P> {
//     parser: P,
// }

pub fn greedy<A>(a: A) -> Greedy<A::Output>
where
    A: IntoCombinator,
{
    Greedy { a: a.into_combinator() }
}