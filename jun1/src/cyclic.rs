use std::ptr::NonNull;

#[derive(Clone)]
struct ForwardRef<A> {
    inner: Option<NonNull<A>>,
}

impl<A> ForwardRef<A> {
    fn new() -> Self {
        Self { inner: None }
    }

    fn set(&mut self, value: A) {
        let boxed = Box::new(value);
        self.inner = Some(NonNull::from(Box::leak(boxed)));
    }

    fn get(&self) -> &A {
        unsafe { self.inner.unwrap().as_ref() }
    }
}

struct Container<A> {
    a: ForwardRef<A>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_ref() {
        let mut a = ForwardRef::new();
        let a_final = Container { a: a.clone() };
        a.set(a_final);
    }
}