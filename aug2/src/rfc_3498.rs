fn foo<'a>(x: &'a ()) -> impl Sized {
    x
}