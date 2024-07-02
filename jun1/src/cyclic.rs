use std::pin::Pin;
use std::marker::PhantomPinned;

pub struct ForwardRef<T> {
    inner: Option<Pin<Box<T>>>,
    _pin: PhantomPinned,
}

impl<T> ForwardRef<T> {
    pub fn new() -> Pin<Box<Self>> {
        Box::pin(Self {
            inner: None,
            _pin: PhantomPinned,
        })
    }

    pub fn set(self: Pin<&mut Self>, value: T) {
        // SAFETY: We're not moving any pinned fields
        let this = unsafe { self.get_unchecked_mut() };
        this.inner = Some(Box::pin(value));
    }

    pub fn get(self: Pin<&Self>) -> Option<&T> {
        // SAFETY: We're not moving the pinned value
        self.inner.as_ref().map(|pinned| pinned.as_ref())
    }
}

pub struct Container<T> {
    a: Pin<Box<ForwardRef<T>>>,
    _pin: PhantomPinned,
}

impl<T> Container<T> {
    pub fn new() -> Pin<Box<Self>> {
        Box::pin(Self {
            a: ForwardRef::new(),
            _pin: PhantomPinned,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_ref() {
        let mut container = Container::new();
        let forward_ref = container.as_mut().a.as_mut();

        // Now we can set the value without causing infinite size issues
        forward_ref.set(42);

        assert_eq!(container.a.as_ref().get(), Some(&42));
    }

    #[test]
    fn test_cyclic_reference() {
        let mut container = Container::<Container<()>>::new();
        let forward_ref = container.as_mut().a.as_mut();

        // Create a cyclic reference
        forward_ref.set(Container::new());

        assert!(container.a.as_ref().get().is_some());
    }
}