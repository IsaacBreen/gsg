use std::cell::RefCell;
use std::rc::Rc;

// Define traits
trait AssocTrait {}

trait Trait {
    type Assoc: AssocTrait;
}

// Forward references are useful for recursive definitions.
#[derive(Clone)]
struct ForwardRef {
    inner: Rc<RefCell<Option<Box<dyn Trait<Assoc = dyn AssocTrait>>>>>,
}

impl ForwardRef {
    fn new() -> Self {
        Self { inner: Rc::new(RefCell::new(None)) }
    }

    // Fill the inner field with the value.
    fn set<T: Trait<Assoc = Assoc>, Assoc: AssocTrait>(&self, value: T) {
        *self.inner.borrow_mut() = Some(Box::new(value));
    }
}

// Container could be anything, including a user-defined type. We cannot hardcode it.
struct Container<T> {
    inner: T,
}

fn main() {
    let mut forward_ref = ForwardRef::new();
    let inner = Box::new(Container { inner: forward_ref.clone() });
    forward_ref.set(inner);
}