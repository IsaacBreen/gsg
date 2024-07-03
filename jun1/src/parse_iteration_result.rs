use std::collections::{HashMap, HashSet};
use std::ops::{BitAnd, BitOr, BitOrAssign};
use crate::gss::GSSNode;
use crate::u8set::U8Set;

#[derive(Clone, PartialEq, Debug)]
pub struct ParserIterationResult {
    pub u8set: U8Set,
    pub id_complete: Option<usize>,
    pub signals: Signals,
    pub node: Option<GSSNode<()>>,
}

impl ParserIterationResult {
    pub fn new(u8set: U8Set, id_complete: Option<usize>, signals: Signals) -> Self {
        Self { u8set, id_complete, signals, node: None }
    }

    pub fn u8set(&self) -> &U8Set {
        &self.u8set
    }

    pub fn is_complete(&self) -> bool {
        self.id_complete.is_some()
    }

    pub fn signals(&self) -> &Signals {
        &self.signals
    }
}

impl ParserIterationResult {
    pub fn merge(self, mut other: Self) -> Self {
        let id_complete = match (self.id_complete, other.id_complete) {
            (None, None) => None,
            (Some(id_complete), None) => Some(id_complete),
            (None, Some(id_complete)) => Some(id_complete),
            (Some(id_complete), Some(other_id_complete)) if id_complete == other_id_complete => {
                Some(id_complete)
            }
            (Some(id_complete), Some(other_id_complete)) => {
                // Merge
                other.signals.merges.insert(id_complete, other_id_complete);
                Some(other_id_complete)
            }
        };
        // Merge the signal sets
        let signals = self.signals | other.signals;
        Self {
            u8set: self.u8set | other.u8set,
            signals,
            node: None,
            id_complete,
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
            id_complete: other.id_complete,
        }
    }

    pub fn forward_assign(&mut self, other: Self) {
        *self = self.clone().forward(other);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SignalAtom {
    usize(usize),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Signal {
    pub atoms: Vec<SignalAtom>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Signals {
    signals: HashMap<usize, Signal>,
    merges: HashMap<usize, usize>,
}

impl Signal {
    pub fn push(&mut self, signal_atom: SignalAtom) {
        self.atoms.push(signal_atom);
    }
}

impl Signals {
    pub fn push(&mut self, signal_atom: SignalAtom) {
        self.signals.entry(self.signals.len()).or_insert_with(|| Signal { atoms: Vec::new() }).push(signal_atom);
    }
}

impl BitOr for Signals {
    type Output = Signals;

    fn bitor(self, other: Self) -> Signals {
        Signals { signals: self.signals.into_iter().chain(other.signals).collect(), merges: self.merges.into_iter().chain(other.merges).collect() }
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
        let ids = self.signals.keys().chain(other.signals.keys()).cloned().collect::<HashSet<_>>();
        let mut signals = Signals::default();
        for id in ids.iter() {
            if self.signals.contains_key(id) && other.signals.contains_key(id) {
                let mut signal: Signal = self.signals[id].clone();
                signal.atoms.extend(other.signals[id].atoms.iter().cloned());
                signals.signals.insert(*id, signal);
            } else if self.signals.contains_key(id) {
                signals.signals.insert(*id, self.signals[id].clone());
            } else if other.signals.contains_key(id) {
                signals.signals.insert(*id, other.signals[id].clone());
            }
        }
        signals.merges.extend(self.merges.iter());
        signals.merges.extend(other.merges.iter());
        signals
    }
}