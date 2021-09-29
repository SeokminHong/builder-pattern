use crate::{attributes::Setters, struct_input::StructInput};

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::str::FromStr;

pub struct BuilderImpl<'a> {
    pub input: &'a StructInput,
}

impl<'a> ToTokens for BuilderImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut async_fields = vec![];
        let mut sync_fields = vec![];
        self.input.required_fields.iter().for_each(|f| {
            if !(f.attrs.setters & (Setters::LAZY | Setters::VALUE)).is_empty() {
                sync_fields.push(f);
            }
            if !(f.attrs.setters & Setters::ASYNC).is_empty() {
                async_fields.push(f);
            }
        });
        self.input.optional_fields.iter().for_each(|f| {
            if !(f.attrs.setters & Setters::ASYNC).is_empty() {
                async_fields.push(f);
            }
        });
        // All of fields have synchronous setters.
        // The structure can be build synchronously.
        if sync_fields.len() == self.input.required_fields.len() {
            self.write_sync_builder(tokens);
        }
        // The structure has asynchronous setter(s).
        // It has to provide asynchronous builder function.
        if !async_fields.is_empty() {
            self.write_async_builder(tokens);
        }
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

    fn write_sync_builder(&self, tokens: &mut TokenStream) {
        let ident = &self.input.ident;
        let vis = &self.input.vis;
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;
        let lifetimes = self.input.lifetimes();

        let fn_lifetime = self.input.fn_lifetime();

        let impl_tokens = self.input.tokenize_impl();
        let optional_generics = self.optional_generics();
        let satisfied_generics = self.satified_generics();
        let ty_tokens = self.input.tokenize_types();

        let mut struct_init_args = vec![];
        let mut validated_init_fields = vec![];
        let mut init_fields = vec![];
        self.input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .for_each(|f| {
                let ident = &f.ident;
                struct_init_args.push(ident.to_token_stream());
                if f.attrs.validator.is_some() {
                    validated_init_fields.push(quote! {
                        let #ident = match match self.#ident.unwrap() {
                            ::builder_pattern::setter::ValidatedSetter::Lazy(f) => f(),
                            ::builder_pattern::setter::ValidatedSetter::Value(v) => Ok(v),
                            _ => unreachable!(),
                        } {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                    });
                } else {
                    init_fields.push(quote! {
                        let #ident = match self.#ident.unwrap() {
                            ::builder_pattern::setter::Setter::Lazy(f) => f(),
                            ::builder_pattern::setter::Setter::Value(v) => v,
                            _ => unreachable!(),
                        };
                    });
                }
            });
        if validated_init_fields.is_empty() {
            tokens.extend(quote! {
                impl <#fn_lifetime, #impl_tokens #(#optional_generics,)*> #builder_name
                    <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#satisfied_generics),*, ()>
                #where_clause
                {
                    #vis fn build(self) -> #ident <#(#lifetimes,)* #ty_tokens> {
                        #(#init_fields)*
                        #ident {
                            #(#struct_init_args),*
                        }
                    }
                }
            })
        } else {
            tokens.extend(quote!{
                impl <#fn_lifetime, #impl_tokens #(#optional_generics,)*> #builder_name
                    <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#satisfied_generics),*, ()>
                #where_clause
                {
                    #vis fn build(self) -> ::std::result::Result<#ident <#(#lifetimes,)* #ty_tokens>, &'static str> {
                        #(#init_fields)*
                        #(#validated_init_fields)*
                        Ok(
                            #ident {
                                #(#struct_init_args),*
                            }
                        )
                    }
                }
            })
        }
    }

    fn write_async_builder(&self, tokens: &mut TokenStream) {
        let ident = &self.input.ident;
        let vis = &self.input.vis;
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;
        let lifetimes = self.input.lifetimes();

        let fn_lifetime = self.input.fn_lifetime();

        let impl_tokens = self.input.tokenize_impl();
        let optional_generics = self.optional_generics();
        let satisfied_generics = self.satified_generics();
        let ty_tokens = self.input.tokenize_types();

        let mut struct_init_args = vec![];
        let mut validated_init_fields = vec![];
        let mut init_fields = vec![];
        self.input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .for_each(|f| {
                let ident = &f.ident;
                struct_init_args.push(ident.to_token_stream());
                if !(f.attrs.setters & (Setters::LAZY | Setters::ASYNC)).is_empty()
                    && f.attrs.validator.is_some()
                {
                    validated_init_fields.push(quote! {
                        let #ident = match match self.#ident.unwrap() {
                            ::builder_pattern::setter::ValidatedSetter::Lazy(f) => f(),
                            ::builder_pattern::setter::ValidatedSetter::Value(v) => Ok(v),
                            ::builder_pattern::setter::ValidatedSetter::Async(f) => f().await,
                        } {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                    });
                } else {
                    init_fields.push(quote! {
                        let #ident = match self.#ident.unwrap() {
                            ::builder_pattern::setter::Setter::Lazy(f) => f(),
                            ::builder_pattern::setter::Setter::Value(v) => v,
                            ::builder_pattern::setter::Setter::Async(f) => f().await,
                        };
                    });
                }
            });
        if validated_init_fields.is_empty() {
            tokens.extend(quote!{
            impl <#fn_lifetime, #impl_tokens #(#optional_generics,)*> #builder_name
                <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#satisfied_generics),*, ::builder_pattern::setter::AsyncBuilderMarker>
                #where_clause
                {
                    #vis async fn build(self) -> #ident <#(#lifetimes,)* #ty_tokens> {
                        #(#init_fields)*
                        #ident {
                            #(#struct_init_args),*
                        }
                    }
                }
            })
        } else {
            tokens.extend(quote!{
            impl <#fn_lifetime, #impl_tokens #(#optional_generics,)*> #builder_name
                <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#satisfied_generics),*, ::builder_pattern::setter::AsyncBuilderMarker>
                #where_clause
                {
                    #vis async fn build(self) -> ::std::result::Result<#ident <#(#lifetimes,)* #ty_tokens>, &'static str> {
                        #(#init_fields)*
                        #(#validated_init_fields)*
                        Ok(
                            #ident {
                                #(#struct_init_args),*
                            }
                        )
                    }
                }
            })
        }
    }
}
