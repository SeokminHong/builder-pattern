#[cfg(feature = "future")]
use futures::future::LocalBoxFuture;

use super::refl::Id;

pub enum Setter<'a, T, D = T> {
    Default(D, Id<D, T>),
    LateBoundDefault(Id<D, T>),
    Value(T),
    Lazy(Box<dyn 'a + FnOnce() -> T>),
    LazyValidated(Box<dyn 'a + FnOnce() -> Result<T, &'static str>>),
    #[cfg(feature = "future")]
    Async(Box<dyn 'a + FnOnce() -> LocalBoxFuture<'a, T>>),
    #[cfg(feature = "future")]
    AsyncValidated(Box<dyn 'a + FnOnce() -> LocalBoxFuture<'a, Result<T, &'static str>>>),
}

pub struct AsyncBuilderMarker {}

pub struct HavingLazyValidator {}
