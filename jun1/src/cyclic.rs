use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct ForwardRef<A> {
    inner: Option<Arc<Mutex<A>>>,
}

impl<A> ForwardRef<A> {
    fn new() -> Self {
        Self { inner: None }
    }

    fn set(&mut self, value: A) {
        self.inner = Some(Arc::new(Mutex::new(value)));
    }

    fn get(&self) -> Arc<Mutex<A>> {
        self.inner.as_ref().unwrap().clone()
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