#![allow(dead_code, clippy::type_complexity)]

use builder_pattern::setter::Setter;
use core::marker::PhantomData;
use std::ops::Add;

struct Nil;

#[derive(Debug, PartialEq)]
struct Op<A, B = f64> {
    a: Option<A>,
    b: A,
    c: Option<B>,
}

impl<A> Op<A, f64> {
    #[allow(clippy::new_ret_no_self)]
    fn new<'a>() -> OpBuilder<'a, (Nil, Nil, Nil), A, f64, false, false> {
        OpBuilder {
            field_a: None,
            field_b: None,
            field_c: None,
            __builder_phantom: PhantomData,
        }
    }
}

struct OpBuilder<'a, Types, A, B, const ASYNC: bool = false, const VALIDATOR: bool = false> {
    field_a: Option<Setter<'a, Option<A>>>,
    field_b: Option<Setter<'a, A>>,
    field_c: Option<Setter<'a, Option<B>>>,
    __builder_phantom: PhantomData<Types>,
}

impl<'a, T2, T3, A, B, const ASYNC: bool, const VALIDATOR: bool>
    OpBuilder<'a, (Nil, T2, T3), A, B, ASYNC, VALIDATOR>
{
    fn a(self, value: Option<A>) -> OpBuilder<'a, (Option<A>, T2, T3), A, B, ASYNC, VALIDATOR> {
        OpBuilder {
            field_a: Some(Setter::Value(value)),
            field_b: self.field_b,
            field_c: self.field_c,
            __builder_phantom: PhantomData,
        }
    }
}

impl<'a, T1, T3, A, B, const ASYNC: bool, const VALIDATOR: bool>
    OpBuilder<'a, (T1, Nil, T3), A, B, ASYNC, VALIDATOR>
{
    fn b(self, value: A) -> OpBuilder<'a, (T1, A, T3), A, B, ASYNC, VALIDATOR> {
        OpBuilder {
            field_a: self.field_a,
            field_b: Some(Setter::Value(value)),
            field_c: self.field_c,
            __builder_phantom: PhantomData,
        }
    }
}

impl<'a, T1, T2, A, B, const ASYNC: bool, const VALIDATOR: bool>
    OpBuilder<'a, (T1, T2, Nil), A, B, ASYNC, VALIDATOR>
{
    fn c<_B>(
        self,
        value: Option<_B>,
    ) -> OpBuilder<'a, (T1, T2, Option<_B>), A, _B, ASYNC, VALIDATOR> {
        OpBuilder {
            field_a: self.field_a,
            field_b: self.field_b,
            field_c: Some(Setter::Value(value)),
            __builder_phantom: PhantomData,
        }
    }
}

impl<'a, A, B> OpBuilder<'a, (Option<A>, A, Option<B>), A, B, false, false> {
    fn build(self) -> Op<A, B> {
        let a = match self.field_a {
            Some(Setter::Value(v)) => v,
            _ => unreachable!("required field not set"),
        };
        let b = match self.field_b {
            Some(Setter::Value(v)) => v,
            _ => unreachable!("required field not set"),
        };
        let c = match self.field_c {
            Some(Setter::Value(v)) => v,
            _ => unreachable!("required field not set"),
        };
        Op { a, b, c }
    }
}

fn op() {
    let _x = Op::<i32>::new();
    let x = Op::new();
    let x = x.a(Some(3));
    let _y = x.c(Some(3)).b(2).build();
    let _x = Op::<i32>::new().a(Some(3));
    let x = Op::new().a(Some(3));
    let _y = x.c(Some("")).b(3).build();
    let x = Op::new().a(Some(3)).c(Some(5));
    let y = x.b(3).build();

    println!("{y:?}");
}

struct IterExtra<T, I = Vec<T>>
where
    I: IntoIterator<Item = T>,
{
    single: T,
    // #[default(None)]
    extra: Option<I>,
}

struct IterExtraBuilder<'a, Types, T, I, const ASYNC: bool, const VALIDATOR: bool> {
    field_single: Option<Setter<'a, T>>,
    field_extra: Option<Setter<'a, Option<I>>>,
    __builder_phantom: PhantomData<Types>,
}

impl<T> IterExtra<T, Vec<T>> {
    #[allow(clippy::new_ret_no_self)]
    fn new<'fn_lifetime>() -> IterExtraBuilder<'fn_lifetime, (Nil, Nil), T, Vec<T>, false, false> {
        IterExtraBuilder {
            field_single: None,
            field_extra: None,
            __builder_phantom: PhantomData,
        }
    }
}

impl<'a, T2, T, I, const ASYNC: bool, const VALIDATOR: bool>
    IterExtraBuilder<'a, (Nil, T2), T, I, ASYNC, VALIDATOR>
{
    fn single(self, value: T) -> IterExtraBuilder<'a, (T, T2), T, I, ASYNC, VALIDATOR> {
        IterExtraBuilder {
            field_single: Some(Setter::Value(value)),
            field_extra: self.field_extra,
            __builder_phantom: PhantomData,
        }
    }
}

impl<'a, T1, T, I, const ASYNC: bool, const VALIDATOR: bool>
    IterExtraBuilder<'a, (T1, Nil), T, I, ASYNC, VALIDATOR>
{
    fn extra(
        self,
        value: Option<I>,
    ) -> IterExtraBuilder<'a, (T1, Option<I>), T, I, ASYNC, VALIDATOR> {
        IterExtraBuilder {
            field_single: self.field_single,
            field_extra: Some(Setter::Value(value)),
            __builder_phantom: PhantomData,
        }
    }
}

