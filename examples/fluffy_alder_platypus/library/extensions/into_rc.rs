use std::rc::Rc;

pub trait IntoRc<T> {
    fn into_rc(self) -> Rc<T>;
}

impl<T> IntoRc<T> for T {
    fn into_rc(self) -> Rc<T> {
        Rc::new(self)
    }
}
