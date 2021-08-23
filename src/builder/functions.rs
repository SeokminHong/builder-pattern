use crate::{field::Field, struct_input::StructInput};

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::{parse_quote, Attribute};

pub struct BuilderFunctions<'a> {
    pub input: &'a StructInput,
}

impl<'a> ToTokens for BuilderFunctions<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let vis = &self.input.vis;
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;
        let lifetimes = self.input.lifetimes();

        let impl_tokens = self.input.tokenize_impl();
        let ty_tokens = self.input.tokenize_types();

        let all_generics = self.input.all_generics().collect::<Vec<TokenStream>>();
        let all_builder_fields = self
            .input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .map(|f| {
                let ident = &f.ident;
                quote! { #ident: self.#ident }
            })
            .collect::<Vec<TokenStream>>();

        let mut index = 0;
        self.input.required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .for_each(move |f| {
                if f.attrs.hidden {
                    index += 1;
                    return;
                }
                let (ident, ty) = (&f.ident, &f.ty);
                let mut other_generics = all_generics.clone();
                other_generics.remove(index);
                let mut before_generics = all_generics.clone();
                before_generics[index] = TokenStream::from_str("()").unwrap();
                let mut after_generics = all_generics.clone();
                after_generics[index] = quote! {#ty};
                let mut builder_fields = all_builder_fields.clone();
                builder_fields[index] = quote! {#ident: Some(value.into())};
                index += 1;

                let (arg_type_gen, arg_type) =
                    if f.attrs.use_into {
                        (Some(quote!{<IntoType: Into<#ty>>}), TokenStream::from_str("IntoType").unwrap())
                    } else {
                        (None, quote! {#ty})
                    };
                let ret_expr = quote! {
                    #builder_name {
                        _phantom: ::std::marker::PhantomData,
                        #(#builder_fields),*
                    }
                };
                let (ret_type, ret_expr) = match &f.attrs.validator {
                    Some(v) => (quote! {
                        ::std::result::Result< #builder_name <#(#lifetimes,)* #ty_tokens #(#after_generics),*>, ()>
                    }, quote_spanned! { v.span() =>
                        #[allow(clippy::useless_conversion)]
                        match #v (value.into()) {
                            ::std::result::Result::Ok(value) => ::std::result::Result::Ok(#ret_expr),
                            ::std::result::Result::Err(_) => ::std::result::Result::Err(())
                        }
                    }),
                    None => (quote! {
                        #builder_name <#(#lifetimes,)* #ty_tokens #(#after_generics),*>
                    }, ret_expr)
                };

                let documents = BuilderFunctions::documents(f);

                tokens.extend(quote! {
                    impl <#impl_tokens #(#other_generics,)*> #builder_name <#(#lifetimes,)* #ty_tokens #(#before_generics),*>
                        #where_clause
                    {
                        #(#documents)*
                        #vis fn #ident #arg_type_gen(self, value: #arg_type) -> #ret_type {
                            #ret_expr
                        }
                    }
                });
            });
    }
}

impl<'a> BuilderFunctions<'a> {
    pub fn new(input: &'a StructInput) -> Self {
        Self { input }
    }

    fn documents(f: &Field) -> Vec<Attribute> {
        let mut docs: Vec<Attribute> = Vec::new();

        let default = match f.attrs.default.as_ref() {
            Some(expr) => format!("\n - Default: `{}`", expr.into_token_stream().to_string()),
            None => String::from(""),
        };
        let doc = format!(
            " # {}\n - Type: `{}`{}\n\n",
            f.ident,
            f.type_documents(),
            default
        );
        docs.push(parse_quote!(#[doc=#doc]));

        docs.append(f.documents().as_mut());

        docs
    }
}
