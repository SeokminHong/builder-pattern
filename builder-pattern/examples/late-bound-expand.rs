use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use builder_pattern::Builder;
#[allow(unused)]
struct LateBound<A, B, F: FnMut(B) -> B = fn(B) -> B> {
    field_a: A,
    field_b: B,
    // #[late_bound_default]
    // # [default (| x | x)]
    transform_b: F,
}
impl<A, B> LateBound<A, B, fn(B) -> B> {
    /// Creating a builder.
    /// ## Required Fields
    /// ### `field_a`
    /// - Type: `A`
    ///
    /// ### `field_b`
    /// - Type: `B`
    ///
    /// ## Optional Fields
    /// ### `transform_b`
    /// - Type: `F`
    /// - Default: `| x | x`
    ///
    #[allow(clippy::new_ret_no_self)]
    fn new<'fn_lifetime>() -> LateBoundBuilder<'fn_lifetime, A, B, fn(B) -> B, (), (), (), (), ()> {
        #[allow(clippy::redundant_closure_call)]
        LateBoundBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            field_a: None,
            field_b: None,
            transform_b: Some(::builder_pattern::setter::Setter::LateBoundDefault(
                ::builder_pattern::refl::refl(),
            )),
        }
    }
}
/// A builder for `LateBound`.
struct LateBoundBuilder<
    'fn_lifetime,
    A,
    B,
    F: FnMut(B) -> B,
    TyBuilderPattern1,
    TyBuilderPattern2,
    TyBuilderPattern3,
    AsyncFieldMarker,
    ValidatorOption,
> {
    __builder_phantom: ::core::marker::PhantomData<(
        &'fn_lifetime (),
        A,
        B,
        F,
        TyBuilderPattern1,
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    )>,
    field_a: Option<::builder_pattern::setter::Setter<'fn_lifetime, A, A>>,
    field_b: Option<::builder_pattern::setter::Setter<'fn_lifetime, B, B>>,
    transform_b: Option<::builder_pattern::setter::Setter<'fn_lifetime, F, fn(B) -> B>>,
}
impl<'fn_lifetime, A, B, F: FnMut(B) -> B, TyBuilderPattern3>
    LateBoundBuilder<'fn_lifetime, A, B, F, A, B, TyBuilderPattern3, (), ()>
{
    #[allow(dead_code)]
    #[allow(clippy::redundant_closure_call)]
    fn build(self) -> LateBound<A, B, F> {
        let field_a = match self.field_a {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(..))
            | Some(::builder_pattern::setter::Setter::Default(..))
            | None => unreachable!("required field not set"),
            _ => panic!("not implemented"),
        };
        let field_b = match self.field_b {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(..))
            | Some(::builder_pattern::setter::Setter::Default(..))
            | None => unreachable!("required field not set"),
            _ => panic!("not implemented"),
        };
        let transform_b = match self.transform_b {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            None => unreachable!("field should have had default"),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(id)) => {
                let val: F = id.cast(|x| x);
                val
            }
            Some(::builder_pattern::setter::Setter::Default(..)) => {
                unreachable!("late-bound optional field was set in new()",)
            }
            _ => panic!("not implemented"),
        };
        LateBound {
            field_a,
            field_b,
            transform_b,
        }
    }
}
impl<
        'fn_lifetime,
        A,
        B,
        F: FnMut(B) -> B,
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
    LateBoundBuilder<
        'fn_lifetime,
        A,
        B,
        F,
        (),
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
{
    /// # field_a
    /// - Type: `A`
    ///
    fn field_a(
        self,
        value: A,
    ) -> LateBoundBuilder<
        'fn_lifetime,
        A,
        B,
        F,
        A,
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    > {
        LateBoundBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            field_a: Some(::builder_pattern::setter::Setter::Value(value.into())),
            field_b: self.field_b,
            transform_b: match self.transform_b {
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
        A,
        B,
        F: FnMut(B) -> B,
        TyBuilderPattern1,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
    LateBoundBuilder<
        'fn_lifetime,
        A,
        B,
        F,
        TyBuilderPattern1,
        (),
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
{
    /// # field_b
    /// - Type: `B`
    ///
    fn field_b(
        self,
        value: B,
    ) -> LateBoundBuilder<
        'fn_lifetime,
        A,
        B,
        F,
        TyBuilderPattern1,
        B,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    > {
        LateBoundBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            field_a: self.field_a,
            field_b: Some(::builder_pattern::setter::Setter::Value(value.into())),
            transform_b: match self.transform_b {
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
        A,
        B,
        F: FnMut(B) -> B,
        TyBuilderPattern1,
        TyBuilderPattern2,
        AsyncFieldMarker,
        ValidatorOption,
    >
    LateBoundBuilder<
        'fn_lifetime,
        A,
        B,
        F,
        TyBuilderPattern1,
        TyBuilderPattern2,
        (),
        AsyncFieldMarker,
        ValidatorOption,
    >
{
    /// # transform_b
    /// - Type: `F`
    /// - Default: `| x | x`
    ///
    fn transform_b(
        self,
        value: F,
    ) -> LateBoundBuilder<
        'fn_lifetime,
        A,
        B,
        F,
        TyBuilderPattern1,
        TyBuilderPattern2,
        F,
        AsyncFieldMarker,
        ValidatorOption,
    > {
        LateBoundBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            field_a: self.field_a,
            field_b: self.field_b,
            transform_b: Some(::builder_pattern::setter::Setter::Value(value.into())),
        }
    }
}
impl<A, B> LateBound<A, B>
where
    B: Clone,
{
    fn get_b(&self) -> B {
        (self.transform_b)(self.field_b.clone())
    }
}
fn with() {
    let l = LateBound::new()
        .field_a(String::new())
        .field_b(5)
        .transform_b(|x| x + 10)
        .build();
    assert_eq!(l.get_b(), 15);
    let l = LateBound::new().field_a(String::new());
}
fn without() {
    let l = LateBound::new().field_a(String::new()).field_b(200).build();
    assert_eq!(l.get_b(), 200);
}

fn main() {
    with();
    without();
}