impl<'a, T2, T, I, const ASYNC: bool, const VALIDATOR: bool>
    IterExtraBuilder<'a, (T, T2), T, I, ASYNC, VALIDATOR>
where
    I: IntoIterator<Item = T>,
{
    fn build(self) -> IterExtra<T, I> {
        let single = match self.field_single {
            Some(Setter::Value(v)) => v,
            _ => unreachable!("required field not set"),
        };
        let extra = match self.field_extra {
            Some(Setter::Value(v)) => v,
            None => None,
            _ => unreachable!("required field not set"),
        };
        IterExtra { single, extra }
    }
}

fn return_extra<T>(v: T) -> IterExtra<T> {
    IterExtra::new().single(v).build()
}

fn return_builder<'a, T>(v: T) -> IterExtraBuilder<'a, (T, Nil), T, Vec<T>, false, false> {
    IterExtra::new().single(v)
}

fn iter() {
    let _x = IterExtra::new().single(3).extra(Some(vec![3])).build();
    let _y = IterExtra::<i32>::new().single(3).build();
    let _z = IterExtra::<i32>::new().single(3);

    let _w = return_extra(3);
    let w = return_builder(3);
    let _w = w.extra(Some(vec![1, 2])).build();
}

struct DefaultedClosure<F1, T, R, F2 = fn(R, &T) -> R>
where
    F1: for<'a> FnMut(R, &T) -> R,
    F2: for<'a> FnMut(R, &T) -> R,
{
    mandatory: F1,
    optional: F2,
    phantom: PhantomData<(T, R)>,
}

struct DefaultedClosureBuilder<'a, Types, F1, T, R, F2, const ASYNC: bool, const VALIDATOR: bool> {
    field_mandatory: Option<Setter<'a, F1>>,
    field_optional: Option<Setter<'a, F2, fn(R, &T) -> R>>,
    field_phantom: Option<Setter<'a, PhantomData<(T, R)>>>,
    __builder_phantom: PhantomData<Types>,
}

impl<F1, T, R> DefaultedClosure<F1, T, R, fn(R, &T) -> R>
where
    F1: for<'a> FnMut(R, &T) -> R,
{
    #[allow(clippy::new_ret_no_self)]
    fn new<'a>(
    ) -> DefaultedClosureBuilder<'a, (Nil, Nil, Nil), F1, T, R, fn(R, &T) -> R, false, false> {
        DefaultedClosureBuilder {
            field_mandatory: None,
            field_optional: Some(Setter::LateBoundDefault(::builder_pattern::refl::refl())),
            field_phantom: None,
            __builder_phantom: PhantomData,
        }
    }
}

impl<'a, T2, T3, F1, T, R, F2, const ASYNC: bool, const VALIDATOR: bool>
    DefaultedClosureBuilder<'a, (Nil, T2, T3), F1, T, R, F2, ASYNC, VALIDATOR>
{
    fn mandatory(
        self,
        value: F1,
    ) -> DefaultedClosureBuilder<'a, (F1, T2, T3), F1, T, R, F2, ASYNC, VALIDATOR> {
        DefaultedClosureBuilder {
            field_mandatory: Some(Setter::Value(value)),
            field_optional: self.field_optional,
            field_phantom: self.field_phantom,
            __builder_phantom: PhantomData,
        }
    }
}

impl<'a, T1, T3, F1, T, R, F2, const ASYNC: bool, const VALIDATOR: bool>
    DefaultedClosureBuilder<'a, (T1, Nil, T3), F1, T, R, F2, ASYNC, VALIDATOR>
{
    fn optional<F2_>(
        self,
        value: F2_,
    ) -> DefaultedClosureBuilder<'a, (T1, F2_, T3), F1, T, R, F2_, ASYNC, VALIDATOR>
    where
        F2_: for<'b> FnMut(R, &T) -> R,
    {
        DefaultedClosureBuilder {
            field_mandatory: self.field_mandatory,
            field_optional: Some(Setter::Value(value)),
            field_phantom: self.field_phantom,
            __builder_phantom: PhantomData,
        }
    }
}

impl<'a, T2, T3, F1, T, R, F2, const ASYNC: bool, const VALIDATOR: bool>
    DefaultedClosureBuilder<'a, (F1, T2, T3), F1, T, R, F2, ASYNC, VALIDATOR>
where
    F1: for<'for_lifetime> FnMut(R, &T) -> R,
    F2: for<'for_lifetime> FnMut(R, &T) -> R,
{
    fn build(self) -> DefaultedClosure<F1, T, R, F2> {
        let mandatory = match self.field_mandatory {
            Some(Setter::Value(v)) => v,
            _ => unreachable!("required field not set"),
        };
        let optional = match self.field_optional {
            Some(Setter::Value(v)) => v,
            Some(Setter::LateBoundDefault(id)) => id.cast(|r, _t| r),
            _ => unreachable!("required field not set"),
        };
        let phantom = match self.field_phantom {
            Some(Setter::Value(v)) => v,
            Some(Setter::LateBoundDefault(id)) => id.cast(PhantomData),
            None => PhantomData,
            _ => unreachable!("required field not set"),
        };
        DefaultedClosure {
            mandatory,
            optional,
            phantom,
        }
    }
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
    op();
    iter();

    infer_f_generic();
    infer_f_missing();
    infer_using_fold();
    infer_before_fold();
    infer_t_r();
    build_with_optional_new_type();
}
