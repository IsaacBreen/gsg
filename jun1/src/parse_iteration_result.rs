use std::ops::{BitOr, BitOrAssign};
use crate::gss::GSSNode;
use crate::u8set::U8Set;

#[derive(Clone, PartialEq, Debug)]
pub struct ParserIterationResult {
    u8set: U8Set,
    pub is_complete: bool,
    signals: Signals,
    pub node: Option<GSSNode<()>>,
}

impl ParserIterationResult {
    pub fn new(u8set: U8Set, is_complete: bool, signals: Signals) -> Self {
        Self { u8set, is_complete, signals, node: None }
    }

    pub fn u8set(&self) -> &U8Set {
        &self.u8set
    }

    pub fn signals(&self) -> &Signals {
        &self.signals
    }
}

impl BitOr for ParserIterationResult {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        let node = match (self.node, other.node) {
            (None, None) => None,
            (Some(node), None) => Some(node),
            (None, Some(node)) => Some(node),
            (Some(mut node), Some(other_node)) => {
                node.merge(other_node);
                Some(node)
            }
        };
        Self {
            u8set: self.u8set | other.u8set,
            is_complete: self.is_complete | other.is_complete,
            // signals: self.signals | other.signals,
            signals: other.signals,
            node
        }
    }
}

impl BitOrAssign for ParserIterationResult {
    fn bitor_assign(&mut self, other: Self) {
        *self = self.clone() | other;
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SignalAtom {
    usize(usize),
}

#[derive(PartialEq, Debug)]
pub struct Signal {
    origin_id: usize,
    id: usize,
    pub atoms: Vec<SignalAtom>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Signals {
    signals: Vec<Signal>,
}

impl Signal {
    pub fn push(&mut self, signal_atom: SignalAtom) {
        self.atoms.push(signal_atom);
    }
}

impl Clone for Signal {
    fn clone(&self) -> Self {
        Self {
            origin_id: self.origin_id,
            id: self.id,
            atoms: self.atoms.clone(),
        }
    }
}

impl Signals {
    pub fn push(&mut self, signal_atom: SignalAtom) {
        for signal in self.signals.iter_mut() {
            signal.push(signal_atom.clone());
        }
    }
}

impl BitOr for Signals {
    type Output = Signals;

    fn bitor(self, other: Self) -> Signals {
        Signals { signals: self.signals.into_iter().chain(other.signals).collect() }
    }
}

impl BitOrAssign for Signals {
    fn bitor_assign(&mut self, other: Self) {
        self.signals.extend(other.signals);
    }
}