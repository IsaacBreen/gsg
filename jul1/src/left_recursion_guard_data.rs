use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::{CombinatorTrait, DynCombinator};

#[derive(Default, Clone)]
pub struct LeftRecursionGuardData {
    pub(crate) skip_on_this_nonterminal_or_fail_on_any_terminal: Option<*const u8>,
    pub(crate) fail_on_these_nonterminals: Vec<*const u8>,
}

impl Debug for LeftRecursionGuardData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeftRecursionGuardData").finish()
    }
}

impl PartialEq for LeftRecursionGuardData {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.skip_on_this_nonterminal_or_fail_on_any_terminal.as_ref().iter().zip(other.skip_on_this_nonterminal_or_fail_on_any_terminal.as_ref().iter()) {
            if std::ptr::eq(*a, *b) {
                continue
            }
            return false
        }
        for (a, b) in self.fail_on_these_nonterminals.iter().zip(other.fail_on_these_nonterminals.iter()) {
            if std::ptr::eq(*a, *b) {
                continue
            }
            return false
        }
        true
    }
}