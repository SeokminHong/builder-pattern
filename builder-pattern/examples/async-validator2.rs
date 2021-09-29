//use builder_pattern::Builder;

#[derive(Debug, PartialEq)]
struct Test {
    //#[validator(is_positive)]
    //#[setter(async, value)]
    pub a: i32,
    //#[validator(is_positive)]
    //#[setter(async, value)]
    pub b: i32,
}
fn is_positive(v: i32) -> Result<i32, &'static str> {
    if v > 0 {
        Ok(v)
    } else {
        Err("Value is negative or zero.")
    }
}
/// A builder for `Test`.
struct TestBuilder<'fn_lifetime, TyBuilderPattern1, TyBuilderPattern2, AsyncFieldMarker> {
    _phantom: ::std::marker::PhantomData<(TyBuilderPattern1, TyBuilderPattern2, AsyncFieldMarker)>,
    a: Option<::builder_pattern::setter::ValidatedSetter<'fn_lifetime, i32>>,
    b: Option<::builder_pattern::setter::ValidatedSetter<'fn_lifetime, i32>>,
}
impl Test {
    /// Creating a builder.
    /// ## Required Fields
    /// ### `a`
    /// - Type: `i32`
    ///
    /// ### `b`
    /// - Type: `i32`
    ///
    fn new<'fn_lifetime>() -> TestBuilder<'fn_lifetime, (), (), ()> {
        #[allow(clippy::redundant_closure_call)]
        TestBuilder {
            _phantom: ::std::marker::PhantomData,
            a: None,
            b: None,
        }
    }
}
impl<'fn_lifetime> TestBuilder<'fn_lifetime, i32, i32, ()> {
    fn build(self) -> ::std::result::Result<Test, &'static str> {
        let a = match match self.a.unwrap() {
            ::builder_pattern::setter::ValidatedSetter::Lazy(f) => f(),
            ::builder_pattern::setter::ValidatedSetter::Value(v) => Ok(v),
            _ => unimplemented!(),
        } {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let b = match match self.b.unwrap() {
            ::builder_pattern::setter::ValidatedSetter::Lazy(f) => f(),
            ::builder_pattern::setter::ValidatedSetter::Value(v) => Ok(v),
            _ => unimplemented!(),
        } {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(Test { a, b })
    }
}

impl<'fn_lifetime>
    TestBuilder<'fn_lifetime, i32, i32, ::builder_pattern::setter::AsyncBuilderMarker>
{
    async fn build(self) -> ::std::result::Result<Test, &'static str> {
        let a = match match self.a.unwrap() {
            ::builder_pattern::setter::ValidatedSetter::Lazy(f) => f(),
            ::builder_pattern::setter::ValidatedSetter::Value(v) => Ok(v),
            ::builder_pattern::setter::ValidatedSetter::Async(f) => f().await,
        } {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let b = match match self.b.unwrap() {
            ::builder_pattern::setter::ValidatedSetter::Lazy(f) => f(),
            ::builder_pattern::setter::ValidatedSetter::Value(v) => Ok(v),
            ::builder_pattern::setter::ValidatedSetter::Async(f) => f().await,
        } {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(Test { a, b })
    }
}
impl<'fn_lifetime, TyBuilderPattern2, AsyncFieldMarker>
    TestBuilder<'fn_lifetime, (), TyBuilderPattern2, AsyncFieldMarker>
{
    /// # a
    /// - Type: `i32`
    ///
    fn a(
        self,
        value: i32,
    ) -> ::std::result::Result<
        TestBuilder<'fn_lifetime, i32, TyBuilderPattern2, AsyncFieldMarker>,
        String,
    > {
        #[allow(clippy::useless_conversion)]
        match is_positive(value.into()) {
            ::std::result::Result::Ok(value) => ::std::result::Result::Ok(TestBuilder {
                _phantom: ::std::marker::PhantomData,
                a: Some(::builder_pattern::setter::ValidatedSetter::Value(
                    value.into(),
                )),
                b: self.b,
            }),
            ::std::result::Result::Err(e) => {
                ::std::result::Result::Err(format!("Validation failed: {:?}", e))
            }
        }
    }
}
impl<'fn_lifetime, TyBuilderPattern2, AsyncFieldMarker>
    TestBuilder<'fn_lifetime, (), TyBuilderPattern2, AsyncFieldMarker>
{
    /// # a
    /// - Type: `i32`
    ///
    fn a_async<
        ReturnType: 'fn_lifetime + ::std::future::Future<Output = i32>,
        ValType: 'fn_lifetime + ::core::ops::Fn() -> ReturnType,
    >(
        self,
        value: ValType,
    ) -> TestBuilder<
        'fn_lifetime,
        i32,
        TyBuilderPattern2,
        ::builder_pattern::setter::AsyncBuilderMarker,
    > {
        TestBuilder {
            _phantom: ::std::marker::PhantomData,
            a: Some(::builder_pattern::setter::ValidatedSetter::Async(
                std::boxed::Box::new(move || {
                    std::boxed::Box::pin(async move { is_positive((value)().await) })
                }),
            )),
            b: self.b,
        }
    }
}
impl<'fn_lifetime, TyBuilderPattern1, AsyncFieldMarker>
    TestBuilder<'fn_lifetime, TyBuilderPattern1, (), AsyncFieldMarker>
{
    /// # b
    /// - Type: `i32`
    ///
    fn b(
        self,
        value: i32,
    ) -> ::std::result::Result<
        TestBuilder<'fn_lifetime, TyBuilderPattern1, i32, AsyncFieldMarker>,
        String,
    > {
        #[allow(clippy::useless_conversion)]
        match is_positive(value.into()) {
            ::std::result::Result::Ok(value) => ::std::result::Result::Ok(TestBuilder {
                _phantom: ::std::marker::PhantomData,
                a: self.a,
                b: Some(::builder_pattern::setter::ValidatedSetter::Value(
                    value.into(),
                )),
            }),
            ::std::result::Result::Err(e) => {
                ::std::result::Result::Err(format!("Validation failed: {:?}", e))
            }
        }
    }
}
impl<'fn_lifetime, TyBuilderPattern1, AsyncFieldMarker>
    TestBuilder<'fn_lifetime, TyBuilderPattern1, (), AsyncFieldMarker>
{
    /// # b
    /// - Type: `i32`
    ///
    fn b_async<
        ReturnType: 'fn_lifetime + ::std::future::Future<Output = i32>,
        ValType: 'fn_lifetime + ::core::ops::Fn() -> ReturnType,
    >(
        self,
        value: ValType,
    ) -> TestBuilder<
        'fn_lifetime,
        TyBuilderPattern1,
        i32,
        ::builder_pattern::setter::AsyncBuilderMarker,
    > {
        TestBuilder {
            _phantom: ::std::marker::PhantomData,
            a: self.a,
            b: Some(::builder_pattern::setter::ValidatedSetter::Async(
                std::boxed::Box::new(move || {
                    std::boxed::Box::pin(async move { is_positive((value)().await) })
                }),
            )),
        }
    }
}

#[tokio::main]
async fn main() {
    // If only value setters are used for validating fields, results should not be `Result`.
    let t1 = Test::new().a(3).unwrap().b(3).unwrap().build();
    println!("{:?}", t1);
    assert_eq!(t1, Test { a: 3, b: 3 });

    let t2 = Test::new()
        .a_async(|| async { 3 })
        .b_async(|| async { 3 })
        .build()
        .await;
    println!("{:?}", t2);
    assert_eq!(t2, Ok(Test { a: 3, b: 3 }));

    let t3 = Test::new()
        .a_async(|| async { 3 })
        .b(3)
        .unwrap()
        .build()
        .await;
    println!("{:?}", t3);
    assert_eq!(t3, Ok(Test { a: 3, b: 3 }));
}
