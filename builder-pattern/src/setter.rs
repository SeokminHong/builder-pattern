#[cfg(feature = "future")]
use futures::future::LocalBoxFuture;

pub enum Setter<'a, T> {
    Value(T),
    Lazy(Box<dyn 'a + FnOnce() -> T>),
    #[cfg(feature = "future")]
    Async(Box<dyn 'a + FnOnce() -> LocalBoxFuture<'a, T>>),
}

pub enum ValidatedSetter<'a, T> {
    Value(T),
    Lazy(Box<dyn 'a + FnOnce() -> Result<T, &'static str>>),
    #[cfg(feature = "future")]
    Async(Box<dyn 'a + FnOnce() -> LocalBoxFuture<'a, Result<T, &'static str>>>),
}

pub struct AsyncBuilderMarker {}
