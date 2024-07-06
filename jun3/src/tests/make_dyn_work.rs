use std::cell::RefCell;
use std::rc::Rc;

// Define traits
trait AssocTrait {
    fn foo(&self, n: usize) -> usize;
}

trait Trait {
    type Assoc: AssocTrait + ?Sized;
    fn get_assoc(&self) -> &Self::Assoc;
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
    fn set<T>(&self, value: T)
    where
        T: Trait<Assoc = dyn AssocTrait> + 'static,
    {
        *self.inner.borrow_mut() = Some(Box::new(value));
    }
}

impl Trait for ForwardRef {
    type Assoc = dyn AssocTrait;
    fn get_assoc(&self) -> &Self::Assoc {
        self.inner.borrow().as_ref().unwrap().get_assoc()
    }
}

struct Subtract1<T> {
    inner: T,
}

struct Subtract1Assoc<T: ?Sized> {
    inner: T,
}

impl<T> Trait for Subtract1<T>
where
    T: Trait<Assoc = dyn AssocTrait>,
{
    type Assoc = Subtract1Assoc<T::Assoc>;
    fn get_assoc(&self) -> &Self::Assoc {
        let inner_assoc: &T::Assoc = self.inner.get_assoc();
        // We need to box the inner Assoc to handle the dynamically sized type
        Box::leak(Box::new(Subtract1Assoc { inner: Box::new(inner_assoc) }))
    }
}

impl<Assoc> AssocTrait for Subtract1Assoc<Assoc>
where
    Assoc: AssocTrait + ?Sized,
{
    fn foo(&self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            self.inner.foo(n - 1)
        }
    }
}

fn main() {
    let forward_ref = ForwardRef::new();
    let subtract_1 = Subtract1 { inner: forward_ref.clone() };
    forward_ref.set(subtract_1);
    let assoc = forward_ref.get_assoc();
    println!("{}", assoc.foo(10));
}