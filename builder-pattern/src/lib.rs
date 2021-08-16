mod structure;

use proc_macro::TokenStream;
use quote::ToTokens;
use structure::StructureInput;
use syn::parse_macro_input;

#[macro_use]
extern crate quote;
extern crate syn;

extern crate proc_macro2;

#[proc_macro_derive(Builder, attributes(default))]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StructureInput);
    TokenStream::from(input.into_token_stream())
}
