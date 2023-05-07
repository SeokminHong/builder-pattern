//! A macro crate for [builder-pattern](https://crates.io/crates/builder-pattern).
//! Do not use this crate directly.

mod attributes;
mod builder;
mod field;
mod struct_impl;
mod struct_input;

use struct_input::StructInput;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[macro_use]
extern crate quote;
extern crate syn;

extern crate proc_macro2;

/// A derivable builder macro.
#[proc_macro_derive(
    Builder,
    attributes(
        default,
        default_async,
        default_lazy,
        hidden,
        into,
        public,
        setter,
        validator,
        infer,
    )
)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StructInput);
    TokenStream::from(input.into_token_stream())
}
