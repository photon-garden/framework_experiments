pub trait PipeIf<T> {
    fn pipe_if<F>(self, condition: bool, call_if_true: F) -> T
    where
        F: Fn(T) -> T;
}

impl<T> PipeIf<T> for T {
    fn pipe_if<F>(self, condition: bool, call_if_true: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if condition {
            self
        } else {
            call_if_true(self)
        }
    }
}
