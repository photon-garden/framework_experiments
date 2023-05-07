// This is a generic newtype used to work around Rust's orphan rule,
// which prevents us from implementing a trait from a library for
// a type from another library.
//
// We get around that limitation by wrapping the type in an instance
// of My. Then we can implement whatever trait for My<T> instead of T.
//
// More info here: https://blog.eizinger.io/8593/generic-newtypes-a-way-to-work-around-the-orphan-rule
pub struct My<T>(pub T);
