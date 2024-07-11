use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::DynCombinator;

#[derive(Default, Clone)]
pub struct LeftRecursionGuardData {
    pub(crate) to_fail: Option<Rc<DynCombinator>>,
    pub(crate) to_pass: Vec<Rc<DynCombinator>>,
}

impl Debug for LeftRecursionGuardData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PartialEq for LeftRecursionGuardData {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.to_fail.as_ref().iter().zip(other.to_fail.as_ref().iter()) {
            if std::ptr::eq(a.as_ref(), b.as_ref()) {
                continue
            }
            return false
        }
        for (a, b) in self.to_pass.iter().zip(other.to_pass.iter()) {
            if std::ptr::eq(a.as_ref(), b.as_ref()) {
                continue
            }
            return false
        }
        true
    }
}