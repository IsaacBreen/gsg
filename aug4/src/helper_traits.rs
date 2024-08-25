use std::any::Any;

pub trait AsAny {
    fn as_any<'a>(&'a self) -> &(dyn Any + 'a);
}

impl<T: AsAny + ?Sized> AsAny for Box<T> {
    fn as_any<'a>(&'a self) -> &(dyn Any + 'a) { self }
}

impl<T: AsAny + ?Sized> AsAny for &T {
    fn as_any<'a>(&'a self) -> &(dyn Any + 'a) { self.as_any() }
}