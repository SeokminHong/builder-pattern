//! Inspired by type lists implementation from
//! https://stackoverflow.com/questions/40219725/constructing-hetereogenous-type-lists-in-rust
pub struct Nil;
pub struct Cons<T>(T);

pub trait Append: Sized {
    type Out;
}

impl Append for Nil {
    type Out = Cons<Nil>;
}

impl<T> Append for Cons<T>
where
    T: Append,
{
    type Out = Cons<<T as Append>::Out>;
}
