use builder_pattern::Builder;
use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::marker::PhantomData;

#[allow(unused)]
#[derive(Builder)]
struct Op<T = f64> {
    #[infer(T)]
    #[default(None)]
    optional_field: Option<T>,
}

fn defaulted() {
    // Should be inferred as Op<f64>, i.e. the macro should notice the defaulted type param.
    let a = Op::new().build();
    assert_eq!(a.type_id(), TypeId::of::<Op<f64>>());
}

fn override_default() {
    // Should be inferred as Op<i32>
    let a = Op::new().optional_field(Some(5i32)).build();
    assert_eq!(a.type_id(), TypeId::of::<Op<i32>>());
}

#[allow(unused)]
#[derive(Builder)]
struct IterExtra<T, I = Vec<T>>
where
    I: IntoIterator<Item = T>,
{
    single: T,
    #[default(None)]
    extra: Option<I>,
}

fn inferred() {
    let a = IterExtra::new().single(1).build();
    assert_eq!(a.type_id(), TypeId::of::<IterExtra<i32, Vec<i32>>>());
}

#[allow(unused)]
#[derive(Builder)]
struct DefaultedClosure<T, R, F = fn(R, &T) -> R>
where
    F: for<'a> FnMut(R, &T) -> R,
{
    #[infer(F)]
    f: Option<F>,
    #[hidden]
    #[default(PhantomData)]
    phantom: PhantomData<(T, R)>,
}

trait Callable<T, R> {
    fn call_fn(&mut self, r: R, t: &T) -> R;
}
impl<T, R, F> Callable<T, R> for DefaultedClosure<T, R, F>
where
    F: for<'a> FnMut(R, &T) -> R,
{
    fn call_fn(&mut self, r: R, t: &T) -> R {
        if let Some(f) = &mut self.f {
            f(r, t)
        } else {
            r
        }
    }
}

fn infer_f_t() {
    let mut a = DefaultedClosure::new()
        .f(Some(|acc, x: &_| acc + x))
        .build();
    let called: i32 = a.call_fn(5, &5);
}

fn main() {
    defaulted();
    override_default();
    inferred();
    infer_f_t();
}
