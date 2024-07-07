#[derive(Clone, PartialEq, Debug)]
pub struct IndentTrackers {
    indent_trackers: Vec<IndentTracker>
}

#[derive(Clone, PartialEq, Debug)]
pub struct IndentTracker {
    i: usize,
    indents: Vec<Vec<u8>>
}

impl Default for IndentTracker {
    fn default() -> Self {
        IndentTracker {
            i: 0,
            indents: vec![vec![]]
        }
    }
}

impl IndentTracker {
    pub fn push(&mut self) {
        self.indents.push(vec![]);
    }

    pub fn pop(&mut self) {
        self.indents.pop();
    }
}