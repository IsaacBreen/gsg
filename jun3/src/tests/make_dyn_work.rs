use std::cell::RefCell;
use std::rc::Rc;

// Define traits
trait AssocTrait {
    fn foo(&self, n: usize) -> usize;
}

trait Trait {
    type Assoc: AssocTrait;
    fn get_assoc(&self) -> &Self::Assoc;
}

// Define some dummy types
struct FooAssoc;
struct Foo;

impl AssocTrait for FooAssoc {
    fn foo(&self, n: usize) -> usize {
        n
    }
}
impl Trait for Foo {
    type Assoc = FooAssoc;
    fn get_assoc(&self) -> &Self::Assoc {
        &FooAssoc
    }
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

impl Trait for ForwardRef {
    type Assoc = dyn AssocTrait;
    fn get_assoc(&self) -> &Self::Assoc {
        self.inner.borrow().as_ref().unwrap().get_assoc()
    }
}

// Pair could contain anything, including user-defined types. We cannot hardcode it.
struct Pair<A, B> {
    a: A,
    b: B,
}
struct PairAssoc<AAssoc, BAssoc> {
    a: AAssoc,
    b: BAssoc,
}

impl<A: Trait<Assoc = AAssoc>, B: Trait<Assoc = BAssoc>, AAssoc: AssocTrait, BAssoc: AssocTrait> Pair<A, B> {
    fn get_assoc(&self) -> PairAssoc<&AAssoc, &BAssoc> {
        PairAssoc { a: self.a.get_assoc(), b: self.b.get_assoc() }
    }
}

fn main() {
    let mut forward_ref = ForwardRef::new();
    let a = Foo;
    let b = forward_ref.clone();
    let pair = Pair { a, b };
    let assoc = pair.get_assoc();
    assoc.foo();
}