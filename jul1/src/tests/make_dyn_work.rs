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
impl Trait for Rc<dyn Trait<Assoc=Box<dyn AssocTrait>>> {
    type Assoc = Box<dyn AssocTrait>;
    fn get_assoc(&self) -> Self::Assoc {
        (**self).get_assoc()
    }
}

struct Wrapper<T>(T);

impl<T, A> Trait for Wrapper<T>
where
    T: Trait<Assoc=A>,
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
    inner: Rc<RefCell<Option<Rc<dyn Trait<Assoc=Box<dyn AssocTrait>>>>>>,
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
        let boxed: Rc<dyn Trait<Assoc=Box<dyn AssocTrait>>> = Rc::new(Wrapper(inner));
        *self.inner.borrow_mut() = Some(boxed);
    }
}

// New wrapper to defer the call to get_assoc
struct DeferredAssoc<T: Trait> {
    inner: Rc<RefCell<Option<T::Assoc>>>,
    trait_obj: T,
}

impl<T: Trait> AssocTrait for DeferredAssoc<T>
where
    T::Assoc: AssocTrait,
{
    fn foo(&self, n: usize) -> usize {
        let mut inner = self.inner.borrow_mut();
        if inner.is_none() {
            *inner = Some(self.trait_obj.get_assoc());
        }
        inner.as_ref().unwrap().foo(n)
    }
}

impl Trait for ForwardRef {
    type Assoc = Box<dyn AssocTrait>;
    fn get_assoc(&self) -> Self::Assoc {
        Box::new(DeferredAssoc {
            inner: Rc::new(RefCell::new(None)),
            trait_obj: <Option<Rc<dyn Trait<Assoc=Box<dyn AssocTrait>>>> as Clone>::clone(&self.inner.borrow()).unwrap().clone(),
        })
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