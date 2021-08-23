use crate::struct_input::StructInput;

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::str::FromStr;

pub struct BuilderImpl<'a> {
    pub input: &'a StructInput,
}

impl<'a> ToTokens for BuilderImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.input.ident;
        let vis = &self.input.vis;
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;
        let lifetimes = self.input.lifetimes();

        let impl_tokens = self.input.tokenize_impl();
        let optional_generics = self.optional_generics();
        let satisfied_generics = self.satified_generics();
        let ty_tokens = self.input.tokenize_types();
        let struct_init_args = self.struct_init_args();

        tokens.extend(quote! {
            impl <#impl_tokens #(#optional_generics,)*> #builder_name <#(#lifetimes,)* #ty_tokens #(#satisfied_generics),*>
            #where_clause
            {
                #vis fn build(self) -> #ident <#(#lifetimes,)* #ty_tokens> {
                    #ident {
                        #(#struct_init_args),*
                    }
                }
            }
        })
    }
}

impl<'a> BuilderImpl<'a> {
    pub fn new(input: &'a StructInput) -> Self {
        Self { input }
    }

    /// An iterator for optional fields.
    fn optional_generics(&self) -> impl Iterator<Item = TokenStream> {
        let offset = self.input.required_fields.len() + 1;
        (0..self.input.optional_fields.len())
            .into_iter()
            .map(move |i| {
                TokenStream::from_str(&format!("TyBuilderPattern{}", i + offset)).unwrap()
            })
    }

    /// An iterator to describe when the builder has enough types to build the struct.
    fn satified_generics(&'_ self) -> impl '_ + Iterator<Item = TokenStream> {
        self.input
            .required_fields
            .iter()
            .map(|f| {
                let ty = &f.ty;
                quote! {#ty}
            })
            .chain(self.optional_generics())
    }

    /// An iterator to express initialize statements.
    fn struct_init_args(&'_ self) -> impl '_ + Iterator<Item = TokenStream> {
        self.input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .map(|f| {
                let ident = &f.ident;
                quote! {
                    #ident: self.#ident.unwrap()
                }
            })
    }
}
