use std::fmt::{Debug, Formatter};

#[derive(Default, Clone)]
pub struct LeftRecursionGuardDownData {
    pub(crate) skip_on_this_nonterminal_or_fail_on_any_terminal: Option<*const u8>,
    pub(crate) fail_on_these_nonterminals: Vec<*const u8>,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct LeftRecursionGuardUpData {
    pub did_skip: bool,
}

impl LeftRecursionGuardDownData {
    pub fn may_consume(&self) -> bool {
        self.skip_on_this_nonterminal_or_fail_on_any_terminal.is_none()
    }

    pub fn on_consume(&mut self) {
        self.skip_on_this_nonterminal_or_fail_on_any_terminal = None;
        self.fail_on_these_nonterminals.clear();
    }

    pub fn did_consume(&self) -> bool {
        self.skip_on_this_nonterminal_or_fail_on_any_terminal.is_none() && self.fail_on_these_nonterminals.is_empty()
    }
}

impl Debug for LeftRecursionGuardDownData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LeftRecursionGuardData").finish()
    }
}

impl PartialEq for LeftRecursionGuardDownData {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.skip_on_this_nonterminal_or_fail_on_any_terminal.as_ref().iter().zip(other.skip_on_this_nonterminal_or_fail_on_any_terminal.as_ref().iter()) {
            if std::ptr::eq(*a, *b) {
                continue;
            }
            return false;
        }
        for (a, b) in self.fail_on_these_nonterminals.iter().zip(other.fail_on_these_nonterminals.iter()) {
            if std::ptr::eq(*a, *b) {
                continue;
            }
            return false;
        }
        true
    }
}