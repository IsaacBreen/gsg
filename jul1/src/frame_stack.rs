use std::collections::HashSet;

use crate::U8Set;

#[derive(Clone, PartialEq, Debug)]
pub struct FrameStack {
    frames: Vec<Frame>,
}

impl Default for FrameStack {
    fn default() -> Self {
        Self {
            frames: vec![Frame::default()],
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Eq)]
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
        self.pos.remove(name);
    }
}

impl FrameStack {
    pub fn contains_prefix_str(&self, name_prefix: &str) -> bool {
        self.frames.iter().any(|frame| frame.contains_prefix(name_prefix))
    }

    pub fn excludes_prefix_str(&self, name_prefix: &str) -> bool {
        !self.contains_prefix_str(name_prefix)
    }

    pub fn contains_str(&self, name: &str) -> bool {
        self.frames.iter().any(|frame| frame.contains(name))
    }

    pub fn excludes_str(&self, name: &str) -> bool {
        !self.contains_str(name)
    }

    pub fn next_u8_given_contains_u8slice(&self, name: &[u8]) -> (U8Set, bool) {
        let mut u8set = U8Set::none();
        let mut is_complete = false;
        for frame in self.frames.iter().rev() {
            let (frame_u8set, frame_is_complete) = frame.next_u8_given_contains(name);
            u8set |= frame_u8set;
            is_complete |= frame_is_complete;
        }
        (u8set, is_complete)
    }

    pub fn next_u8_given_excludes_slice(&self, name: &[u8]) -> (U8Set, bool) {
        let mut result_set = U8Set::all();
        let mut is_complete = true;

        for frame in self.frames.iter().rev() {
            let (frame_set, frame_complete) = frame.next_u8_given_excludes(name);
            result_set &= frame_set;
            is_complete &= frame_complete;
        }

        (result_set, is_complete)
    }

    pub fn contains_prefix_u8vec(&self, name_prefix: &[u8]) -> bool {
        self.contains_prefix_str(std::str::from_utf8(name_prefix).unwrap())
    }

    pub fn excludes_prefix_u8vec(&self, name_prefix: &[u8]) -> bool {
        self.excludes_prefix_str(std::str::from_utf8(name_prefix).unwrap())
    }

    pub fn contains_u8vec(&self, name: &[u8]) -> bool {
        self.contains_str(std::str::from_utf8(name).unwrap())
    }

    pub fn push_frame(&mut self, new_frame: Frame) {
        self.frames.push(new_frame);
    }

    pub fn push_empty_frame(&mut self) {
        self.push_frame(Frame::default());
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
        self.frames.retain(predicate);
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

impl std::ops::BitOr for Frame {
    type Output = Frame;

    fn bitor(self, other: Self) -> Frame {
        Frame { pos: self.pos.union(&other.pos).cloned().collect() }
    }
}

impl std::ops::BitOr for FrameStack {
    type Output = FrameStack;

    fn bitor(self, other: Self) -> FrameStack {
        let new_frames = self.frames.iter()
            .zip(other.frames.iter())
            .map(|(f1, f2)| f1.clone() | f2.clone())
            .collect();

        FrameStack {
            frames: new_frames,
        }
    }
}