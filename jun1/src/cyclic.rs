#[derive(Clone)]
struct ForwardRef<A> {
    inner: Option<A>,
}

impl<A> ForwardRef<A> {
    fn new() -> Self {
        Self {
            inner: None,
        }
    }

    fn set(&mut self, value: A) {
        self.inner = Some(value);
    }

    fn get(&self) -> &A {
        self.inner.as_ref().unwrap()
    }
}

struct Container<A> {
    a: A,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_ref() {
        let mut A = ForwardRef::new();
        let A_final = Container { a: A.clone() };
        A.set(A_final);
    }
}