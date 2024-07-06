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

struct Wrapper<T>(T);

impl<T, A> Trait for Wrapper<T>
where
    T: Trait<Assoc = A>,
    A: AssocTrait + 'static,
{
    type Assoc = Box<dyn AssocTrait>;

    fn get_assoc(&self) -> Self::Assoc {
        Box::new(self.0.get_assoc())
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
    fn set<T, A>(&self, inner: T)
    where
        T: Trait<Assoc=A> + 'static,
        A: AssocTrait + 'static,
    {
        let boxed: Box<dyn Trait<Assoc=Box<dyn AssocTrait>>> = Box::new(Wrapper(inner));
        *self.inner.borrow_mut() = Some(boxed);
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

impl<T: Trait> Trait for Subtract1<T>
where
    T::Assoc: AssocTrait + 'static,
{
    type Assoc = Subtract1Assoc<T::Assoc>;
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

#[test]
fn test() {
    let forward_ref = ForwardRef::new();
    let subtract_1 = Subtract1 { inner: forward_ref.clone() };
    forward_ref.set(subtract_1);
    let assoc = forward_ref.get_assoc();
    println!("{}", assoc.foo(10));
}