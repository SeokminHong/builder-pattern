extern crate std;

use std::any::{Any, TypeId};
#[allow(unused)]
struct Op<T = f64> {
    // #[infer(T)]
    a: Option<T>,
    b: T,
}
impl Op<f64> {
    /// Creating a builder.
    /// ## Required Fields
    /// ### `a`
    /// - Type: `Option < T >`
    ///
    /// ### `b`
    /// - Type: `T`
    ///
    #[allow(clippy::new_ret_no_self)]
    fn new<'fn_lifetime>() -> OpBuilder<'fn_lifetime, f64, (), (), (), ()> {
        #[allow(clippy::redundant_closure_call)]
        OpBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            a: None,
            b: None,
        }
    }
}
/// A builder for `Op`.
struct OpBuilder<
    'fn_lifetime,
    T,
    TyBuilderPattern1,
    TyBuilderPattern2,
    AsyncFieldMarker,
    ValidatorOption,
> {
    __builder_phantom: ::core::marker::PhantomData<(
        &'fn_lifetime (),
        T,
        TyBuilderPattern1,
        TyBuilderPattern2,
        AsyncFieldMarker,
        ValidatorOption,
    )>,
    a: Option<::builder_pattern::setter::Setter<'fn_lifetime, Option<T>, Option<f64>>>,
    b: Option<::builder_pattern::setter::Setter<'fn_lifetime, T, f64>>,
}
impl<'fn_lifetime, T> OpBuilder<'fn_lifetime, T, Option<T>, T, (), ()> {
    #[allow(dead_code)]
    #[allow(clippy::redundant_closure_call)]
    fn build(self) -> Op<T> {
        let a = match self.a {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(..))
            | Some(::builder_pattern::setter::Setter::Default(..))
            | None => unreachable!("required field not set"),
            _ => panic!("not implemented"),
        };
        let b = match self.b {
            Some(::builder_pattern::setter::Setter::Value(v)) => v,
            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(..))
            | Some(::builder_pattern::setter::Setter::Default(..))
            | None => unreachable!("required field not set"),
            _ => panic!("not implemented"),
        };
        Op { a, b }
    }
}
impl<'fn_lifetime, T, TyBuilderPattern2, AsyncFieldMarker, ValidatorOption>
    OpBuilder<'fn_lifetime, T, (), TyBuilderPattern2, AsyncFieldMarker, ValidatorOption>
{
    /// # a
    /// - Type: `Option < T >`
    ///
    fn a<T_>(
        self,
        value: Option<T_>,
    ) -> OpBuilder<'fn_lifetime, T_, Option<T_>, TyBuilderPattern2, AsyncFieldMarker, ValidatorOption>
    {
        OpBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            a: Some(::builder_pattern::setter::Setter::Value(value.into())),
            b: ::builder_pattern::refl::refl().cast(self.b),
        }
    }
}
impl<'fn_lifetime, T, TyBuilderPattern1, AsyncFieldMarker, ValidatorOption>
    OpBuilder<'fn_lifetime, T, TyBuilderPattern1, (), AsyncFieldMarker, ValidatorOption>
{
    /// # b
    /// - Type: `T`
    ///
    fn b(
        self,
        value: T,
    ) -> OpBuilder<'fn_lifetime, T, TyBuilderPattern1, T, AsyncFieldMarker, ValidatorOption> {
        OpBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            a: self.a,
            b: Some(::builder_pattern::setter::Setter::Value(value.into())),
        }
    }
}
fn main() {
    let a = Op::new().a(Some(5i32)).b(1).build();
    assert_eq!(a.type_id(), TypeId::of::<Op<i32>>());
}
