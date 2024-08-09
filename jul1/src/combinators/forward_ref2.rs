use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;
use crate::*;

#[derive(Debug, Clone, Eq)]
pub struct ForwardRef2<'a> {
    b: Rc<RefCell<Option<&'a Combinator<'a>>>>,
}

impl PartialEq for ForwardRef2<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Hash for ForwardRef2<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state);
    }
}

impl ForwardRef2<'_> {
    pub fn set(self, a: &'static Combinator) {
        *self.b.borrow_mut() = Some(&a);
    }
}

impl CombinatorTrait<'_> for ForwardRef2<'_> {
    fn parse(&self, right_data: RightData, bytes: &[u8]) -> (Parser, ParseResults) {
        let binding = self.b.borrow();
        let a = binding.as_ref().unwrap();
        a.parse(right_data, bytes)
    }
}

impl<'a> From<&'a ForwardRef2<'a>> for Combinator<'_> {
    fn from(value: &'a ForwardRef2<'a>) -> Self {
        Combinator::ForwardRef2(value.clone())
    }
}

pub fn forward_ref2<'a>() -> ForwardRef2<'a> {
    ForwardRef2 { b: Rc::new(RefCell::new(None)) }
}

#[test]
fn test_forward_ref2_0() {
    fn make() -> Combinator<'static> {
        let mut a = forward_ref2();
        let a_inner = choice!(seq!(eat('a'), &a), eat('b'));
        a.set(&a_inner);
        a_inner
    }
}

#[test]
fn test_forward_ref2_1() {
    // should fail to compile :
    // error[E0597]: `a` does not live long enough
    //   --> src/combinators/forward_ref2.rs:52:15
    //    |
    // 51 |         let a: Combinator = eps().into();
    //    |             - binding `a` declared here
    // 52 |         f.set(&a);
    //    |         ------^^-
    //    |         |     |
    //    |         |     borrowed value does not live long enough
    //    |         argument requires that `a` is borrowed for `'static`
    // 53 |         f
    // 54 |     }
    //    |     - `a` dropped here while still borrowed
    //
//     fn make() -> Combinator {
//         let mut f = forward_ref2();
//         let a: Combinator = eps().into();
//         let b = seq!(eat('b'), &f);
//         f.set(&a);
//         b
//     }
}