#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use builder_pattern::Builder;
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::ops::Add;
#[allow(unused)]
struct Op<T = f64> {
    #[infer(T)]
    #[default(None)]
    optional_field: Option<T>,
}
impl Op<f64> {
    /// Creating a builder.
    /// ## Optional Fields
    /// ### `optional_field`
    /// - Type: `Option < T >`
    /// - Default: `None`
    ///
    #[allow(clippy::new_ret_no_self)]
    fn new<'fn_lifetime>() -> OpBuilder<'fn_lifetime, f64, (), (), ()> {
        #[allow(clippy::redundant_closure_call)]
        OpBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            optional_field: Some(::builder_pattern::setter::Setter::Default(
                None,
                ::builder_pattern::refl::refl(),
            )),
        }
    }
}
/// A builder for `Op`.
struct OpBuilder<'fn_lifetime, T, TyBuilderPattern1, AsyncFieldMarker, ValidatorOption> {
    __builder_phantom: ::core::marker::PhantomData<(
        &'fn_lifetime (),
        T,
        TyBuilderPattern1,
        AsyncFieldMarker,
        ValidatorOption,
    )>,
    optional_field: Option<::builder_pattern::setter::Setter<'fn_lifetime, Option<T>, Option<f64>>>,
}
impl<'fn_lifetime, T, TyBuilderPattern1> OpBuilder<'fn_lifetime, T, TyBuilderPattern1, (), ()> {
    #[allow(dead_code)]
    #[allow(clippy::redundant_closure_call)]
    fn build(self) -> Op<T> {
        let optional_field = match self.optional_field {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            None => ::core::panicking::unreachable_display(
                &"early-bound optional field had no default set in new()",
            ),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(..)) => {
                ::core::panicking::unreachable_display(
                    &"early-bound optional field had no default set in new()",
                )
            }
            Some(::builder_pattern::setter::Setter::Default(default, id)) => id.cast(default),
            _ => ::core::panicking::panic("not implemented"),
        };
        Op { optional_field }
    }
}
impl<'fn_lifetime, T, AsyncFieldMarker, ValidatorOption>
    OpBuilder<'fn_lifetime, T, (), AsyncFieldMarker, ValidatorOption>
{
    /// # optional_field
    /// - Type: `Option < T >`
    /// - Default: `None`
    ///
    fn optional_field<T_>(
        self,
        value: Option<T_>,
    ) -> OpBuilder<'fn_lifetime, T_, Option<T_>, AsyncFieldMarker, ValidatorOption> {
        OpBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            optional_field: Some(::builder_pattern::setter::Setter::Value(value.into())),
        }
    }
}
fn defaulted() {
    let a = Op::new().build();
    match (&a.type_id(), &TypeId::of::<Op<f64>>()) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
fn override_default() {
    let a = Op::new().optional_field(Some(5i32)).build();
    match (&a.type_id(), &TypeId::of::<Op<i32>>()) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[allow(unused)]
struct IterExtra<T, I = Vec<T>>
where
    I: IntoIterator<Item = T>,
{
    single: T,
    #[default(None)]
    extra: Option<I>,
}
impl<T> IterExtra<T, Vec<T>>
where
    Vec<T>: IntoIterator<Item = T>,
{
    /// Creating a builder.
    /// ## Required Fields
    /// ### `single`
    /// - Type: `T`
    ///
    /// ## Optional Fields
    /// ### `extra`
    /// - Type: `Option < I >`
    /// - Default: `None`
    ///
    #[allow(clippy::new_ret_no_self)]
    fn new<'fn_lifetime>() -> IterExtraBuilder<'fn_lifetime, T, Vec<T>, (), (), (), ()> {
        #[allow(clippy::redundant_closure_call)]
        IterExtraBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            single: None,
            extra: Some(::builder_pattern::setter::Setter::Default(
                None,
                ::builder_pattern::refl::refl(),
            )),
        }
    }
}
/// A builder for `IterExtra`.
struct IterExtraBuilder<
    'fn_lifetime,
    T,
    I,
    TyBuilderPattern1,
    TyBuilderPattern2,
    AsyncFieldMarker,
    ValidatorOption,
> where
    I: IntoIterator<Item = T>,
{
    __builder_phantom: ::core::marker::PhantomData<(
        &'fn_lifetime (),
        T,
        I,
        TyBuilderPattern1,
        TyBuilderPattern2,
        AsyncFieldMarker,
        ValidatorOption,
    )>,
    single: Option<::builder_pattern::setter::Setter<'fn_lifetime, T, T>>,
    extra: Option<::builder_pattern::setter::Setter<'fn_lifetime, Option<I>, Option<Vec<T>>>>,
}
impl<'fn_lifetime, T, I, TyBuilderPattern2>
    IterExtraBuilder<'fn_lifetime, T, I, T, TyBuilderPattern2, (), ()>
where
    I: IntoIterator<Item = T>,
{
    #[allow(dead_code)]
    #[allow(clippy::redundant_closure_call)]
    fn build(self) -> IterExtra<T, I> {
        let single = match self.single {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(..))
            | Some(::builder_pattern::setter::Setter::Default(..))
            | None => ::core::panicking::unreachable_display(&"required field not set"),
            _ => ::core::panicking::panic("not implemented"),
        };
        let extra = match self.extra {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            None => ::core::panicking::unreachable_display(
                &"early-bound optional field had no default set in new()",
            ),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(..)) => {
                ::core::panicking::unreachable_display(
                    &"early-bound optional field had no default set in new()",
                )
            }
            Some(::builder_pattern::setter::Setter::Default(default, id)) => id.cast(default),
            _ => ::core::panicking::panic("not implemented"),
        };
        IterExtra { single, extra }
    }
}
impl<'fn_lifetime, T, I, TyBuilderPattern2, AsyncFieldMarker, ValidatorOption>
    IterExtraBuilder<'fn_lifetime, T, I, (), TyBuilderPattern2, AsyncFieldMarker, ValidatorOption>
