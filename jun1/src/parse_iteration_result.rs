use std::collections::{HashMap, HashSet};
use std::ops::{BitAnd, BitOr};

use crate::u8set::U8Set;

#[derive(Clone, PartialEq, Debug)]
pub struct ParserIterationResult {
    pub u8set: U8Set,
    pub is_complete: bool,
    pub frame_stack: FrameStack,
}

impl ParserIterationResult {
    pub fn new(u8set: U8Set, is_complete: bool, frame_stack: FrameStack) -> Self {
        Self { u8set, is_complete, frame_stack }
    }

    pub fn u8set(&self) -> &U8Set {
        &self.u8set
    }
}

impl ParserIterationResult {
    pub fn merge(mut self, other: Self) -> Self {
        self.is_complete = self.is_complete || other.is_complete;
        // Merge the signal sets
        Self {
            u8set: self.u8set | other.u8set,
            is_complete: self.is_complete,
            frame_stack: self.frame_stack | other.frame_stack,
        }
    }

    pub fn merge_assign(&mut self, other: Self) {
        *self = self.clone().merge(other);
    }

    pub fn forward(self, other: Self) -> Self {
        Self {
            u8set: self.u8set | other.u8set,
            is_complete: other.is_complete,
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

// TODO:
//  - create a tree of frames rather than a vector. Each frame is a node. A frame can have multiple parents and multiple children.
//  - for filtering, traverse the tree and paint nodes that pass the filter green. Then paint any node
//    that is connected to a red node (parent, child, grandparent, grandchild etc.) blue.
//    Remove any unpainted nodes from the tree.
//  - remove the neg set altogether
//  - fix any affected methods, including the excludes methods.
#[derive(Clone, PartialEq, Debug)]
pub struct FrameStack {
    frames: Vec<Frame>,
    tree: HashMap<usize, FrameNode>,
    root_id: usize,
    next_id: usize,
}

impl Default for FrameStack {
    fn default() -> Self {
        let root_frame = Frame::default();
        let root_id = 0;
        let mut tree = HashMap::new();
        tree.insert(root_id, FrameNode {
            frame: root_frame,
            parent_ids: Vec::new(),
            child_ids: Vec::new(),
        });
        Self {
            frames: vec![Frame::default()],
            tree,
            root_id,
            next_id: 1,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct FrameNode {
    frame: Frame,
    parent_ids: Vec<usize>,
    child_ids: Vec<usize>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Frame {
    pos: HashSet<String>,
}

impl Frame {
    pub fn contains_prefix(&self, name_prefix: &str) -> bool {
        self.pos.iter().any(|name| name.starts_with(name_prefix))
    }

    pub fn excludes_prefix(&self, name_prefix: &str) -> bool {
        !self.contains_prefix(name_prefix)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.pos.contains(name)
    }

    pub fn excludes(&self, name: &str) -> bool {
        !self.contains(name)
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
        !self.contains_prefix(name_prefix)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.frames.iter().any(|frame| frame.contains(name))
    }

    pub fn excludes(&self, name: &str) -> bool {
        !self.contains(name)
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
        let mut result_set = U8Set::all();
        let mut is_complete = true;

        for frame in self.frames.iter().rev() {
            let (frame_set, frame_complete) = frame.next_u8_given_excludes(name);
            result_set &= frame_set;
            is_complete &= frame_complete;
        }

        (result_set, is_complete)
    }

    pub fn push_empty_frame(&mut self) {
        let new_frame = Frame::default();
        let new_id = self.next_id;
        self.next_id += 1;

        let parent_id = *self.tree.keys().max().unwrap();
        self.tree.get_mut(&parent_id).unwrap().child_ids.push(new_id);

        self.tree.insert(new_id, FrameNode {
            frame: new_frame.clone(),
            parent_ids: vec![parent_id],
            child_ids: Vec::new(),
        });

        self.frames.push(new_frame);
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

    pub fn filter(&mut self, predicate: impl Fn(&Frame) -> bool) {
        let mut to_remove = Vec::new();
        let mut to_keep = Vec::new();

        for (id, node) in self.tree.iter() {
            if predicate(&node.frame) {
                to_keep.push(*id);
            } else {
                to_remove.push(*id);
            }
        }

        for id in to_remove {
            if let Some(node) = self.tree.remove(&id) {
                for parent_id in node.parent_ids {
                    if let Some(parent) = self.tree.get_mut(&parent_id) {
                        parent.child_ids.retain(|&child_id| child_id != id);
                    }
                }
                for child_id in node.child_ids {
                    if let Some(child) = self.tree.get_mut(&child_id) {
                        child.parent_ids.retain(|&parent_id| parent_id != id);
                    }
                }
            }
        }

        self.frames = to_keep.into_iter()
            .filter_map(|id| self.tree.get(&id).map(|node| node.frame.clone()))
            .collect();
    }

    pub fn filter_contains(&mut self, name: &[u8]) {
        let name = std::str::from_utf8(name).unwrap();
        self.filter(|frame| frame.contains(name));
    }

    pub fn filter_excludes(&mut self, name: &[u8]) {
        let name = std::str::from_utf8(name).unwrap();
        self.filter(|frame| !frame.contains(name));
    }
}

impl BitOr for Frame {
    type Output = Frame;

    fn bitor(self, other: Self) -> Frame {
        Frame { pos: self.pos.union(&other.pos).cloned().collect() }
    }
}

impl BitOr for FrameStack {
    type Output = FrameStack;

    fn bitor(self, other: Self) -> FrameStack {
        // Merge the trees
        let mut new_tree = self.tree.clone();
        let mut id_map = HashMap::new();

        for (old_id, node) in other.tree {
            let new_id = self.next_id + old_id;
            id_map.insert(old_id, new_id);

            let mut new_node = node.clone();
            new_node.parent_ids = new_node.parent_ids.iter().map(|&id| id_map[&id]).collect();
            new_node.child_ids = new_node.child_ids.iter().map(|&id| id_map[&id]).collect();

            new_tree.insert(new_id, new_node);
        }

        // Merge the frames
        let new_frames = self.frames.iter()
            .zip(other.frames.iter())
            .map(|(f1, f2)| f1.clone() | f2.clone())
            .collect();

        FrameStack {
            frames: new_frames,
            tree: new_tree,
            root_id: self.root_id,
            next_id: self.next_id + other.next_id,
        }
    }
}
