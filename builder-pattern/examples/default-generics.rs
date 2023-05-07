use builder_pattern::Builder;
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::ops::Add;

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
struct DefaultedClosure<F1, T, R, F2 = fn(R, &T) -> R>
where
    F1: for<'a> FnMut(R, &T) -> R,
    F2: for<'a> FnMut(R, &T) -> R,
{
    mandatory: F1,
    #[infer(F2)]
    #[late_bound_default]
    #[default(|r, _t| r)]
    optional: F2,
    #[hidden]
    #[default(PhantomData)]
    phantom: PhantomData<(T, R)>,
}

trait Callable<T, R> {
    fn call_fn(&mut self, r: R, t: &T) -> R;
    fn call_inverse(&mut self, r: R, t: &T) -> R;
}
impl<F1, T, R, F2> Callable<T, R> for DefaultedClosure<F1, T, R, F2>
where
    F1: for<'a> FnMut(R, &T) -> R,
    F2: for<'a> FnMut(R, &T) -> R,
{
    fn call_fn(&mut self, r: R, t: &T) -> R {
        (self.mandatory)(r, t)
    }
    fn call_inverse(&mut self, r: R, t: &T) -> R {
        let f = &mut self.optional;
        f(r, t)
    }
}

fn accumulate_sum<T>(acc: T, next: &T) -> T
where
    T: for<'a> Add<&'a T, Output = T>,
{
    acc + next
}

fn infer_f_generic() {
    let mut _a = DefaultedClosure::new()
        .mandatory(|acc: f64, x| acc + x)
        .optional(accumulate_sum)
        .build();
}

fn infer_f_missing() {
    let mut _a = DefaultedClosure::new()
        .mandatory(|acc: f64, x| acc + x)
        .build();
}

fn fold_with_closure<'b, F1, T: 'b, R, F2>(
    iter: impl Iterator<Item = &'b T>,
    init: R,
    mut c: DefaultedClosure<F1, T, R, F2>,
) -> R
where
    F1: for<'a> FnMut(R, &T) -> R,
    F2: for<'a> FnMut(R, &T) -> R,
{
    iter.fold(init, move |acc, x| c.call_fn(acc, x))
}

fn infer_using_fold() {
    let _ = fold_with_closure(
        core::iter::once(&5i32),
        0,
        DefaultedClosure::new()
            .mandatory(|acc, &x| acc + x)
            .optional(|_acc, &x| x)
            .build(),
    );
}

fn infer_before_fold() {
    let folder = DefaultedClosure::new().mandatory(|acc, &x| acc + x).build();
    let _ = fold_with_closure(core::iter::once(&5i32), 0, folder);
}

fn infer_t_r() {
    let mut a = DefaultedClosure::new()
        // The types of the closure params should be inferred
        .mandatory(|acc, x| acc + x)
        .build();
    let _called: i32 = a.call_fn(5i32, &5);
}

fn build_with_optional_new_type() {
    let mut captured = String::from("hello");
    let mut a = DefaultedClosure::new()
        // The types of the closure params should be inferred
        .mandatory(|acc, x| acc + x)
        .optional(move |acc, x| {
            captured.push_str("hello");
            acc - x
        })
        .build();
    let _called: i32 = a.call_fn(5i32, &5);
}

fn main() {
    defaulted();
    override_default();
    inferred();
    infer_f_generic();
    infer_f_missing();
    infer_using_fold();
    infer_before_fold();
    build_with_optional_new_type();
    infer_t_r();
}
