use std::any::{Any, TypeId};

fn x() {
    let x = std::collections::HashMap::<i32, i32>::new();
    let y = std::collections::HashMap::<i32, i32>::default();
}

#[allow(unused)]
struct Op<T = f64> {
    // #[infer(T)]
    // #[default(None)]
    optional_field: Option<T>,
}
impl<T> Op<T> {
    /// Creating a builder.
    /// ## Optional Fields
    /// ### `optional_field`
    /// - Type: `Option < T >`
    /// - Default: `None`
    ///
    #[allow(clippy::new_ret_no_self)]
    fn new<'fn_lifetime>() -> OpBuilder<'fn_lifetime, T, (), (), ()> {
        #[allow(clippy::redundant_closure_call)]
        OpBuilder {
            __builder_phantom: ::core::marker::PhantomData,
            optional_field: Some(::builder_pattern::setter::Setter::Value(None)),
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
            None => unreachable!(&"early-bound optional field had no default set in new()",),
            Some(::builder_pattern::setter::Setter::LateBoundDefault(..)) => {
                unreachable!(&"early-bound optional field had no default set in new()",)
            }
            Some(::builder_pattern::setter::Setter::Default(default, id)) => id.cast(default),
            _ => panic!("not implemented"),
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
    assert_eq!(a.type_id(), TypeId::of::<Op<f64>>());
}
fn override_default() {
    let a = Op::new().optional_field(Some(5i32)).build();
    assert_eq!(a.type_id(), TypeId::of::<Op<i32>>());
}

fn main() {
    defaulted();
    override_default();
}
