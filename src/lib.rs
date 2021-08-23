//! # builder-pattern
//!
//! A derivable macro for declaring a builder pattern.
//!
//! ## Features
//!
//! - **Chaining**: Can make structure with chained setters.
//! - **Complex types are supported**: Lifetime and trait bounds, and where clauses are well supported.
//! - **Type safety**: Autocompletion tools can suggest correct setters to build the struct. Also, `build`
//! function is allowed only the all of required fields are provided. **No Result**, **No Unwrap**. Just use it.
//! - **No additional tasks**: There's no additional constraints to use the macro. Any structures and fields are allowed.
//! - **Auto-generated documentations**: Documentations for the builder functions are automatically generated.

mod attributes;
mod builder;
mod field;
mod struct_impl;
mod struct_input;

use proc_macro::TokenStream;
use quote::ToTokens;
use struct_input::StructInput;
use syn::parse_macro_input;

#[macro_use]
extern crate quote;
extern crate syn;

extern crate proc_macro2;

/// # Builder
///
/// ## Usage
///
/// ```
/// use builder_pattern::Builder;
/// # enum Gender {
/// #     Male,
/// #     Female,
/// #     Nonbinary
/// # }
///
/// #[derive(Builder)]
/// struct Person {
///     #[into]
///     name: String,
///     age: i32,
///     #[default(Gender::Nonbinary)]
///     gender: Gender,
/// }
///
/// let p1 = Person::new()          // PersonBuilder<(), (), ()>
///     .name(String::from("Joe"))  // PersonBuilder<String, (), ()>
///     .age(27)                    // PersonBuilder<String, i32, ()>
///     .build();
///     
/// // Orders does not matter.
/// let p2 = Person::new()          // PersonBuilder<(), (), ()>
///     .age(32)                    // PersonBuilder<(), i32, ()>
///     // `&str` is implicitly converted into `String`
///     // because of `into` attribute!
///     .name("Jack")               // PersonBuilder<String, i32, ()>
///     .gender(Gender::Male)       // PersonBuilder<String, i32, Gender>
///     .build();                   // Person
/// ```
///
/// ```compile_fail
/// # use builder_pattern::Builder;
/// # enum Gender {
/// #     Male,
/// #     Female,
/// #     Nonbinary
/// # }
/// #
/// # #[derive(Builder)]
/// # struct Person {
/// #     #[into]
/// #     name: String,
/// #     age: i32,
/// #     #[default(Gender::Nonbinary)]
/// #     gender: Gender,
/// # }
/// // `name` field required - Compilation error.
/// let p3 = Person::new()          // PersonBuilder<(), (), ()>
///     .age(15)                    // PersonBuilder<(), i32, ()>
///     .build();
/// ```
///
/// ## Attributes
///
/// ### `#[default(expr)]`
///
/// A field having this attribute will be considered as optional and the `expr` will be evaluated
/// as a default value of the field. `build` function can be called without providing this field.
///
/// ### `#[into]`
///
/// A setter function for a field having this attribute will accept an `Into` trait as a parameter.
/// You can use this setter with implicit conversion.
///
/// Example:
///
/// ```
/// # use builder_pattern::Builder;
/// #[derive(Builder)]
/// struct Test {
///     #[into]
///     pub name: String,
/// }
///
/// let test = Test::new()          // TestBuilder<()>
///     // `&str` is implicitly converted into `String`.
///     .name("Hello")              // TestBuilder<String>
///     .build();                   // Test
/// ```
///
/// ### `#[validator(expr)]`
///
/// Implement a validator for a field. `expr` could be a validating function that takes the field's
/// type and returns `Result`.
///
/// ```
/// # use builder_pattern::Builder;
/// #[derive(Builder)]
/// struct Test {
///     #[validator(is_not_empty)]
///     #[into]
///     pub name: String,
/// }
///
/// fn is_not_empty(name: String) -> Result<String, ()> {
///     if name.is_empty() {
///         Err(())
///     } else {
///         Ok(name)
///     }
/// }
///
/// let test1 = Test::new()         // TestBuilder<()>
///     // `&str` is implicitly converted into `String`.
///     .name("Hello")              // Ok(TestBuilder<String>)
///     .unwrap()                   // TestBuilder<String>
///     .build();                   // Test
/// ```
/// ```should_panic
/// # use builder_pattern::Builder;
/// # #[derive(Builder)]
/// # struct Test {
/// #     #[validator(is_not_empty)]
/// #     #[into]
/// #     pub name: String,
/// # }
/// #
/// # fn is_not_empty(name: String) -> Result<String, ()> {
/// #     if name.is_empty() {
/// #         Err(())
/// #     } else {
/// #         Ok(name)
/// #     }
/// # }
/// #
/// let test2 = Test::new()         // TestBuilder<()>
///     .name("")                   // Err(())
///     .unwrap()                   // panic!
///     .build();
/// ```
///
/// ## Auto-Generated Documentions
///
/// This crate generates documentations for the builder functions. If you documentate the fields,
/// the builder functions for them also copy the documentations.
///
/// ### Example
///
/// Example code:
///
/// ```
/// # use builder_pattern::Builder;
/// #[derive(Builder)]
/// struct Test {
///     /// A positive integer.
///     pub positive: i32,
///
///     /// A integer having zero as a default value.
///     #[default(0)]
///     pub zero: i32,
/// }
/// ```
///
/// Generated code:
///
/// ```
/// # use std::marker::PhantomData;
/// # struct Test {
/// #     pub positive: i32,
/// #     pub zero: i32,
/// # }
/// impl Test {
///     /// Creating a builder.
///     /// ## Required fields
///     /// ### `positive`
///     /// - Type: `i32`
///     ///
///     /// A positive integer.
///     ///
///     /// ## Optional fields
///     /// ### `zero`
///     /// - Type: `i32`
///     /// - Default: `0`
///     ///
///     /// A integer having zero as a default value.
///     fn new() -> TestBuilder<(), ()> {
///         TestBuilder {
///             _phatom: PhantomData,
///             positive: None,
///             zero: Some(0),
///         }
///     }
/// }
///
/// /// A builder for `Test`.
/// struct TestBuilder<T1, T2> {
///     _phatom: PhantomData<(T1, T2)>,
///     positive: Option<i32>,
///     zero: Option<i32>,
/// }
///
/// impl TestBuilder<i32, i32> {
///     fn build(self) -> Test {
///         Test {
///             positive: self.positive.unwrap(),
///             zero: self.zero.unwrap(),
///         }
///     }
/// }
///
/// impl<T2> TestBuilder<(), T2> {
///     /// # positive
///     /// - Type: `i32`
///     ///
///     /// A positive integer.
///     pub fn positive(self, value: i32) -> TestBuilder<i32, T2> {
///         TestBuilder {
///             _phatom: PhantomData,
///             positive: Some(value),
///             zero: self.zero,
///         }
///     }
/// }
///
/// impl<T1> TestBuilder<T1, ()> {
///     /// # zero
///     /// - Type: `i32`
///     /// - Default: `0`
///     ///
///     /// A integer having zero as a default value.
///     pub fn zero(self, value: i32) -> TestBuilder<T1, i32> {
///         TestBuilder {
///             _phatom: PhantomData,
///             positive: self.positive,
///             zero: Some(value),
///         }
///     }
/// }
/// ```
/// ## How it works
///
/// The following code
///
/// ```
/// # use builder_pattern::Builder;
/// # enum Gender {
/// #     Male,
/// #     Female,
/// #     Nonbinary
/// # }
/// # fn is_not_empty(val: String) -> Result<String, ()> {
/// #    Ok(val)
/// # }
/// #[derive(Builder)]
/// struct Person {
///     #[into]
///     #[validator(is_not_empty)]
///     name: String,
///     age: i32,
///     #[default(Gender::Nonbinary)]
///     gender: Gender,
/// }
/// ```
///
/// will generates:
///
/// ```
/// # use ::std::marker::PhantomData;
/// # enum Gender {
/// #     Male,
/// #     Female,
/// #     Nonbinary
/// # }
/// # struct Person {
/// #     name: String,
/// #     age: i32,
/// #     gender: Gender,
/// # }
/// # fn is_not_empty(val: String) -> Result<String, ()> {
/// #    Ok(val)
/// # }
/// struct PersonBuilder<T1, T2, T3> {
///     name: Option<String>,
///     age: Option<i32>,
///     gender: Option<Gender>,
///     _phantom: PhantomData<(T1, T2, T3)>
/// }
///
/// impl Person {
///     // Create an empty builder
///     fn new() -> PersonBuilder<(), (), ()> {
///         PersonBuilder {
///             name: None,
///             age: None,
///             // Default value
///             gender: Some(Gender::Nonbinary),
///             _phantom: PhantomData
///         }
///     }
/// }
///
/// // Builder for `name`.
/// impl<T2, T3> PersonBuilder<(), T2, T3> {
///     // Receives `Into` traits.
///     fn name<IntoType: Into<String>>(self, value: IntoType) ->
///         Result<PersonBuilder<String, T2, T3>, ()> {
///         // Validation check.
///         match is_not_empty(value.into()) {
///             Ok(value) => Ok(PersonBuilder {
///                 // Converts `IntoType` into `String`.
///                 name: Some(value.into()),
///                 age: self.age,
///                 gender: self.gender,
///                 _phantom: PhantomData,
///             }),
///             Err(_) => Err(())
///         }
///     }
/// }
///
/// // Builder for `age`.
/// impl<T1, T3> PersonBuilder<T1, (), T3> {
///     fn age(self, value: i32) -> PersonBuilder<T1, i32, T3> {
///         PersonBuilder {
///             name: self.name,
///             age: Some(value),
///             gender: self.gender,
///             _phantom: PhantomData,
///         }
///     }
/// }
///
/// // Builder for `gender`.
/// impl<T1, T2> PersonBuilder<T1, T2, ()> {
///     fn gender(self, value: Gender) -> PersonBuilder<T1, T2, Gender> {
///         PersonBuilder {
///             name: self.name,
///             age: self.age,
///             gender: Some(value),
///             _phantom: PhantomData,
///         }
///     }
/// }
///
/// // `build` function
/// // It can be called regardless of whether `T3` is `()` or `Gender`.
/// impl<T3> PersonBuilder<String, i32, T3> {
///     fn build(self) -> Person {
///         Person {
///             name: self.name.unwrap(),
///             age: self.age.unwrap(),
///             gender: self.gender.unwrap(),
///         }
///     }
/// }
/// ```
#[proc_macro_derive(Builder, attributes(default, into, validator))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StructInput);
    TokenStream::from(input.into_token_stream())
}
