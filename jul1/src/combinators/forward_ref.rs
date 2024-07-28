use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use serde::{Serialize, Deserialize};

use crate::{Combinator, CombinatorTrait, Parser, ParseResults, ParserTrait, Stats};
use crate::parse_state::RightData;

#[derive(Debug, Clone)]
pub struct ForwardRef {
    a: Rc<RefCell<Option<Rc<Combinator>>>>,
    id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableForwardRef {
    id: usize,
}

impl Serialize for ForwardRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializableForwardRef { id: self.id }.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ForwardRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let SerializableForwardRef { id } = SerializableForwardRef::deserialize(deserializer)?;
        Ok(ForwardRef {
            a: Rc::new(RefCell::new(None)),
            id,
        })
    }
}

impl Hash for ForwardRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.a).hash(state);
    }
}

impl PartialEq for ForwardRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.a, &other.a)
    }
}

impl Eq for ForwardRef {}

impl CombinatorTrait for ForwardRef {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        self.a.borrow().as_ref().unwrap().parser(right_data)
    }
}

impl CombinatorTrait for RefCell<Option<Rc<Combinator>>> {
    fn parser(&self, right_data: RightData) -> (Parser, ParseResults) {
        self.borrow().as_ref().unwrap().parser(right_data)
    }
}

pub fn forward_ref() -> Combinator {
    Combinator::ForwardRef(ForwardRef { a: Rc::new(RefCell::new(None)) })
}

impl ForwardRef {
    pub fn set(&mut self, a: Combinator) -> Combinator {
        let a = Rc::new(a);
        *self.a.borrow_mut() = Some(a.clone());
        Combinator::ForwardRef(self.clone())
    }
}

#[macro_export]
macro_rules! forward_decls {
    ($($name:ident),* $(,)?) => {
        $(
            let mut $name = forward_ref();
        )*
    };
}
