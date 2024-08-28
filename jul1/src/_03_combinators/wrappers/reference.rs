
// src/_03_combinators/wrappers/reference.rs
use crate::BaseCombinatorTrait;
use crate::*;
use once_cell::unsync::OnceCell;
use std::fmt::{Debug, Formatter};
// src/combinators/reference.rs
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

pub struct WeakRef<T> {
    pub inner: Weak<OnceCell<T>>,
}

pub struct StrongRef<T> {
    pub inner: Rc<OnceCell<T>>,
}

impl<T> Debug for WeakRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeakRef").finish_non_exhaustive()
    }
}

impl<T: Debug> Debug for StrongRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StrongRef").finish_non_exhaustive()
    }
}

impl<T> Clone for StrongRef<T> {
    fn clone(&self) -> Self {
        StrongRef {
            inner: self.inner.clone()
        }
    }
}

impl<T> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        WeakRef {
            inner: self.inner.clone()
        }
    }
}

impl<T> PartialEq for WeakRef<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.ptr_eq(&other.inner)
    }
}

impl<T> Eq for WeakRef<T> {}

impl<T> Hash for WeakRef<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self.inner, state);
    }
}

impl<T> PartialEq for StrongRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> Eq for StrongRef<T> {}

impl<T> Hash for StrongRef<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(&self.inner, state);
    }
}

impl<T: CombinatorTrait> DynCombinatorTrait for WeakRef<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait> CombinatorTrait for WeakRef<T> {
    type Parser<'a> = T::Parser<'a> where Self: 'a;
    type Output = T::Output;
    type PartialOutput = T::PartialOutput;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let combinator = self.get().unwrap();
        combinator.one_shot_parse(right_data, bytes)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        let combinator = self.get().unwrap();
        combinator.parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for WeakRef<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(self.inner.upgrade().expect("WeakRef is already dropped").get().expect("Combinator hasn't been set"));
    }
}

impl<T: CombinatorTrait> DynCombinatorTrait for StrongRef<T> {
    fn parse_dyn(&self, right_data: RightData, bytes: &[u8]) -> (Box<dyn ParserTrait + '_>, ParseResults) {
        let (parser, parse_results) = self.parse(right_data, bytes);
        (Box::new(parser), parse_results)
    }

    fn one_shot_parse_dyn<'a>(&'a self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        self.one_shot_parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait> CombinatorTrait for StrongRef<T> {
    type Parser<'a> = T::Parser<'a> where Self: 'a;
    type Output = T::Output;
    type PartialOutput = T::PartialOutput;

    fn one_shot_parse(&self, right_data: RightData, bytes: &[u8]) -> UnambiguousParseResults {
        let combinator = self.inner.get().unwrap();
        combinator.one_shot_parse(right_data, bytes)
    }

    fn old_parse(&self, right_data: RightData, bytes: &[u8]) -> (Self::Parser<'_>, ParseResults) {
        self.inner
            .get()
            .unwrap()
            .parse(right_data, bytes)
    }
}

impl<T: CombinatorTrait> BaseCombinatorTrait for StrongRef<T> {
    fn as_any(&self) -> &dyn std::any::Any where Self: 'static {
        self
    }
    fn apply_to_children(&self, f: &mut dyn FnMut(&dyn BaseCombinatorTrait)) {
        f(self.inner.get().unwrap());
    }
}

pub fn strong_ref<T>() -> StrongRef<T> {
    StrongRef {
        inner: Rc::new(OnceCell::new())
    }
}

impl<T> StrongRef<T> {
    pub fn set(&self, inner: T) {
        self.inner.set(inner).ok().expect("Cannot set value more than once");
    }

    pub fn downgrade(&self) -> WeakRef<T> {
        WeakRef {
            inner: Rc::downgrade(&self.inner)
        }
    }

    pub fn new(inner: T) -> Self {
        let cell = OnceCell::new();
        cell.set(inner).ok().expect("Cannot set value more than once");
        Self { inner: Rc::new(cell) }
    }
}

impl<T> WeakRef<T> {
    pub fn upgrade(&self) -> Option<StrongRef<T>> {
        self.inner.upgrade().map(|inner| StrongRef { inner })
    }

    pub fn get(&self) -> Option<&T> {
        // Upgrade the weak reference to a strong reference
        let strong_ref = self.inner.upgrade()?;

        // Safely access the inner Combinator
        // Note: We use unsafe code here to transmute the lifetime.
        // This is safe because the OnceCell guarantees that the value,
        // once set, will live as long as the Rc/Weak, which is 'static.
        unsafe {
            let combinator: &T = std::mem::transmute(strong_ref.get().unwrap());
            Some(combinator)
        }
    }
}