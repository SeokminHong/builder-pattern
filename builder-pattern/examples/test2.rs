#[derive(Debug, PartialEq)]
struct Op2<A, B = f64> {
    a: Option<A>,
    b: A,
    c: Option<B>,
}
impl<A, B> Op2<A, B> {
    /// Creating a builder.
    /// ## Required Fields
    /// ### `a`
    /// - Type: `Option < A >`
    ///
    /// ### `b`
    /// - Type: `A`
    ///
    /// ### `c`
    /// - Type: `Option < B >`
    ///
    #[allow(clippy::new_ret_no_self)]
    fn new<'fn_lifetime>() -> Op2Builder<'fn_lifetime, A, B, (), (), (), (), ()> {
        #[allow(clippy::redundant_closure_call)]
        Op2Builder {
            _phantom: ::core::marker::PhantomData,
            a: None,
            b: None,
            c: None,
        }
    }
}
/// A builder for `Op2`.
struct Op2Builder<
    'fn_lifetime,
    A,
    B,
    TyBuilderPattern1,
    TyBuilderPattern2,
    TyBuilderPattern3,
    AsyncFieldMarker,
    ValidatorOption,
> {
    _phantom: ::core::marker::PhantomData<(
        A,
        B,
        TyBuilderPattern1,
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    )>,
    a: Option<::builder_pattern::setter::Setter<'fn_lifetime, Option<A>>>,
    b: Option<::builder_pattern::setter::Setter<'fn_lifetime, A>>,
    c: Option<::builder_pattern::setter::Setter<'fn_lifetime, Option<B>>>,
}
impl<'fn_lifetime, A, B> Op2Builder<'fn_lifetime, A, B, Option<A>, A, Option<B>, (), ()> {
    #[allow(dead_code)]
    fn build(self) -> Op2<A, B> {
        let a = match self.a.unwrap() {
            ::builder_pattern::setter::Setter::Value(v) => v,
            ::builder_pattern::setter::Setter::Lazy(f) => f(),
            _ => ::core::panicking::panic("not implemented"),
        };
        let b = match self.b.unwrap() {
            ::builder_pattern::setter::Setter::Value(v) => v,
            ::builder_pattern::setter::Setter::Lazy(f) => f(),
            _ => ::core::panicking::panic("not implemented"),
        };
        let c = match self.c.unwrap() {
            ::builder_pattern::setter::Setter::Value(v) => v,
            ::builder_pattern::setter::Setter::Lazy(f) => f(),
            _ => ::core::panicking::panic("not implemented"),
        };
        Op2 { a, b, c }
    }
}
impl<
        'fn_lifetime,
        A,
        B,
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
    Op2Builder<
        'fn_lifetime,
        A,
        B,
        (),
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
{
    /// # a
    /// - Type: `Option < A >`
    ///
    fn a(
        self,
        value: Option<A>,
    ) -> Op2Builder<
        'fn_lifetime,
        A,
        B,
        Option<A>,
        TyBuilderPattern2,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    > {
        Op2Builder {
            _phantom: ::core::marker::PhantomData,
            a: Some(::builder_pattern::setter::Setter::Value(value.into())),
            b: self.b,
            c: self.c,
        }
    }
}
impl<
        'fn_lifetime,
        A,
        B,
        TyBuilderPattern1,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
    Op2Builder<
        'fn_lifetime,
        A,
        B,
        TyBuilderPattern1,
        (),
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    >
{
    /// # b
    /// - Type: `A`
    ///
    fn b(
        self,
        value: A,
    ) -> Op2Builder<
        'fn_lifetime,
        A,
        B,
        TyBuilderPattern1,
        A,
        TyBuilderPattern3,
        AsyncFieldMarker,
        ValidatorOption,
    > {
        Op2Builder {
            _phantom: ::core::marker::PhantomData,
            a: self.a,
            b: Some(::builder_pattern::setter::Setter::Value(value.into())),
            c: self.c,
        }
    }
}
impl<
        'fn_lifetime,
        A,
        B,
        TyBuilderPattern1,
        TyBuilderPattern2,
        AsyncFieldMarker,
        ValidatorOption,
    >
    Op2Builder<
        'fn_lifetime,
        A,
        B,
        TyBuilderPattern1,
        TyBuilderPattern2,
        (),
        AsyncFieldMarker,
        ValidatorOption,
    >
{
    /// # c
    /// - Type: `Option < B >`
    ///
    fn c(
        self,
        value: Option<B>,
    ) -> Op2Builder<
        'fn_lifetime,
        A,
        B,
        TyBuilderPattern1,
        TyBuilderPattern2,
        Option<B>,
        AsyncFieldMarker,
        ValidatorOption,
    > {
        Op2Builder {
            _phantom: ::core::marker::PhantomData,
            a: self.a,
            b: self.b,
            c: Some(::builder_pattern::setter::Setter::Value(value.into())),
        }
    }
}
fn main() {
    let _x = Op::<i32>::new();
    let _x = Op::<i32>::new().a(Some(3));
    let _x = Op::<i32, i32>::new().a(Some(3));
    let x = Op::new().a(Some(3)).c(Some(5));
    let y = x.b(3).build();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &[::core::fmt::ArgumentV1::new_debug(&y)],
        ));
    };
    let _x = Op2::<i32>::new();
    let x = Op2::new().a(Some(3)).c(Some(5));
    let y = x.b(3).build();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &[::core::fmt::ArgumentV1::new_debug(&y)],
        ));
    };
}
