pub fn indent(indent: u8) -> impl Fn(u8) -> u8 {
    move |n| n + indent
}