where
    I: IntoIterator<Item = T>,
{
    /// # single
    /// - Type: `T`
    ///
    fn single(
        self,
        value: T,
    ) -> IterExtraBuilder<'fn_lifetime, T, I, T, TyBuilderPattern2, AsyncFieldMarker, ValidatorOption>
    {
        IterExtraBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            single: Some(::builder_pattern::setter::Setter::Value(value.into())),
            extra: self.extra,
        }
    }
}
impl<'fn_lifetime, T, I, TyBuilderPattern1, AsyncFieldMarker, ValidatorOption>
    IterExtraBuilder<'fn_lifetime, T, I, TyBuilderPattern1, (), AsyncFieldMarker, ValidatorOption>
where
    I: IntoIterator<Item = T>,
{
    /// # extra
    /// - Type: `Option < I >`
    /// - Default: `None`
    ///
    fn extra(
        self,
        value: Option<I>,
    ) -> IterExtraBuilder<
        'fn_lifetime,
        T,
        I,
        TyBuilderPattern1,
        Option<I>,
        AsyncFieldMarker,
        ValidatorOption,
    > {
        IterExtraBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            single: self.single,
            extra: Some(::builder_pattern::setter::Setter::Value(value.into())),
        }
    }
}
fn inferred() {
    let a = IterExtra::new().single(1).build();
    match (&a.type_id(), &TypeId::of::<IterExtra<i32, Vec<i32>>>()) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
#[allow(unused)]
struct DefaultedClosure<F1, T, R, F2 = fn(R, &T) -> R>
where
    F1: for<'a> FnMut(R, &T) -> R,
    F2: for<'a> FnMut(R, &T) -> R,
{
    mandatory: F1,
    #[infer(F2)]
    #[late_bound_default]
    # [default (| r , _t | r)]
    optional: F2,
    #[hidden]
    #[default(PhantomData)]
    phantom: PhantomData<(T, R)>,
}
impl<F1, T, R> DefaultedClosure<F1, T, R, fn(R, &T) -> R>
where
    F1: for<'a> FnMut(R, &T) -> R,
    fn(R, &T) -> R: for<'a> FnMut(R, &T) -> R,
{
    /// Creating a builder.
    /// ## Required Fields
    /// ### `mandatory`
    /// - Type: `F1`
    ///
    /// ## Optional Fields
    /// ### `optional`
    /// - Type: `F2`
    /// - Default: `| r, _t | r`
    ///
    /// ### `phantom`
    /// - Type: `PhantomData < (T, R) >`
    /// - Default: `PhantomData`
    ///
    #[allow(clippy::new_ret_no_self)]
    fn new<'fn_lifetime>(
    ) -> DefaultedClosureBuilder<'fn_lifetime, F1, T, R, fn(R, &T) -> R, (), (), (), (), ()> {
        #[allow(clippy::redundant_closure_call)]
        DefaultedClosureBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            mandatory: None,
            optional: Some(::builder_pattern::setter::Setter::LateBoundDefault(
                ::builder_pattern::refl::refl(),
            )),
            phantom: Some(::builder_pattern::setter::Setter::LateBoundDefault(
                ::builder_pattern::refl::refl(),
            )),
        }
    }
}
/// A builder for `DefaultedClosure`.
struct DefaultedClosureBuilder<
    'fn_lifetime,
    F1,
    T,
    R,
    F2,
    TyBuilderPattern1,
    TyBuilderPattern2,
    TyBuilderPattern3,
    AsyncFieldMarker,
    ValidatorOption,
