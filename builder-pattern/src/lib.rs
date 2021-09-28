//! # builder-pattern
//!
//! A derivable macro for declaring a builder pattern.
//! This crate is highly inspired by [derive_builder](https://crates.io/crates/derive-builder).
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

pub use builder_pattern_macro::Builder;
pub mod setter;
