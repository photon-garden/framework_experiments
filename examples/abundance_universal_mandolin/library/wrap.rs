pub trait Wrap<T: Sized> {
    fn wrap<Wrapper, Wrapped>(self, wrapper: Wrapper) -> Wrapped
    where
        Wrapper: Fn(T) -> Wrapped;
}

impl<T> Wrap<T> for T {
    fn wrap<Wrapper, Wrapped>(self, wrapper: Wrapper) -> Wrapped
    where
        Wrapper: Fn(T) -> Wrapped,
    {
        wrapper(self)
    }
}
