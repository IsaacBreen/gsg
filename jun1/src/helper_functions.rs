use std::cell::RefCell;
use std::rc::Rc;
use crate::combinator::Combinator;
use crate::combinators::*;
use crate::FrameStack;
use crate::parse_iteration_result::ParserIterationResult;
use crate::state::CombinatorState;
use crate::u8set::U8Set;

// Include all helper functions and macros

pub fn seq2<A, B, StateA, StateB>(a: A, b: B) -> Rc<Seq2<A, B>>
where
    A: Combinator<State = StateA>,
    B: Combinator<State = StateB>,
{
    Rc::new(Seq2(a, b))
}

pub fn repeat1<State>(a: Rc<dyn Combinator<State = State>>) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(Repeat1(a as Rc<dyn Combinator<State = Box<dyn CombinatorState>>>))
}

pub fn choice2<A, B, StateA, StateB>(a: A, b: B) -> Rc<Choice2<A, B>>
where
    A: Combinator<State = StateA>,
    B: Combinator<State = StateB>,
{
    Rc::new(Choice2(a, b))
}

pub fn eat_u8_matching(u8set: U8Set) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(EatU8Matching(u8set))
}

pub fn eat_u8(value: char) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    eat_u8_matching(U8Set::from_char(value))
}

pub fn eat_u8_range(start: char, end: char) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    eat_u8_matching(U8Set::from_range(start as u8, end as u8))
}

pub fn eat_string(value: &'static str) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(EatString(value))
}

pub fn eps() -> Eps {
    Eps
}

pub fn opt<C, State>(a: C) -> Rc<Choice2<C, Eps>>
where
    C: Combinator<State = State>,
{
    choice2(a, eps())
}

pub fn repeat<State>(a: Rc<dyn Combinator<State = State>>) -> Rc<Choice2<Rc<dyn Combinator<State=Box<dyn CombinatorState>>>, Eps>> {
    opt(repeat1(a))
}

pub fn forward_ref() -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(ForwardRef(Rc::new(RefCell::new(None))))
}

pub fn eat_u8_range_complement(start: char, end: char) -> Rc<Choice2<Rc<dyn Combinator<State=Box<dyn CombinatorState>>>, Rc<dyn Combinator<State=Box<dyn CombinatorState>>>>> {
    choice2(
        eat_u8_range(0 as char, start),
        eat_u8_range(end, 255 as char),
    )
}

pub fn process<C: Combinator<State = State> + ?Sized, State>(
    combinator: &C,
    c: Option<char>,
    its: &mut Vec<State>,
    signal_id: &mut usize,
) -> ParserIterationResult {
    if its.len() > 100 {
        // Warn if there are too many states
        eprintln!("Warning: there are {} states (process)", its.len());
    }
    let mut final_result = ParserIterationResult::new(U8Set::none(), false, FrameStack::default());
    its.retain_mut(|it| {
        let result = combinator.next_state(it, c, signal_id);
        let is_empty = result.u8set().is_empty();
        final_result.merge_assign(result);
        !is_empty
    });
    final_result
}

pub fn seq2_helper<C: Combinator<State = State> + ?Sized, State>(
    b: &C,
    a_result: &mut ParserIterationResult,
    b_result: ParserIterationResult,
    b_its: &mut Vec<State>,
    signal_id: &mut usize,
) {
    if b_its.len() > 100 {
        // Warn if there are too many states
        eprintln!("Warning: there are {} states (seq2_helper)", b_its.len());
    }
    if a_result.is_complete {
        let mut b_it = b.initial_state(signal_id, a_result.frame_stack.clone());
        let b_result = b.next_state(&mut b_it, None, signal_id);
        b_its.push(b_it);
        a_result.forward_assign(b_result);
    }
    a_result.merge_assign(b_result);
}

#[macro_export]
macro_rules! _seq {
    ($a:expr) => {
        $a
    };

    ($a:expr, $($b:expr),+ $(,)?) => {
        $crate::Seq2($a, $crate::_seq!($($b),+))
    };
}

#[macro_export]
macro_rules! _choice {
    ($a:expr) => {
        $a
    };

    ($a:expr, $($b:expr),+ $(,)?) => {
        $crate::Choice2($a, $crate::_choice!($($b),+))
    };
}

#[macro_export]
macro_rules! seq {
    ($($a:expr),+ $(,)?) => {
        Rc::new($crate::_seq!($($a),+))
    };
}

#[macro_export]
macro_rules! choice {
    ($($a:expr),+ $(,)?) => {
        Rc::new($crate::_choice!($($a),+))
    };
}