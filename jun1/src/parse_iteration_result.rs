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
    pub signals2: Signals2,
    pub frame_stack: FrameStack,
}

impl ParserIterationResult {
    pub fn new(u8set: U8Set, id_complete: Option<usize>, signals: Signals) -> Self {
        Self { u8set, id_complete, signals, node: None, signals2: Default::default(), frame_stack: Default::default() }
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

    pub fn signals2(&self) -> &Signals2 {
        &self.signals2
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
        Self {
            u8set: self.u8set | other.u8set,
            signals: self.signals | other.signals,
            node: None,
            id_complete,
            signals2: self.signals2 | other.signals2,
            frame_stack: self.frame_stack | other.frame_stack,
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
            signals2: self.signals2 | other.signals2,
            frame_stack: other.frame_stack,
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

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Signals2 {
    // prev id -> (next id, signal atom)
    pub(crate) signals: HashMap<usize, (usize, SignalAtom)>,
    finished_signal_ids: Vec<usize>,
}

impl Signals2 {
    pub fn new() -> Self {
        Self { signals: HashMap::new(), finished_signal_ids: Vec::new() }
    }

    pub fn push(&mut self, old_id: usize, new_id: usize, signal_atom: SignalAtom) {
        self.signals.insert(old_id, (new_id, signal_atom));
    }

    pub fn push_to_many(&mut self, old_ids: Vec<usize>, new_id: usize, signal_atom: SignalAtom) {
        for old_id in old_ids.iter() {
            self.signals.insert(*old_id, (new_id, signal_atom.clone()));
        }
    }

    pub fn push_to_finished(&mut self, new_id: usize, signal_atom: SignalAtom) {
        self.push_to_many(self.finished_signal_ids.clone(), new_id, signal_atom);
        self.finished_signal_ids = vec![new_id];
    }

    pub fn add_finished(&mut self, id: usize) {
        self.finished_signal_ids.push(id);
    }

    pub fn clear_finished(&mut self) {
        self.finished_signal_ids.clear();
    }

    pub fn merge(&mut self, other: Self) {
        for (old_id, (new_id, signal_atom)) in other.signals {
            assert!(!self.signals.contains_key(&old_id));
            self.signals.insert(old_id, (new_id, signal_atom));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.signals.is_empty()
    }
}

impl BitAnd for Signals2 {
    type Output = Signals2;

    fn bitand(self, other: Self) -> Signals2 {
        let mut signals = Signals2::new();
        for (old_id, (new_id, signal_atom)) in self.signals {
            if let Some((other_new_id, other_signal_atom)) = other.signals.get(&old_id) {
                assert_eq!(new_id, *other_new_id);
                signals.push(old_id, new_id, signal_atom.clone());
                signals.push(old_id, new_id, other_signal_atom.clone());
            }
        }
        signals
    }
}

impl BitOr for Signals2 {
    type Output = Signals2;

    fn bitor(self, other: Self) -> Signals2 {
        let mut signals = Signals2::new();
        for (old_id, (new_id, signal_atom)) in self.signals {
            signals.push(old_id, new_id, signal_atom);
        }
        for (old_id, (new_id, signal_atom)) in other.signals {
            signals.push(old_id, new_id, signal_atom);
        }
        signals
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FrameStack {
    frames: Vec<Frame>,
}

impl Default for FrameStack {
    fn default() -> Self {
        Self { frames: vec![Frame::default()] }
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Frame {
    pos: HashSet<String>,
    neg: HashSet<String>,
}

impl Frame {
    pub fn contains_prefix(&self, name_prefix: &str) -> bool {
        self.pos.iter().any(|name| name.starts_with(name_prefix))
    }

    pub fn excludes_prefix(&self, name_prefix: &str) -> bool {
        !self.contains_prefix(name_prefix) || self.neg.iter().any(|name| name.starts_with(name_prefix))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.pos.contains(name)
    }

    pub fn excludes(&self, name: &str) -> bool {
        !self.contains(name) || self.neg.contains(name)
    }

    pub fn next_u8_given_contains(&self, name: &[u8]) -> (U8Set, bool) {
        let mut u8set = U8Set::none();
        let mut is_complete = false;
        for existing_name in self.pos.iter() {
            let existing_name = existing_name.as_bytes();
            if name.len() <= existing_name.len() && existing_name[..name.len()] == *name {
                let next = existing_name[name.len()..].iter().copied().next();
                if let Some(next) = next {
                    u8set.insert(next);
                } else {
                    is_complete = true;
                }
            }
        }
        (u8set, is_complete)
    }

    pub fn next_u8_given_excludes(&self, name: &[u8]) -> (U8Set, bool) {
        todo!()
    }

    pub fn push_name(&mut self, name: &[u8]) {
        let name: &str = std::str::from_utf8(name).unwrap();
        assert!(!self.contains(&name));
        self.pos.insert(name.to_string());
    }

    pub fn pop_name(&mut self, name: &[u8]) {
        let name: &str = std::str::from_utf8(name).unwrap();
        assert!(self.contains(&name));
        self.pos.remove(&name.to_string());
    }
}

impl FrameStack {
    pub fn contains_prefix(&self, name_prefix: &str) -> bool {
        self.frames.iter().any(|frame| frame.contains_prefix(name_prefix))
    }

    pub fn excludes_prefix(&self, name_prefix: &str) -> bool {
        !self.contains_prefix(name_prefix) || self.frames.iter().any(|frame| frame.excludes_prefix(name_prefix))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.frames.iter().any(|frame| frame.contains(name))
    }

    pub fn excludes(&self, name: &str) -> bool {
        !self.contains(name) || self.frames.iter().any(|frame| frame.excludes(name))
    }

    pub fn next_u8_given_contains(&self, name: &[u8]) -> (U8Set, bool) {
        let mut u8set = U8Set::none();
        let mut is_complete = false;
        for frame in self.frames.iter().rev() {
            let (frame_u8set, frame_is_complete) = frame.next_u8_given_contains(name);
            u8set |= frame_u8set;
            is_complete |= frame_is_complete;
        }
        (u8set, is_complete)
    }

    pub fn next_u8_given_excludes(&self, name: &[u8]) -> (U8Set, bool) {
        todo!()
    }

    pub fn push_empty_frame(&mut self) {
        self.frames.push(Frame::default());
    }

    pub fn push_name(&mut self, name: &[u8]) {
        self.frames.last_mut().unwrap().push_name(name);
    }

    pub fn pop_name(&mut self, name: &[u8]) {
        self.frames.last_mut().unwrap().pop_name(name);
    }

    pub fn pop(&mut self) {
        self.frames.pop();
    }
}

impl BitOr for Frame {
    type Output = Frame;

    fn bitor(self, other: Self) -> Frame {
        Frame { pos: self.pos.union(&other.pos).cloned().collect(), neg: self.neg.union(&other.neg).cloned().collect() }
    }
}

impl BitOr for FrameStack {
    type Output = FrameStack;

    fn bitor(self, other: Self) -> FrameStack {
        // All except the last frames should be the same
        assert_eq!(self.frames.len(), other.frames.len());
        for (frame1, frame2) in self.frames.iter().zip(other.frames.iter()).rev().skip(1) {
            assert_eq!(frame1, frame2);
        }
        let mut frames = self.frames.clone();
        let last_frame = frames.last_mut().unwrap();
        *last_frame = last_frame.clone() | other.frames.last().unwrap().clone();
        FrameStack { frames }
    }
}