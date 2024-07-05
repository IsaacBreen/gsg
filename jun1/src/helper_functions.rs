use std::cell::RefCell;
use std::rc::Rc;
use crate::combinator::Combinator;
use crate::combinators::*;
use crate::FrameStack;
use crate::parse_iteration_result::ParserIterationResult;
use crate::state::CombinatorState;
use crate::u8set::U8Set;

// Include all helper functions and macros

pub fn seq<I, C>(combinators: I) -> Rc<Seq<C>>
where
    I: IntoIterator<Item = C>,
    C: Combinator,
{
    Rc::new(Seq(Rc::from_iter(combinators)))
}

pub fn repeat1<C: Combinator>(a: C) -> Rc<Repeat1<C>> {
    Rc::new(Repeat1(a))
}

pub fn choice<I, C>(combinators: I) -> Rc<Choice<C>>
where
    I: IntoIterator<Item = C>,
{
    Rc::new(Choice(Rc::from_iter(combinators)))
}

pub fn eat_u8_matching(u8set: U8Set) -> Rc<EatU8Matching> {
    Rc::new(EatU8Matching(u8set))
}

pub fn eat_u8(value: char) -> Rc<EatU8Matching> {
    eat_u8_matching(U8Set::from_char(value))
}

pub fn eat_u8_range(start: char, end: char) -> Rc<EatU8Matching> {
    eat_u8_matching(U8Set::from_range(start as u8, end as u8))
}

pub fn eat_string(value: &'static str) -> Rc<EatString> {
    Rc::new(EatString(value))
}

pub fn eps() -> Eps {
    Eps
}

pub fn repeat<C: Combinator>(a: C) -> Rc<dyn Combinator<State = dyn CombinatorState>> {
    opt(repeat1(a))
}

pub fn call<F, C>(f: F) -> Rc<Call<F, C, C::State>>
where
    F: Fn() -> Rc<C> + 'static,
    C: Combinator,
{
    Rc::new(Call(f))
}

pub fn forward_ref<C: Combinator>() -> Rc<ForwardRef<C>> {
    Rc::new(ForwardRef(Rc::new(RefCell::new(None))))
}

pub fn in_frame_stack<C: Combinator>(a: C) -> Rc<InFrameStack<C>> {
    Rc::new(InFrameStack(Rc::new(a)))
}

pub fn not_in_frame_stack<C: Combinator>(a: C) -> Rc<NotInFrameStack<C>> {
    Rc::new(NotInFrameStack(Rc::new(a)))
}

pub fn add_to_frame_stack<C: Combinator>(a: C) -> Rc<AddToFrameStack<C>> {
    Rc::new(AddToFrameStack(Rc::new(a)))
}

pub fn remove_from_frame_stack<C: Combinator>(a: C) -> Rc<RemoveFromFrameStack<C>> {
    Rc::new(RemoveFromFrameStack(Rc::new(a)))
}

pub fn process<C, State>(
    combinator: &C,
    c: Option<char>,
    its: &mut Vec<State>,
    signal_id: &mut usize,
) -> ParserIterationResult
where
    C: Combinator<State = State>,
{
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

pub fn seq2_helper<C: Combinator<State = State>, State>(
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
macro_rules! seq {
    ($($a:expr),+ $(,)?) => {
        seq(vec![$($a.into()),+])
    }
}

#[macro_export]
macro_rules! choice {
    ($($a:expr),+ $(,)?) => {
        choice(vec![$($a.into()),+])
    }
}

pub fn opt<C: Combinator>(a: C) -> Rc<Choice<dyn CombinatorState>> {
    choice!(a, eps())
}

pub fn eat_u8_range_complement(start: char, end: char) -> Rc<EatU8Matching> {
    choice!(
        eat_u8_range(0 as char, start as char),
        eat_u8_range(end, 255 as char),
    )
}
