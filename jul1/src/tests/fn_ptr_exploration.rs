use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[test]
fn test() {
    fn get_fn_ptr<F: Fn() + ?Sized>(f: &F) -> *const () {
        f as *const F as *const ()
    }

    fn f() {}

    let f_dyn_1: &dyn Fn() = &f;
    let f_dyn_2: &dyn Fn() = &f;

    // Succeeds
    assert!(std::ptr::eq(f_dyn_1, f_dyn_2));

    // ??
    assert_eq!(get_fn_ptr(f_dyn_1), get_fn_ptr(f_dyn_2));

    // Fails
    assert_eq!(std::ptr::addr_of!(f_dyn_1), std::ptr::addr_of!(f_dyn_2));

    let f1_dyn_addr_1 = &f_dyn_1 as *const _ as usize;
    let f2_dyn_addr_1 = &f_dyn_2 as *const _ as usize;

    // Fails
    assert_eq!(f1_dyn_addr_1, f2_dyn_addr_1);
}

#[test]
fn test2() {
    fn get_fn_ptr(f: &Rc<dyn Fn()>) -> *const () {
        // Get the raw pointer to the actual function
        let vtable_ptr = &**f as *const dyn Fn() as *const *const ();
        unsafe { *vtable_ptr }
    }

    fn hash_fn(f: &Rc<dyn Fn()>) -> u64 {
        let mut hasher = DefaultHasher::new();
        let ptr = get_fn_ptr(f);
        ptr.hash(&mut hasher);
        hasher.finish()
    }

    fn f() {}
    let f_rc_1: Rc<dyn Fn()> = Rc::new(f);
    let f_rc_2: Rc<dyn Fn()> = Rc::new(f);

    // Compare function pointers
    assert_eq!(get_fn_ptr(&f_rc_1), get_fn_ptr(&f_rc_2));

    // Hash function pointers
    assert_eq!(hash_fn(&f_rc_1), hash_fn(&f_rc_2));

    // You can also use the hash directly as a 'key'
    let key_1 = hash_fn(&f_rc_1);
    let key_2 = hash_fn(&f_rc_2);
    assert_eq!(key_1, key_2);

    // Demonstrate that Rc addresses are different
    assert_ne!(Rc::as_ptr(&f_rc_1) as *const (), Rc::as_ptr(&f_rc_2) as *const ());
}

#[test]
fn test_debug() {
    fn get_fn_ptr(f: &Rc<dyn Fn()>) -> *const () {
        let vtable_ptr = &**f as *const dyn Fn() as *const *const ();
        unsafe { *vtable_ptr }
    }

    fn f() {}
    let f_rc_1: Rc<dyn Fn()> = Rc::new(f);
    let f_rc_2: Rc<dyn Fn()> = Rc::new(f);

    println!("f_rc_1 address: {:p}", Rc::as_ptr(&f_rc_1));
    println!("f_rc_2 address: {:p}", Rc::as_ptr(&f_rc_2));

    let ptr1 = get_fn_ptr(&f_rc_1);
    let ptr2 = get_fn_ptr(&f_rc_2);

    println!("Function pointer 1: {:p}", ptr1);
    println!("Function pointer 2: {:p}", ptr2);

    assert_eq!(ptr1, ptr2, "Function pointers are not equal");
}

#[test]
fn test_direct_comparison() {
    fn get_fn_ptr(f: &Rc<dyn Fn()>) -> *const () {
        let vtable_ptr = &**f as *const dyn Fn() as *const *const ();
        unsafe { *vtable_ptr }
    }

    fn f() {}
    let f_ptr: fn() = f;
    let f_rc_1: Rc<dyn Fn()> = Rc::new(f);
    let f_rc_2: Rc<dyn Fn()> = Rc::new(f);

    println!("Direct function pointer: {:p}", f_ptr as *const ());
    println!("f_rc_1 data pointer: {:p}", Rc::as_ptr(&f_rc_1));
    println!("f_rc_2 data pointer: {:p}", Rc::as_ptr(&f_rc_2));

    // Compare with the actual function pointer
    assert_eq!(f_ptr as *const (), get_fn_ptr(&f_rc_1));
    assert_eq!(f_ptr as *const (), get_fn_ptr(&f_rc_2));
}

#[test]
fn test_simple_function() {
    fn get_fn_ptr(f: &Rc<dyn Fn()>) -> *const () {
        let vtable_ptr = &**f as *const dyn Fn() as *const *const ();
        unsafe { *vtable_ptr }
    }

    fn f() {}
    let f_rc_1: Rc<dyn Fn()> = Rc::new(f);
    let f_rc_2: Rc<dyn Fn()> = Rc::new(f);

    assert_eq!(get_fn_ptr(&f_rc_1), get_fn_ptr(&f_rc_2));
}
