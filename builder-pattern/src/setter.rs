#[cfg(feature = "future")]
use futures::{future::LocalBoxFuture, Future};

pub enum Setter<'a, T> {
    Value(T),
    Lazy(Box<dyn 'a + Fn() -> T>),
    #[cfg(feature = "future")]
    Async(Box<dyn 'a + Fn() -> LocalBoxFuture<'a, T>>),
}

pub struct AsyncBuilderMarker {}
