use std::cell::RefCell;
use std::rc::Rc;
use crate::combinator::Combinator;
use crate::combinators::*;
use crate::FrameStack;
use crate::parse_iteration_result::ParserIterationResult;
use crate::state::CombinatorState;
use crate::u8set::U8Set;

// Include all helper functions and macros

pub fn seq<C, State, I>(combinators: I) -> Rc<Seq<C>>
where
    C: Combinator<State = State>,
    I: IntoIterator<Item = C>,
{
    Rc::new(Seq(combinators.into_iter().collect::<Vec<_>>()))
}

pub fn repeat1(a: Rc<dyn Combinator<State = Box<dyn CombinatorState>>>) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(Repeat1(a))
}

pub fn choice<I>(combinators: I) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>>
where
    I: IntoIterator<Item = Rc<dyn Combinator<State = Box<dyn CombinatorState>>>>,
{
    Rc::new(Choice(Rc::from(combinators.into_iter().collect::<Vec<_>>())))
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

pub fn eps() -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(Eps)
}

pub fn opt(a: Rc<dyn Combinator<State = Box<dyn CombinatorState>>>) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    choice(vec![a, eps()])
}

pub fn repeat(a: Rc<dyn Combinator<State = Box<dyn CombinatorState>>>) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    opt(repeat1(a))
}

pub fn forward_ref() -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(ForwardRef(Rc::new(RefCell::new(None))))
}

pub fn in_frame_stack(a: Rc<dyn Combinator<State = Box<dyn CombinatorState>>>) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(InFrameStack(a))
}

pub fn not_in_frame_stack(a: Rc<dyn Combinator<State = Box<dyn CombinatorState>>>) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(NotInFrameStack(a))
}

pub fn add_to_frame_stack(a: Rc<dyn Combinator<State = Box<dyn CombinatorState>>>) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(AddToFrameStack(a))
}

pub fn remove_from_frame_stack(a: Rc<dyn Combinator<State = Box<dyn CombinatorState>>>) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    Rc::new(RemoveFromFrameStack(a))
}

pub fn eat_u8_range_complement(start: char, end: char) -> Rc<dyn Combinator<State = Box<dyn CombinatorState>>> {
    choice(vec![
        eat_u8_range(0 as char, start as char),
        eat_u8_range(end, 255 as char),
    ])
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
macro_rules! seq {
    ($($a:expr),+ $(,)?) => {
        seq(vec![$($a),+])
    }
}

#[macro_export]
macro_rules! choice {
    ($($a:expr),+ $(,)?) => {
        choice(vec![$($a),+])
    }
}
