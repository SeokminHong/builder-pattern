use std::{marker::PhantomData, ops::Add};

struct DefaultedClosure<F1, T, R, F2 = fn(R, &T) -> R>
where
    F1: for<'a> FnMut(R, &T) -> R,
    F2: for<'a> FnMut(R, &T) -> R,
{
    mandatory: F1,
    // #[infer(F2)]
    // #[late_bound_default]
    // # [default (| r , _t | r)]
    optional: F2,
    // #[hidden]
    // #[default(PhantomData)]
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
            | None => unreachable!("required field not set"),
            _ => panic!("not implemented"),
        };
        let optional = match self.optional {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            None => unreachable!("field should have had default"),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(id)) => {
                let val: F2 = id.cast(|r, _t| r);
                val
            }
            Some(::builder_pattern::setter::Setter::Default(..)) => {
                unreachable!("late-bound optional field was set in new()",)
            }
            _ => panic!("not implemented"),
        };
        let phantom = match self.phantom {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            None => unreachable!("field should have had default"),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(id)) => {
                let val: PhantomData<(T, R)> = id.cast(PhantomData);
                val
            }
            Some(::builder_pattern::setter::Setter::Default(..)) => {
                unreachable!("late-bound optional field was set in new()",)
            }
            _ => panic!("not implemented"),
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
                _ => panic!("internal error: entered unreachable code"),
            },
            phantom: match self.phantom {
                Some(::builder_pattern::setter::Setter::LateBoundDefault(d)) => {
                    Some(::builder_pattern::setter::Setter::LateBoundDefault(d))
                }
                Some(::builder_pattern::setter::Setter::Value(val)) => {
                    Some(::builder_pattern::setter::Setter::Value(val))
                }
                _ => panic!("internal error: entered unreachable code"),
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
                _ => panic!("internal error: entered unreachable code"),
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
    let mut a = DefaultedClosure::new()
        .mandatory(|acc, x| acc + x)
        .optional(|r, _t| r)
        .build();
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

fn main() {
    infer_f_generic();
    infer_f_missing();
    infer_using_fold();
    infer_before_fold();
    infer_t_r();
    build_with_optional_new_type();
}
