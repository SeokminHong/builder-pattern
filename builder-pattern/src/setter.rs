extern crate alloc;

use core::future::Future;
use core::pin::Pin;

pub type LocalBoxFuture<'a, T> = Pin<alloc::boxed::Box<dyn Future<Output = T> + 'a>>;

pub enum Setter<'a, T> {
    Value(T),
    Lazy(Box<dyn 'a + FnOnce() -> T>),
    LazyValidated(Box<dyn 'a + FnOnce() -> Result<T, &'static str>>),
    Async(Box<dyn 'a + FnOnce() -> LocalBoxFuture<'a, T>>),
    AsyncValidated(Box<dyn 'a + FnOnce() -> LocalBoxFuture<'a, Result<T, &'static str>>>),
}

pub struct AsyncBuilderMarker {}

pub struct HavingLazyValidator {}
