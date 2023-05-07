use crate::{builder::builder_functions::replace_type_params_in, struct_input::StructInput};

use core::str::FromStr;
use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{parse_quote, spanned::Spanned, Attribute, Generics};

/// Implementation for the given structure.
/// It creates a `new` function.
pub struct StructImpl<'a> {
    pub input: &'a StructInput,
}

impl<'a> ToTokens for StructImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.input.ident;
        let vis = &self.input.vis;
        let builder_name = self.input.builder_name();

        let lifetimes = self.input.lifetimes();
        let empty_generics = self.empty_generics();
        let defaulted_generics = self.defaulted_generics();
        fn with_param_default(
            generics: &Generics,
            defaulted_generics: &[Ident],
            ident: &Ident,
        ) -> TokenTree {
            let with_prmdef =
                |ident: &Ident| with_param_default(&generics, defaulted_generics, ident);
            generics
                .type_params()
                .find_map(|x| {
                    if x.ident == *ident {
                        let default = x.default.as_ref().unwrap();
                        let stream = quote! { #default };
                        let replaced_within =
                            replace_type_params_in(stream, defaulted_generics, &with_prmdef);
                        Some(TokenTree::Group(Group::new(
                            proc_macro2::Delimiter::None,
                            replaced_within,
                        )))
                    } else {
                        None
                    }
                })
                .expect("hmmmm")
        }

        let with_prmdef =
            |ident: &Ident| with_param_default(&self.input.generics, &defaulted_generics, ident);

        let impl_tokens = self.input.tokenize_impl(&defaulted_generics);

        let where_clause = &self.input.generics.where_clause;
        let where_tokens =
            replace_type_params_in(quote! { #where_clause }, &defaulted_generics, &with_prmdef);

        let ty_tokens = replace_type_params_in(
            self.input.tokenize_types(&[], false),
            &defaulted_generics,
            &with_prmdef,
        );

        let fn_lifetime = self.input.fn_lifetime();

        let builder_init_args = self.builder_init_args();
        let docs = self.documents();

        tokens.extend(quote! {
            impl <#impl_tokens> #ident <#(#lifetimes,)* #ty_tokens> #where_tokens {
                #(#docs)*
                #[allow(clippy::new_ret_no_self)]
                #vis fn new<#fn_lifetime>() -> #builder_name<
                    #fn_lifetime,
                    #(#lifetimes,)*
                    #ty_tokens
                    #(#empty_generics),*,
                    (),
                    ()
                > {
                    #[allow(clippy::redundant_closure_call)]
                    #builder_name {
                        _phantom: ::core::marker::PhantomData,
                        #(#builder_init_args),*
                    }
                }
            }
        });
    }
}

impl<'a> StructImpl<'a> {
    pub fn new(input: &'a StructInput) -> StructImpl<'a> {
        StructImpl { input }
    }

    /// An iterator to describe initial state of builder.
    fn empty_generics(&self) -> impl Iterator<Item = TokenStream> {
        (0..(self.input.required_fields.len() + self.input.optional_fields.len()))
            .into_iter()
            .map(|_| TokenStream::from_str("()").unwrap())
    }

    /// An iterator for initialize arguments of the builder.
    /// Required fields are filled with `None`, optional fields are filled with given value via `default` attribute.
    fn builder_init_args(&self) -> Vec<TokenStream> {
        let v = self
            .input
            .required_fields
            .iter()
            .map(|f| {
                let ident = &f.ident;
                quote! {
                    #ident: None
                }
            })
            .chain(self.input.optional_fields.iter().map(|f| {
                if let (ident, Some((expr, _setters))) = (&f.ident, &f.attrs.default.as_ref()) {
                    quote_spanned! { expr.span() => #ident: None }
                } else {
                    unimplemented!()
                }
            }))
            .collect::<Vec<_>>();
        v
    }

    fn documents(&self) -> Vec<Attribute> {
        let mut docs: Vec<Attribute> = Vec::new();

        docs.push(parse_quote!(#[doc=" Creating a builder."]));

        if !self.input.required_fields.is_empty() {
            docs.push(parse_quote!(#[doc=" ## Required Fields"]));
            for f in self.input.required_fields.iter() {
                let ident = &f.ident;

                let doc = format!(" ### `{}`\n - Type: `{}`\n\n", ident, f.type_documents());
                docs.push(parse_quote!(#[doc=#doc]));
                docs.append(f.documents().as_mut());
            }
        }

        if !self.input.optional_fields.is_empty() {
            docs.push(parse_quote!(#[doc=" ## Optional Fields"]));
            for f in self.input.optional_fields.iter() {
                let ident = &f.ident;
                let (expr, _) = f
                    .attrs
                    .default
                    .as_ref()
                    .unwrap_or_else(|| unimplemented!("Invalid expression is provided!"));

                let doc = format!(
                    " ### `{}`\n - Type: `{}`\n - Default: `{}`\n\n",
                    ident,
                    f.type_documents(),
                    expr.into_token_stream()
                );
                docs.push(parse_quote!(#[doc=#doc]));
                docs.append(f.documents().as_mut());
            }
        }

        docs
    }

    fn defaulted_generics(&self) -> Vec<Ident> {
        self.input
            .generics
            .type_params()
            .filter(|x| x.default.is_some())
            .map(|x| x.ident.clone())
            .collect()
    }
}
