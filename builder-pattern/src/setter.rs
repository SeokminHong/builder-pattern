#[cfg(feature = "future")]
use futures::future::LocalBoxFuture;

use super::refl::Id;

pub enum Setter<'a, T, D = T> {
    // Initially, T = D.
    // If you set a value with an #[infer(T)] setter,
    // then T gets replaced with an inferred type, and we
    // no longer _know_ that T = D. It could still be.
    // But we will store a Setter::Value, so the Id<T, D> is gone.
    Default(T, Id<T, D>),
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
