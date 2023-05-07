use crate::{
    attributes::Setters, builder::builder_functions::replace_type_params_in,
    struct_input::StructInput,
};

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
        let defaulted_generics = self.input.defaulted_generics();

        let with_prmdef = |ident: &Ident| self.input.with_param_default(&defaulted_generics, ident);
        let replace_defaults =
            |stream: TokenStream| replace_type_params_in(stream, &defaulted_generics, &with_prmdef);

        let impl_tokens = self.input.tokenize_impl(&defaulted_generics);

        let where_clause = &self.input.generics.where_clause;
        let where_tokens =
            replace_type_params_in(quote! { #where_clause }, &defaulted_generics, &with_prmdef);

        let ty_tokens = replace_defaults(self.input.tokenize_types(&[], false));

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
                    #(#empty_generics,)*
                    (),
                    ()
                > {
                    #[allow(clippy::redundant_closure_call)]
                    #builder_name {
                        __builder_phantom: ::core::marker::PhantomData,
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
                if let (ident, Some((expr, setters))) = (&f.ident, &f.attrs.default.as_ref()) {
                    if f.attrs.late_bound_default {
                        quote_spanned! { expr.span() => #ident: None }
                    } else {
                        match *setters {
                            Setters::VALUE => quote_spanned! { expr.span() =>
                                #ident: Some(::builder_pattern::setter::Setter::Default(
                                    #expr,
                                    ::builder_pattern::refl::refl()
                                ))
                            },
                            Setters::LAZY => {
                                quote_spanned! { expr.span() =>
                                    #ident: Some(
                                        ::builder_pattern::setter::Setter::Lazy(
                                            Box::new(#expr)
                                        )
                                    )
                                }
                            }
                            _ => unimplemented!(),
                        }
                    }
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
}
