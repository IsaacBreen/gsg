struct Foo {}
struct Bar<'a> {
    foo: &'a Foo,
}

fn foo<'a>(foo: &'a Foo) -> Bar<'a> {
    Bar { foo }
}

#[test]
fn test() {
    let x = Foo {};
    let y = foo(&x);
    drop(x);
    // y still has a reference to x
    // Shouldn't this cause a lifetime error?

    // this does cause an error, though...
    // let x = y.foo;

    // ...so I guess lifetime errors only occur if the compiler detects that you try to *use* a value that has been dropped,
    // BUT the compiler has a bunch of tricks to tell that a dangling reference won't be used, so it's not a problem.

    // Is there any way to opt out of this cleverness?
    // To say: "don't allow this reference to exist after it's value has been dropped, **even if the reference is never used**"

    // Possibly related:
    // - https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md#the-drop-check-rule
}