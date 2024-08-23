struct Foo {}
struct Bar<'a> {
    foo: &'a Foo,
}
impl Drop for Bar<'_> {
    fn drop(&mut self) {}
}

fn foo<'a>(foo: &'a Foo) -> Bar<'a> {
    Bar { foo }
}

#[test]
fn test() {
    let x = Foo {};
    let y = foo(&x);
    drop(x);
    // *Now* this causes an error. Why??
}