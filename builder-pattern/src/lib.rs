//! A derivable macro for declaring a builder pattern.
//! This crate is highly inspired by [derive_builder](https://crates.io/crates/derive-builder).
//!
//! ## Usage
//!
//! ```
//! use builder_pattern::Builder;
//! # enum Gender {
//! #     Male,
//! #     Female,
//! #     Nonbinary
//! # }
//!
//! #[derive(Builder)]
//! struct Person {
//!     #[into]
//!     name: String,
//!     age: i32,
//!     #[default(Gender::Nonbinary)]
//!     gender: Gender,
//! }
//!
//! let p1 = Person::new()          // PersonBuilder<(), (), (), ...>
//!     .name(String::from("Joe"))  // PersonBuilder<String, (), (), ...>
//!     .age(27)                    // PersonBuilder<String, i32, (), ...>
//!     .build();                   // Person
//!     
//! // Order does not matter.
//! let p2 = Person::new()          // PersonBuilder<(), (), (), ...>
//!     .age(32)                    // PersonBuilder<(), i32, (), ...>
//!     // `&str` is implicitly converted into `String`
//!     // because of `into` attribute!
//!     .name("Jack")               // PersonBuilder<String, i32, (), ...>
//!     .gender(Gender::Male)       // PersonBuilder<String, i32, Gender, ...>
//!     .build();                   // Person
//! // cont
//! ```
//! ```compile_fail
//! // cont
//! # use builder_pattern::Builder;
//! # enum Gender {
//! #     Male,
//! #     Female,
//! #     Nonbinary
//! # }
//! #
//! # #[derive(Builder)]
//! # struct Person {
//! #     #[into]
//! #     name: String,
//! #     age: i32,
//! #     #[default(Gender::Nonbinary)]
//! #     gender: Gender,
//! # }
//! // `name` field required - Compilation error.
//! let p3 = Person::new()          // PersonBuilder<(), (), (), ...>
//!     .age(15)                    // PersonBuilder<(), i32, (), ...>
//!     .build();
//! ```
//!
//! ## Features
//!
//! - **Chaining**: Can make structure with chained setters.
//! - **Complex types are supported**: Lifetime, trait bounds, and where clauses are well supported.
//! - **Type safety**: Autocompletion tools can suggest correct setters to build the struct. Also, `build`
//! function is allowed only the all of required fields are provided. **No Result**, **No Unwrap**. Just use it.
//! - **Lazy evaluation and asynchronous**: Lazy evaluation and asynchronous are supported.
//! The values will be evaluated when the structure is built.
//! - **No additional tasks**: There's no additional constraints to use the macro. Any structures and fields are allowed.
//! - **Auto-generated documentation**: Documentation for the builder functions are automatically generated.
//!
//! ## Attributes
//!
//! ### `#[default(expr)]`
//!
//! A field having this attribute will be considered as optional, and the `expr` will be evaluated
//! as a default value of the field. `build` function can be called without providing this field.
//!
//! ```
//! # use builder_pattern::Builder;
//! #[derive(Builder)]
//! struct Test {
//!     #[default(5)]
//!     pub a: i32,
//!     pub b: &'static str,
//! }
//!
//! let t1 = Test::new().b("Hello").build(); // The structure can be built without `a`.
//! let t2 = Test::new().b("Hi").a(3).build();
//! ```
//! ### `#[default_lazy(expr)]`
//!
//! A field having this attribute will be considered as optional, and the `expr`
//! will be lazily evaluated as a default value of the field. `expr` should be
//! a function or a closure having no arguments.
//!
//! ```
//! # use builder_pattern::Builder;
//! # fn some_heavy_task() -> i32 { 3 }
//! #[derive(Builder)]
//! struct Test {
//!     #[default_lazy(|| some_heavy_task() + 3)]
//!     pub a: i32,
//!     #[default_lazy(some_heavy_task)]
//!     pub b: i32,
//! }
//!
//! // The structure can be built without `a` and `b`.
//! let t1 = Test::new().build();
//! let t2 = Test::new().a(3).build();
//! ```
//!
//! ### `#[hidden]`
//!
//! If this attribute is present, the builder function would not be generated for the field.
//! This field requires `default` or `default_lazy` attribute.
//!
//! Example:
//!
//! ```
//! # use builder_pattern::Builder;
//! # use uuid::Uuid;
//! #[derive(Builder)]
//! struct Test {
//!     #[default(Uuid::new_v4())]
//!     #[hidden]
//!     id: Uuid,
//!     name: String,
//! }
//!
//! let test1 = Test::new()         // TestBuilder<(), ()>
//!     .name(String::from("Joe"))  // TestBuilder<String, ()>
//!     .build();                   // Test
//! # // cont
//! ```
//! ```compile_fail
//! # // cont
//! # use builder_pattern::Builder;
//! # use uuid::Uuid;
//! # #[derive(Builder)]
//! # struct Test {
//! #     #[default(Uuid::new_v4())]
//! #     #[hidden]
//! #     id: Uuid,
//! #     name: String,
//! # }
//! let test2 = Test::new()         // TestBuilder<(), ()>
//!     .name(String::from("Jack")) // TestBuilder<String, ()>
//!     // Error: `id` function is not generated.
//!     .id(Uuid::parse_str("46ebd0ee-0e6d-43c9-b90d-ccc35a913f3e").unwrap())
//!     .build();
//! ```
//!
//! ### `#[setter(value | lazy | async)]`
//!
//! If this attribute presents, it provides specified setters.
//! If it doesn't, only the value setter is provided.
//!
//! ```
//! # use builder_pattern::Builder;
//! #[derive(Builder, Debug)]
//! struct Person {
//!     // All kinds of setters are provided.
//!     #[setter(value, lazy, async)]
//!     name: String,
//!     // Only value setter is provided.
//!     age: u8,
//!     // Only lazy setter is provided.
//!     #[setter(lazy)]
//!     address: &'static str,
//! }
//!
//! # tokio_test::block_on(async {
//! let p1 = Person::new()
//!     .name_async(|| async { String::from("Joe") })
//!     .age(15)
//!     .address_lazy(|| "123 Main St")
//!     .build()  // `address` is validated here
//!     .await; // `name` is validated here
//! # });
//! ```
//!
//! ### `#[into]`
//!
//! A setter function for a field having this attribute will accept `Into`
//! trait as a parameter. You can use this setter with implicit conversion.
//! Currently, it cannot be used with async or lazy setters.
//!
//! Example:
//!
//! ```
//! # use builder_pattern::Builder;
//! #[derive(Builder)]
//! struct Test {
//!     #[into]
//!     pub name: String,
//! }
//!
//! let test = Test::new()          // TestBuilder<(), ...>
//!     // `&str` is implicitly converted into `String`.
//!     .name("Hello")              // TestBuilder<String, ...>
//!     .build();                   // Test
//! ```
//!
//! ### `#[validator(expr)]`
//!
//! Implement a validator for a field. `expr` could be a validating function that takes the field's
//! type and returns `Result`.
//!
//! ```
//! # use builder_pattern::Builder;
//! #[derive(Builder)]
//! struct Test {
//!     #[validator(is_not_empty)]
//!     #[into]
//!     pub name: String,
//! }
//!
//! fn is_not_empty(name: String) -> Result<String, &'static str> {
//!     if name.is_empty() {
//!         Err("Name cannot be empty.")
//!     } else {
//!         Ok(name)
//!     }
//! }
//!
//! let test1 = Test::new()          // TestBuilder<(), ...>
//!     .name(String::from("Hello")) // Ok(TestBuilder<String, ...>)
//!     .unwrap()                    // TestBuilder<String, ...>
//!     .build();                    // Test
//! # // cont
//! ```
//! ```should_panic
//! # // cont
//! # use builder_pattern::Builder;
//! # #[derive(Builder)]
//! # struct Test {
//! #     #[validator(is_not_empty)]
//! #     #[into]
//! #     pub name: String,
//! # }
//! #
//! # fn is_not_empty(name: String) -> Result<String, &'static str> {
//! #     if name.is_empty() {
//! #         Err("Name cannot be empty.")
//! #     } else {
//! #         Ok(name)
//! #     }
//! # }
//! #
//! let test2 = Test::new() // TestBuilder<()>
//!     .name("")           // Err(String{ "Validation failed: Name cannot be empty." })
//!     .unwrap()           // panic!
//!     .build();
//! ```
//!
//! If a `validator` is used with `lazy` or `async` setters,
//! it will also validated lazily or asynchronously. So, the
//! setter doesn't return `Result` but it is returned when it is built.
//!
//! ```
//! # use builder_pattern::Builder;
//! #[derive(Builder)]
//! struct Test {
//!     #[validator(is_not_empty)]
//!     #[setter(lazy, async)]
//!     pub name: &'static str,
//! }
//! # fn is_not_empty(name: &'static str) -> Result<&'static str, &'static str> {
//! #     if name.is_empty() {
//! #         Err("Name cannot be empty.")
//! #     } else {
//! #         Ok(name)
//! #     }
//! # }
//! #
//!
//! let test1 = Test::new()         // TestBuilder<(), ...>
//!     .name_lazy(|| "Hello")         // TestBuilder<String, ...>
//!     .build()                    // Ok(Test)
//!     .unwrap();                  // Test
//!
//! # tokio_test::block_on(async {
//! let test2 = Test::new()         // TestBuilder<(), ...>
//!     .name_async(|| async { "Hello" }) // TestBuilder<String, ...>
//!     .build()                    // Future<Result<Test, String>>
//!     .await                      // Ok(Test)
//!     .unwrap();                  // Test
//! # });
//! ```
//!
//! ## Auto-Generated Documentation
//!
//! This crate generates documentation for the builder functions. If you document fields,
//! the builder functions for them also copy the documentation.
//!
//! ### Example
//!
//! Example code:
//!
//! ```
//! # use builder_pattern::Builder;
//! #[derive(Builder)]
//! struct Test {
//!     /// A positive integer.
//!     pub positive: i32,
//!
//!     /// An integer having zero as a default value.
//!     #[default(0)]
//!     pub zero: i32,
//! }
//! ```
//!
//! Generated code:
//!
//! ```
//! # use core::marker::PhantomData;
//! # use builder_pattern::setter::*;
//! # struct Test {
//! #     pub positive: i32,
//! #     pub zero: i32,
//! # }
//! impl Test {
//!     /// Creating a builder.
//!     /// ## Required fields
//!     /// ### `positive`
//!     /// - Type: `i32`
//!     ///
//!     /// A positive integer.
//!     ///
//!     /// ## Optional fields
//!     /// ### `zero`
//!     /// - Type: `i32`
//!     /// - Default: `0`
//!     ///
//!     /// An integer having zero as a default value.
//!     fn new<'a>() -> TestBuilder<'a, (), (), (), ()> {
//!         TestBuilder {
//!             _phantom: PhantomData,
//!             positive: None,
//!             zero: Some(Setter::Value(0)),
//!         }
//!     }
//! }
//!
//! /// A builder for `Test`.
//! struct TestBuilder<
//!     'a,
//!     T1,
//!     T2,
//!     AsyncFieldMarker,
//!     ValidatorOption,
//! > {
//!     _phantom: PhantomData<(
//!         T1,
//!         T2,
//!         AsyncFieldMarker,
//!         ValidatorOption,
//!     )>,
//!     positive: Option<Setter<'a, i32>>,
//!     zero: Option<Setter<'a, i32>>,
//! }
//!
//! impl<'a, T2, AsyncFieldMarker, ValidatorOption>
//!     TestBuilder<'a, (), T2, AsyncFieldMarker, ValidatorOption>
//! {
//!     /// # positive
//!     /// - Type: `i32`
//!     ///
//!     /// A positive integer.
//!     pub fn positive(
//!         self,
//!         value: i32
//!     ) -> TestBuilder<'a, i32, T2, AsyncFieldMarker, ValidatorOption> {
//!         TestBuilder {
//!             _phantom: PhantomData,
//!             positive: Some(Setter::Value(value)),
//!             zero: self.zero,
//!         }
//!     }
//! }
//!
//! impl<'a, T1, AsyncFieldMarker, ValidatorOption>
//!     TestBuilder<'a, T1, (), AsyncFieldMarker, ValidatorOption>
//! {
//!     /// # zero
//!     /// - Type: `i32`
//!     /// - Default: `0`
//!     ///
//!     /// An integer having zero as a default value.
//!     pub fn zero(
//!         self,
//!         value: i32
//!     ) -> TestBuilder<'a, T1, i32, AsyncFieldMarker, ValidatorOption> {
//!         TestBuilder {
//!             _phantom: PhantomData,
//!             positive: self.positive,
//!             zero: Some(Setter::Value(value)),
//!         }
//!     }
//! }
//! ```
//! ## How it works
//!
//! The following code
//!
//! ```
//! # use builder_pattern::Builder;
//! # enum Gender {
//! #     Male,
//! #     Female,
//! #     Nonbinary
//! # }
//! # fn is_not_empty(val: String) -> Result<String, ()> {
//! #    Ok(val)
//! # }
//! #[derive(Builder)]
//! struct Person {
//!     #[into]
//!     #[validator(is_not_empty)]
//!     name: String,
//!     age: i32,
//!     #[default(Gender::Nonbinary)]
//!     gender: Gender,
//! }
//! ```
//!
//! will generates:
//!
//! ```
//! # use core::marker::PhantomData;
//! # use builder_pattern::setter::*;
//! # enum Gender {
//! #     Male,
//! #     Female,
//! #     Nonbinary
//! # }
//! # fn is_not_empty(val: String) -> Result<String, ()> {
//! #    Ok(val)
//! # }
//! # struct Person {
//! #     name: String,
//! #     age: i32,
//! #     gender: Gender,
//! # }
//! impl Person {
//!     // Create an empty builder
//!     fn new<'a>() -> PersonBuilder<'a, (), (), (), (), ()> {
//!         PersonBuilder {
//!             _phantom: PhantomData,
//!             age: None,
//!             name: None,
//!             gender: Some(Setter::Value(Gender::Nonbinary)),
//!         }
//!     }
//! }
//! // A builder structure for `Person`.
//! struct PersonBuilder<
//!     'a, T1, T2, T3,
//!     AsyncFieldMarker, // A generic for checking async fields
//!     ValidatorOption,  // A generic for checking lazy validators
//! > {
//!     _phantom: PhantomData<(
//!         T1, T2, T3,
//!         AsyncFieldMarker,
//!         ValidatorOption,
//!     )>,
//!     // Fields are wrapped in `Option`s.
//!     age: Option<Setter<'a, i32>>,
//!     name: Option<Setter<'a, String>>,
//!     gender: Option<Setter<'a, Gender>>,
//! }
//! // Implementation for `build` function
//! impl<'a, T3>
//!     // It can be called regardless of whether `T3` is `()` or `Gender`.
//!     PersonBuilder<'a, i32, String, T3, (), ()>
//! {
//!     fn build(self) -> Person {
//!         let age = match self.age.unwrap() {
//!             Setter::Value(v) => v,
//!             Setter::Lazy(f) => f(),
//!             _ => unimplemented!(),
//!         };
//!         let name = match self.name.unwrap() {
//!             Setter::Value(v) => v,
//!             Setter::Lazy(f) => f(),
//!             _ => unimplemented!(),
//!         };
//!         let gender = match self.gender.unwrap() {
//!             Setter::Value(v) => v,
//!             Setter::Lazy(f) => f(),
//!             _ => unimplemented!(),
//!         };
//!         Person { age, name, gender }
//!     }
//! }
//! impl<'a, T2, T3, AsyncFieldMarker, ValidatorOption>
//!     PersonBuilder<
//!         'a, (), T2, T3,
//!         AsyncFieldMarker,
//!         ValidatorOption,
//!     >
//! {
//!     // Setter for `age`
//!     fn age(
//!         self,
//!         value: i32,
//!     ) -> PersonBuilder<
//!         'a, i32, T2, T3,
//!         AsyncFieldMarker,
//!         ValidatorOption,
//!     > {
//!         PersonBuilder {
//!             _phantom: PhantomData,
//!             age: Some(Setter::Value(value.into())),
//!             name: self.name,
//!             gender: self.gender,
//!         }
//!     }
//! }
//! impl<'a, T1, T3, AsyncFieldMarker, ValidatorOption>
//!     PersonBuilder<
//!         'a, T1, (), T3,
//!         AsyncFieldMarker,
//!         ValidatorOption,
//!     >
//! {
//!     // Setter for `name`
//!     fn name<IntoType: Into<String>>(
//!         self,
//!         value: IntoType,
//!     ) -> Result<
//!         PersonBuilder<
//!             'a, T1, String, T3,
//!             AsyncFieldMarker,
//!             ValidatorOption,
//!         >,
//!         String,
//!     > {
//!         // Validate the value
//!         match is_not_empty(value.into()) {
//!             Ok(value) => Ok(PersonBuilder {
//!                 _phantom: PhantomData,
//!                 age: self.age,
//!                 name: Some(Setter::Value(value)),
//!                 gender: self.gender,
//!             }),
//!             Err(e) => Err(format!("Validation failed: {:?}", e)),
//!         }
//!     }
//! }
//! impl<'a, T1, T2, AsyncFieldMarker, ValidatorOption>
//!     PersonBuilder<
//!         'a, T1, T2, (),
//!         AsyncFieldMarker,
//!         ValidatorOption,
//!     >
//! {
//!     // Setter for `gender`
//!     fn gender(
//!         self,
//!         value: Gender,
//!     ) -> PersonBuilder<
//!         'a, T1, T2, Gender,
//!         AsyncFieldMarker,
//!         ValidatorOption,
//!     > {
//!         PersonBuilder {
//!             _phantom: PhantomData,
//!             age: self.age,
//!             name: self.name,
//!             gender: Some(Setter::Value(value.into())),
//!         }
//!     }
//! }
//! ```

pub use builder_pattern_macro::Builder;

#[doc(hidden)]
pub mod setter;
