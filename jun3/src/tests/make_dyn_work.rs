use std::cell::RefCell;
use std::rc::Rc;

// Define traits
trait AssocTrait {
    fn foo(&self, n: usize) -> usize;
}

trait Trait {
    type Assoc: AssocTrait;
    fn get_assoc(&self) -> Self::Assoc;
}

// Implement it for boxed dyn traits
impl AssocTrait for Box<dyn AssocTrait> {
    fn foo(&self, n: usize) -> usize {
        (**self).foo(n)
    }
}

// Forward references are useful for recursive definitions.
#[derive(Clone)]
struct ForwardRef {
    inner: Rc<RefCell<Option<Box<dyn Trait<Assoc = Box<dyn AssocTrait>>>>>>,
}

impl ForwardRef {
    fn new() -> Self {
        Self { inner: Rc::new(RefCell::new(None)) }
    }

    // Fill the inner field with the value.
    fn set<T: Trait<Assoc = Assoc>, Assoc: AssocTrait>(&self, inner: T) {
        todo!()
    }
}

impl Trait for ForwardRef {
    type Assoc = Box<dyn AssocTrait>;
    fn get_assoc(&self) -> Self::Assoc {
        self.inner.borrow().as_ref().unwrap().get_assoc()
    }
}

struct Subtract1<T> {
    inner: T,
}
struct Subtract1Assoc<T> {
    inner: T,
}

impl<T: Trait<Assoc = Assoc>, Assoc: AssocTrait + 'static> Trait for Subtract1<T> {
    type Assoc = Subtract1Assoc<Assoc>;
    fn get_assoc(&self) -> Self::Assoc {
        Subtract1Assoc { inner: self.inner.get_assoc() }
    }
}

impl<Assoc: AssocTrait> AssocTrait for Subtract1Assoc<Assoc> {
    fn foo(&self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            self.inner.foo(n - 1)
        }
    }
}

fn main() {
    let mut forward_ref = ForwardRef::new();
    let subtract_1 = Subtract1 { inner: forward_ref.clone() };
    forward_ref.set(subtract_1);
    let assoc = forward_ref.get_assoc();
    println!("{}", assoc.foo(10));
}