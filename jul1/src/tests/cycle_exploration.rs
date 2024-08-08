use std::cell::RefCell;
use std::rc::{Rc, Weak};

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

    // Memory leak occurs here because `A` holds a strong reference to `B`
    // and `B` holds a strong reference back to `A`.
    let a = make_A();
    assert_eq!(Rc::strong_count(&a), 2); // Two strong references to `a`
    assert_eq!(Rc::weak_count(&a), 0);   // No weak references
}

#[test]
fn weak() {
    struct A {
        b: RefCell<Option<Rc<B>>>,
    }

    struct B {
        a: Weak<A>, // Use a weak reference to avoid cycle
    }

    fn make_A() -> Rc<A> {
        let a = Rc::new(A {
            b: RefCell::new(None),
        });
        let b = Rc::new(B {
            a: Rc::downgrade(&a),
        });
        a.b.replace(Some(b));
        a
    }

    let a = make_A();
    assert_eq!(Rc::strong_count(&a), 1); // One strong reference to `a`
    assert_eq!(Rc::weak_count(&a), 1);   // One weak reference to `a`
}

#[test]
fn careful_lifetimes() {
    struct A {
        b: RefCell<Option<Weak<B>>>,
    }

    struct B {
        a: Rc<A>,
    }

    fn make_A() -> Rc<A> {
        let a = Rc::new(A {
            b: RefCell::new(None),
        });
        let b = Rc::new(B {
            a: a.clone(),
        });
        a.b.replace(Some(Rc::downgrade(&b)));
        a
    }

    let a = make_A();
    assert_eq!(Rc::strong_count(&a), 1); // One strong reference to `a`
    assert_eq!(Rc::weak_count(&a), 0);   // No weak references to `a`

    // Check the strong and weak counts for `b`
    if let Some(weak_b) = a.b.borrow().as_ref() {
        if let Some(b) = weak_b.upgrade() {
            assert_eq!(Rc::strong_count(&b), 1); // One strong reference to `b`
            assert_eq!(Rc::weak_count(&b), 1);   // One weak reference to `b`
        } else {
            panic!("Failed to upgrade weak reference to `b`");
        }
    } else {
        panic!("`a.b` should contain a weak reference to `b`");
    };
}
