use std::pin::Pin;
use std::marker::PhantomPinned;

struct ForwardRef<T> {
    inner: Option<Pin<Box<T>>>,
    _pin: PhantomPinned,
}

impl<T> ForwardRef<T> {
    fn new() -> Pin<Box<Self>> {
        Box::pin(Self {
            inner: None,
            _pin: PhantomPinned,
        })
    }

    fn set(self: Pin<&mut Self>, value: Pin<Box<T>>) {
        // SAFETY: We're not moving any pinned fields
        unsafe {
            self.get_unchecked_mut().inner = Some(value);
        }
    }

    fn get(self: Pin<&Self>) -> Option<&T> {
        // SAFETY: We're not moving the pinned value
        self.inner.as_ref().map(|pinned| pinned.as_ref())
    }
}

struct Container<T> {
    a: ForwardRef<T>,
    _pin: PhantomPinned,
}

impl<T> Container<T> {
    fn new(a: ForwardRef<T>) -> Pin<Box<Self>> {
        Box::pin(Self {
            a,
            _pin: PhantomPinned,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyclic_reference() {
        let mut a = ForwardRef::new();
        let mut container = Container::new(ForwardRef::new());

        // Set the value (still requires careful handling)
        let pinned_container = container.as_mut();
        a.as_mut().set(Box::pin(pinned_container));

        // Accessing the value
        let retrieved_container = a.get();
        // ...
    }
}