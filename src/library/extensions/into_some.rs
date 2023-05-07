pub trait IntoSome<T: Sized> {
    fn into_some(self) -> Option<T>;
}

impl<T> IntoSome<T> for T {
    fn into_some(self) -> Option<Self> {
        Some(self)
    }
}
