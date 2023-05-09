//! https://github.com/Centril/refl
//!
//! Used under the MIT license. Just the basics.

use core::marker::PhantomData;
use core::mem;

///
/// ```compile_fail
/// use builder_pattern::refl::Id;
/// let id = Id::<String, Vec<i32>>::REFL;
/// ```
///
/// ```
/// use builder_pattern::refl::{refl, Id};
/// fn get_i32<T>(t: T, id: Id<T, i32>) -> i32 {
///     id.cast(t)
/// }
/// let five = get_i32(5, refl());
/// assert_eq!(five, 5i32);
/// ```
///
pub struct Id<S: ?Sized, T: ?Sized>(PhantomData<(fn(S) -> S, fn(T) -> T)>);

impl<T: ?Sized> Id<T, T> {
    pub const REFL: Self = Id(PhantomData);
}

pub fn refl<T: ?Sized>() -> Id<T, T> {
    Id::REFL
}

impl<S: ?Sized, T: ?Sized> Id<S, T> {
    /// Casts a value of type `S` to `T`.
    ///
    /// This is safe because the `Id` type is always guaranteed to
    /// only be inhabited by `Id<T, T>` types by construction.
    pub fn cast(self, value: S) -> T
    where
        S: Sized,
        T: Sized,
    {
        unsafe {
            // Transmute the value;
            // This is safe since we know by construction that
            // S == T (including lifetime invariance) always holds.
            let cast_value = mem::transmute_copy(&value);

            // Forget the value;
            // otherwise the destructor of S would be run.
            mem::forget(value);

            cast_value
        }
    }
    /// Converts `Id<S, T>` into `Id<T, S>` since type equality is symmetric.
    pub fn sym(self) -> Id<T, S> {
        Id(PhantomData)
    }
}