> where
    F1: for<'a> FnMut(R, &T) -> R,
    F2: for<'a> FnMut(R, &T) -> R,
{
    __builder_phantom: ::core::marker::PhantomData<(
        &'fn_lifetime (),
        F1,
        T,
        R,
        F2,
        TyBuilderPattern1,
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    )>,
    mandatory: Option<::builder_pattern::setter::Setter<'fn_lifetime, F1, F1>>,
    optional: Option<::builder_pattern::setter::Setter<'fn_lifetime, F2, fn(R, &T) -> R>>,
    phantom: Option<
        ::builder_pattern::setter::Setter<'fn_lifetime, PhantomData<(T, R)>, PhantomData<(T, R)>>,
    >,
}
impl<'fn_lifetime, F1, T, R, F2, TyBuilderPattern2, TyBuilderPattern3>
    DefaultedClosureBuilder<
        'fn_lifetime,
        F1,
        T,
        R,
        F2,
        F1,
        TyBuilderPattern2,
        TyBuilderPattern3,
        (),
        (),
    >
where
    F1: for<'a> FnMut(R, &T) -> R,
    F2: for<'a> FnMut(R, &T) -> R,
{
    #[allow(dead_code)]
    #[allow(clippy::redundant_closure_call)]
    fn build(self) -> DefaultedClosure<F1, T, R, F2> {
        let mandatory = match self.mandatory {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(..))
            | Some(::builder_pattern::setter::Setter::Default(..))
            | None => ::core::panicking::unreachable_display(&"required field not set"),
            _ => ::core::panicking::panic("not implemented"),
        };
        let optional = match self.optional {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            None => ::core::panicking::unreachable_display(&"field should have had default"),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(id)) => {
                let val: F2 = id.cast(|r, _t| r);
                val
            }
            Some(::builder_pattern::setter::Setter::Default(..)) => {
                ::core::panicking::unreachable_display(
                    &"late-bound optional field was set in new()",
                )
            }
            _ => ::core::panicking::panic("not implemented"),
        };
        let phantom = match self.phantom {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            None => ::core::panicking::unreachable_display(&"field should have had default"),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(id)) => {
                let val: PhantomData<(T, R)> = id.cast(PhantomData);
                val
            }
            Some(::builder_pattern::setter::Setter::Default(..)) => {
                ::core::panicking::unreachable_display(
                    &"late-bound optional field was set in new()",
                )
            }
            _ => ::core::panicking::panic("not implemented"),
        };
        DefaultedClosure {
            mandatory,
            optional,
            phantom,
        }
    }
}
impl<
        'fn_lifetime,
        F1,
        T,
        R,
        F2,
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
    DefaultedClosureBuilder<
        'fn_lifetime,
        F1,
        T,
        R,
        F2,
        (),
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
where
    F1: for<'a> FnMut(R, &T) -> R,
    F2: for<'a> FnMut(R, &T) -> R,
{
    /// # mandatory
    /// - Type: `F1`
    ///
    fn mandatory(
        self,
        value: F1,
    ) -> DefaultedClosureBuilder<
        'fn_lifetime,
        F1,
        T,
        R,
        F2,
        F1,
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    > {
        DefaultedClosureBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            mandatory: Some(::builder_pattern::setter::Setter::Value(value.into())),
            optional: match self.optional {
                Some(::builder_pattern::setter::Setter::LateBoundDefault(d)) => {
                    Some(::builder_pattern::setter::Setter::LateBoundDefault(d))
                }
                Some(::builder_pattern::setter::Setter::Value(val)) => {
                    Some(::builder_pattern::setter::Setter::Value(val))
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
            phantom: match self.phantom {
                Some(::builder_pattern::setter::Setter::LateBoundDefault(d)) => {
                    Some(::builder_pattern::setter::Setter::LateBoundDefault(d))
                }
                Some(::builder_pattern::setter::Setter::Value(val)) => {
                    Some(::builder_pattern::setter::Setter::Value(val))
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
        }
    }
}
impl<
        'fn_lifetime,
        F1,
        T,
        R,
        F2,
        TyBuilderPattern1,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
    DefaultedClosureBuilder<
        'fn_lifetime,
        F1,
        T,
        R,
        F2,
        TyBuilderPattern1,
        (),
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
where
    F1: for<'a> FnMut(R, &T) -> R,
    F2: for<'a> FnMut(R, &T) -> R,
{
    /// # optional
    /// - Type: `F2`
    /// - Default: `| r, _t | r`
    ///
    fn optional<F2_>(
        self,
        value: F2_,
    ) -> DefaultedClosureBuilder<
        'fn_lifetime,
        F1,
        T,
        R,
        F2_,
        TyBuilderPattern1,
        F2_,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
    where
        F1: for<'a> FnMut(R, &T) -> R,
        F2_: for<'a> FnMut(R, &T) -> R,
    {
        DefaultedClosureBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            mandatory: self.mandatory,
            optional: Some(::builder_pattern::setter::Setter::Value(value.into())),
            phantom: match self.phantom {
                Some(::builder_pattern::setter::Setter::LateBoundDefault(d)) => {
                    Some(::builder_pattern::setter::Setter::LateBoundDefault(d))
                }
                Some(::builder_pattern::setter::Setter::Value(val)) => {
                    Some(::builder_pattern::setter::Setter::Value(val))
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            },
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
    let mut a = DefaultedClosure::new().mandatory(|acc, x| acc + x).build();
    let _called: i32 = a.call_fn(5i32, &5);
}
fn build_with_optional_new_type() {
    let mut captured = String::from("hello");
    let mut a = DefaultedClosure::new()
        .mandatory(|acc, x| acc + x)
        .optional(move |acc, x| {
            captured.push_str("hello");
            acc - x
        })
        .build();
    let _called: i32 = a.call_fn(5i32, &5);
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "main"]
pub const main: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("main"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(main())),
};
#[allow(dead_code)]
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
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&main])
}
