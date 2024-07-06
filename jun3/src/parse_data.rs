use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use crate::FrameStack;

pub trait ParseDataField: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn merge(&self, other: &dyn ParseDataField) -> Box<dyn ParseDataField>;
    fn clone_box(&self) -> Box<dyn ParseDataField>;
    fn eq(&self, other: &dyn ParseDataField) -> bool;
}

#[derive(Default, Debug)]
pub struct ParseData {
    data: HashMap<String, Box<dyn ParseDataField>>,
}

impl PartialEq for ParseData {
    fn eq(&self, other: &Self) -> bool {
        for key in self.data.keys() {
            if !other.data.contains_key(key) {
                return false;
            }
            if !self.data[key].eq(other.data[key].as_ref()) {
                return false;
            }
        }
        true
    }
}

impl ParseData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: ParseDataField + 'static>(&mut self, key: String, value: T) {
        self.data.insert(key, Box::new(value));
    }

    pub fn get<T: ParseDataField + 'static>(&self, key: &str) -> Option<&T> {
        self.data.get(key)
            .and_then(|boxed| boxed.as_any().downcast_ref())
    }

    pub fn merge(&self, other: &Self) -> Self {
        let mut merged = self.clone();
        for (key, other_value) in &other.data {
            if let Some(self_value) = self.data.get(key) {
                let merged_value = self_value.merge(other_value.as_ref());
                merged.data.insert(key.clone(), merged_value);
            } else {
                merged.data.insert(key.clone(), other_value.clone_box());
            }
        }
        merged
    }
}

impl Clone for ParseData {
    fn clone(&self) -> Self {
        ParseData {
            data: self.data.iter().map(|(k, v)| (k.clone(), v.clone_box())).collect(),
        }
    }
}

impl ParseDataField for FrameStack {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn merge(&self, other: &dyn ParseDataField) -> Box<dyn ParseDataField> {
        if let Some(other_frame_stack) = other.as_any().downcast_ref::<FrameStack>() {
            // Implement actual merging logic here
            Box::new(self.clone())
        } else {
            Box::new(self.clone())
        }
    }

    fn clone_box(&self) -> Box<dyn ParseDataField> {
        Box::new(self.clone())
    }

    fn eq(&self, other: &dyn ParseDataField) -> bool {
        self.as_any().downcast_ref::<FrameStack>().unwrap() == other.as_any().downcast_ref::<FrameStack>().unwrap()
    }
}