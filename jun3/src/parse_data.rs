use std::any::Any;
use std::fmt::Debug;
use crate::FrameStack;

pub trait ParseDataComponentDyn: Debug {
    fn clone_box(&self) -> Box<dyn ParseDataComponentDyn> {
        unimplemented!()
    }
    fn merge(&self, other: &dyn ParseDataComponentDyn) -> Box<dyn ParseDataComponentDyn> {
        unimplemented!()
    }
}

impl PartialEq for Box<dyn ParseDataComponentDyn> {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
    }
}

impl ParseDataComponentDyn for Box<dyn ParseDataComponentDyn> {}

pub trait ParseDataComponent: ParseDataComponentDyn {
    fn merge(self, other: Self) -> Self;
}

impl Clone for Box<dyn ParseDataComponentDyn> {
    fn clone(&self) -> Box<dyn ParseDataComponentDyn> {
        self.clone_box()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct ParseData {
    components: Vec<Box<dyn ParseDataComponentDyn>>,
}

impl ParseData {
    pub fn new(frame_stack: FrameStack) -> Self {
        let mut components: Vec<Box<dyn ParseDataComponentDyn>> = Vec::new();
        components.push(Box::new(frame_stack));
        Self { components }
    }

    pub fn add_component(&mut self, component: Box<dyn ParseDataComponentDyn>) {
        self.components.push(component);
    }

    pub fn merge(self, other: Self) -> Self {
        let mut new_components = self.components.clone();
        for comp in &other.components {
            // Find a matching type component to merge
            let mut merged = false;
            for new_comp in new_components.iter() {
                if new_comp.type_id() == comp.type_id() {
                    // *new_comp = new_comp.merge(comp);
                    merged = true;
                    break;
                }
            }
            if !merged {
                new_components.push(comp.clone());
            }
        }

        Self {
            components: new_components,
        }
    }
}

impl Clone for ParseData {
    fn clone(&self) -> Self {
        Self {
            components: self.components.clone(),
        }
    }
}

impl ParseDataComponentDyn for FrameStack {}

impl ParseDataComponent for FrameStack {
    fn merge(self, other: Self) -> Self {
        self | other
    }
}
