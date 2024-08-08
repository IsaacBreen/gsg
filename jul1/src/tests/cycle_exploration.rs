use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn memory_leak() {
    struct A {
        b: Rc<RefCell<Option<B>>>,
    }

    struct B {
        a: Rc<A>,
    }

    fn make_A() -> Rc<A> {
        let a = Rc::new(A {
            b: Rc::new(RefCell::new(None)),
        });
        let b = B {
            a: a.clone(),
        };
        a.b.replace(Some(b));
        a
    }
}

#[test]
fn weak() {
    todo!()
}

#[test]
fn careful_lifetimes() {
    // No weak, no memory leak, just careful lifetime management
    todo!()
}
