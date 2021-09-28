#[cfg(feature = "future")]
use futures::future::LocalBoxFuture;

pub enum Setter<'a, T> {
    Value(T),
    Lazy(Box<dyn 'a + Fn() -> T>),
    #[cfg(feature = "future")]
    Async(Box<dyn 'a + Fn() -> LocalBoxFuture<'a, T>>),
}

pub enum ValidatedSetter<'a, T> {
    Value(T),
    Lazy(Box<dyn 'a + Fn() -> Result<T, &'static str>>),
    #[cfg(feature = "future")]
    Async(Box<dyn 'a + Fn() -> LocalBoxFuture<'a, Result<T, &'static str>>>),
}

pub struct AsyncBuilderMarker {}
