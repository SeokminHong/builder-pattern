//use builder_pattern::Builder;

// Inspired by type lists implementation from
// https://stackoverflow.com/questions/40219725/constructing-hetereogenous-type-lists-in-rust
pub struct Nil;
pub struct Cons<T>(T);

trait Append: Sized {
    type Out;
}

impl Append for Nil {
    type Out = Cons<Nil>;
}

impl<T> Append for Cons<T>
where
    T: Append,
{
    type Out = Cons<<T as Append>::Out>;
}

#[derive(Debug, PartialEq)]
struct Test {
    //#[default(-1)]
    //#[validator(is_positive)]
    pub a: i32,
    pub b: i32,
}

impl Test {
    /// Creating a builder.
    /// ## Required Fields
    /// ### `b`
    /// - Type: `i32`
    ///
    /// ## Optional Fields
    /// ### `a`
    /// - Type: `i32`
    /// - Default: `- 1`
    ///
    #[allow(clippy::new_ret_no_self)]
    fn new<'fn_lifetime>() -> TestBuilder<'fn_lifetime, (), (), (), (), <Nil as Append>::Out> {
        #[allow(clippy::redundant_closure_call)]
        TestBuilder {
            _phantom: ::std::marker::PhantomData,
            b: None,
            a: Some(::builder_pattern::setter::Setter::LazyValidated(Box::new(
                || is_positive(-1),
            ))),
        }
    }
}
/// A builder for `Test`.
struct TestBuilder<
    'fn_lifetime,
    TyBuilderPattern1,
    TyBuilderPattern2,
    AsyncFieldMarker,
    ValidatorOption,
    ValidatorDefault,
> {
    _phantom: ::std::marker::PhantomData<(
        TyBuilderPattern1,
        TyBuilderPattern2,
        AsyncFieldMarker,
        ValidatorOption,
        ValidatorDefault,
    )>,
    b: Option<::builder_pattern::setter::Setter<'fn_lifetime, i32>>,
    a: Option<::builder_pattern::setter::Setter<'fn_lifetime, i32>>,
}
impl<'fn_lifetime, TyBuilderPattern2>
    TestBuilder<'fn_lifetime, i32, TyBuilderPattern2, (), (), Nil>
{
    #[allow(dead_code)]
    fn build(self) -> Test {
        let b = match self.b.unwrap() {
            ::builder_pattern::setter::Setter::Value(v) => v,
            ::builder_pattern::setter::Setter::Lazy(f) => f(),
            _ => unimplemented!(),
        };
        let a = match self.a.unwrap() {
            ::builder_pattern::setter::Setter::Value(v) => v,
            ::builder_pattern::setter::Setter::Lazy(f) => f(),
            _ => unimplemented!(),
        };
        #[allow(clippy::inconsistent_struct_constructor)]
        Test { b, a }
    }
}
impl<'fn_lifetime, TyBuilderPattern2, ConsType>
    TestBuilder<'fn_lifetime, i32, TyBuilderPattern2, (), (), Cons<ConsType>>
{
    #[allow(dead_code)]
    fn build(self) -> ::std::result::Result<Test, &'static str> {
        let b = match self.b.unwrap() {
            ::builder_pattern::setter::Setter::Value(v) => v,
            ::builder_pattern::setter::Setter::Lazy(f) => f(),
            _ => unimplemented!(),
        };
        let a = match match self.a.unwrap() {
            ::builder_pattern::setter::Setter::Value(v) => Ok(v),
            ::builder_pattern::setter::Setter::Lazy(f) => Ok(f()),
            ::builder_pattern::setter::Setter::LazyValidated(f) => f(),
            _ => unimplemented!(),
        } {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(
            #[allow(clippy::inconsistent_struct_constructor)]
            Test { b, a },
        )
    }
}
impl<'fn_lifetime, TyBuilderPattern2, AsyncFieldMarker, ValidatorOption, ValidatorDefault>
    TestBuilder<
        'fn_lifetime,
        (),
        TyBuilderPattern2,
        AsyncFieldMarker,
        ValidatorOption,
        ValidatorDefault,
    >
{
    /// # b
    /// - Type: `i32`
    ///
    fn b(
        self,
        value: i32,
    ) -> TestBuilder<
        'fn_lifetime,
        i32,
        TyBuilderPattern2,
        AsyncFieldMarker,
        ValidatorOption,
        ValidatorDefault,
    > {
        #[allow(clippy::useless_conversion)]
        TestBuilder {
            _phantom: ::std::marker::PhantomData,
            b: Some(::builder_pattern::setter::Setter::Value(value.into())),
            a: self.a,
        }
    }
}
impl<'fn_lifetime, TyBuilderPattern1, AsyncFieldMarker, ValidatorOption, ConsType>
    TestBuilder<
        'fn_lifetime,
        TyBuilderPattern1,
        (),
        AsyncFieldMarker,
        ValidatorOption,
        Cons<ConsType>,
    >
{
    /// # a
    /// - Type: `i32`
    /// - Default: `- 1`
    ///
    fn a(
        self,
        value: i32,
    ) -> ::std::result::Result<
        TestBuilder<
            'fn_lifetime,
            TyBuilderPattern1,
            i32,
            AsyncFieldMarker,
            ValidatorOption,
            ConsType,
        >,
        String,
    > {
        #[allow(clippy::useless_conversion)]
        match is_positive(value.into()) {
            ::std::result::Result::Ok(value) => ::std::result::Result::Ok(TestBuilder {
                _phantom: ::std::marker::PhantomData,
                b: self.b,
                a: Some(::builder_pattern::setter::Setter::Value(value)),
            }),
            ::std::result::Result::Err(e) => {
                ::std::result::Result::Err(format!("Validation failed: {}", e))
            }
        }
    }
}

fn is_positive(v: i32) -> Result<i32, &'static str> {
    if v > 0 {
        Ok(v)
    } else {
        Err("Value is negative or zero.")
    }
}

fn main() {
    let t1 = Test::new().a(4).unwrap().b(5).build();
    println!("{:?}", t1);
    assert_eq!(t1, Test { a: 4, b: 5 });

    let t2 = Test::new().b(5).build();
    println!("{:?}", t2);
    assert!(t2.is_err());
}
