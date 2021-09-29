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

/// A derivable builder macro.
#[proc_macro_derive(
    Builder,
    attributes(default, default_async, default_lazy, hidden, into, setter, validator)
)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StructInput);
    TokenStream::from(input.into_token_stream())
}