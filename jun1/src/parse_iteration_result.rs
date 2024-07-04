use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, BitOrAssign};
use crate::gss::GSSNode;
use crate::u8set::U8Set;

#[derive(Clone, PartialEq, Debug)]
pub struct ParserIterationResult {
    pub u8set: U8Set,
    pub is_complete: bool,
    pub signals: Signals,
    pub node: Option<GSSNode<()>>,
}

impl ParserIterationResult {
    pub fn new(u8set: U8Set, is_complete: Option<usize>) -> Self {
        Self { u8set, is_complete: is_complete.is_some(), signals: Default::default(), node: None }
    }
}

impl ParserIterationResult {
    pub fn merge(self, mut other: Self) -> Self {
        let is_complete = self.is_complete || other.is_complete;
        // Merge the signal sets
        Self {
            u8set: self.u8set | other.u8set,
            signals: self.signals | other.signals,
            node: None,
            is_complete,
        }
    }

    pub fn merge_assign(&mut self, other: Self) {
        *self = self.clone().merge(other);
    }

    pub fn forward(self, other: Self) -> Self {
        let signals = self.signals & other.signals;
        Self {
            u8set: self.u8set | other.u8set,
            signals,
            node: None,
            is_complete: other.is_complete,
        }
    }

    pub fn forward_assign(&mut self, other: Self) {
        *self = self.clone().forward(other);
    }
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum SignalAtom {
    Start(usize),
    End(usize),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Signal {
    pub atoms: Vec<SignalAtom>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Signals {
    // state id -> signal
    pub(crate) signals: HashMap<usize, Signal>,
    // state id -> next state id
    pub(crate) next_state_ids: HashMap<usize, usize>,
    active_state_id: Option<usize>,
}

impl Signal {
    pub fn push(&mut self, state_id: usize, signal_atom: SignalAtom) {
        self.atoms.push(signal_atom);
    }
}

impl Signals {
    pub fn push(&mut self, state_id: usize, signal_atom: SignalAtom) {
        self.signals.entry(state_id).or_insert_with(|| Signal { atoms: Vec::new() }).push(state_id, signal_atom);
    }

    pub fn start_new_signal_group(&mut self) {
        self.active_state_id = Some(self.signals.len());
    }

    pub fn set_active_state_id(&mut self, state_id: usize) {
        self.active_state_id = Some(state_id);
    }

    pub fn add_finished(&mut self, state_id: usize) {
        if let Some(active_state_id) = self.active_state_id {
            self.next_state_ids.insert(active_state_id, state_id);
        }
        self.active_state_id = None;
    }

    pub fn is_empty(&self) -> bool {
        self.signals.is_empty()
    }
}

impl BitOr for Signals {
    type Output = Signals;

    fn bitor(self, other: Self) -> Signals {
        Signals { signals: self.signals.into_iter().chain(other.signals).collect(), next_state_ids: Default::default(), active_state_id: None }
    }
}

impl BitOrAssign for Signals {
    fn bitor_assign(&mut self, other: Self) {
        self.signals.extend(other.signals);
    }
}

impl BitAnd for Signals {
    type Output = Signals;

    fn bitand(self, other: Self) -> Signals {
        let ids = self.signals.keys().chain(other.signals.keys()).cloned().collect::<Vec<_>>();
        let mut signals = Signals::default();
        for id in ids.iter() {
            if self.signals.contains_key(id) && other.signals.contains_key(id) {
                let mut signal: Signal = self.signals[id].clone();
                signal.atoms.extend(other.signals[id].atoms.iter().cloned());
                signals.signals.insert(*id, signal);
            }
        }
        signals
    }
}