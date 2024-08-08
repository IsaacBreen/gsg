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
    struct A<'a> {
        b: RefCell<Option<&'a B<'a>>>,
    }

    struct B<'a> {
        a: &'a A<'a>,
    }

    fn make_A<'a>() -> A<'a> {
        let a = A {
            b: RefCell::new(None),
        };
        a
    }

    fn make_B<'a>(a: &'a A<'a>) -> B<'a> {
        B { a }
    }

    let a = make_A();
    let b = make_B(&a);
    a.b.replace(Some(&b));

    // No strong or weak references to count, but we can check the structure
    assert!(a.b.borrow().is_some());
}