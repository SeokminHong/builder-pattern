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
            self.write_builder(tokens, false);
        }
        // The structure has asynchronous setter(s).
        // It has to provide asynchronous builder function.
        if !async_fields.is_empty() {
            self.write_builder(tokens, true);
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

    fn write_builder(&self, tokens: &mut TokenStream, is_async: bool) {
        let ident = &self.input.ident;
        let vis = &self.input.vis;
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;
        let lifetimes = self.input.lifetimes();

        let fn_lifetime = self.input.fn_lifetime();

        let impl_tokens = self.input.tokenize_impl();
        let optional_generics = self.optional_generics().collect::<Vec<_>>();
        let satisfied_generics = self.satified_generics().collect::<Vec<_>>();
        let ty_tokens = self.input.tokenize_types();

        let mut struct_init_args = vec![];
        let mut validated_init_fields = vec![];
        let mut init_fields = vec![];
        let mut no_lazy_validation_fields = vec![];
        self.input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .for_each(|f| {
                let ident = &f.ident;
                struct_init_args.push(ident.to_token_stream());
                if f.attrs.validator.is_some()
                    && (!(f.attrs.setters & (Setters::LAZY | Setters::ASYNC)).is_empty()
                        || f.attrs.default.is_some())
                {
                    let async_case = if is_async {
                        quote! {
                            ::builder_pattern::setter::Setter::Async(f) => Ok(f().await),
                            ::builder_pattern::setter::Setter::AsyncValidated(f) => f().await,
                        }
                    } else {
                        quote! {_ => unimplemented!()}
                    };
                    validated_init_fields.push(quote! {
                        let #ident = match match self.#ident.unwrap() {
                            ::builder_pattern::setter::Setter::Value(v) => Ok(v),
                            ::builder_pattern::setter::Setter::Lazy(f) => Ok(f()),
                            ::builder_pattern::setter::Setter::LazyValidated(f) => f(),
                            #async_case
                        } {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                    });
                } else {
                    let async_case = if is_async {
                        quote! {
                            ::builder_pattern::setter::Setter::Async(f) => f().await,
                            _ => unimplemented!(),
                        }
                    } else {
                        quote! {_ => unimplemented!()}
                    };
                    init_fields.push(quote! {
                        let #ident = match self.#ident.unwrap() {
                            ::builder_pattern::setter::Setter::Value(v) => v,
                            ::builder_pattern::setter::Setter::Lazy(f) => f(),
                            #async_case
                        };
                    });
                }
                let async_case = if is_async {
                    quote! {
                        ::builder_pattern::setter::Setter::Async(f) => f().await,
                        _ => unimplemented!(),
                    }
                } else {
                    quote! {_ => unimplemented!()}
                };
                no_lazy_validation_fields.push(quote! {
                    let #ident = match self.#ident.unwrap() {
                        ::builder_pattern::setter::Setter::Value(v) => v,
                        ::builder_pattern::setter::Setter::Lazy(f) => f(),
                        #async_case
                    };
                });
            });
        let (kw_async, async_generic) = if is_async {
            (
                Some(quote! {async}),
                quote! {::builder_pattern::setter::AsyncBuilderMarker},
            )
        } else {
            (None, quote! {()})
        };
        tokens.extend(quote! {
            impl <#fn_lifetime, #impl_tokens #(#optional_generics,)*> #builder_name <
                #fn_lifetime,
                #(#lifetimes,)*
                #ty_tokens
                #(#satisfied_generics),*,
                #async_generic,
                (),
                ::builder_pattern::list::Nil
            >
                #where_clause
            {
                #[allow(dead_code)]
                #vis #kw_async fn build(self) -> #ident <#(#lifetimes,)* #ty_tokens> {
                    #(#no_lazy_validation_fields)*
                    #[allow(clippy::inconsistent_struct_constructor)]
                    #ident {
                        #(#struct_init_args),*
                    }
                }
            }
        });

        tokens.extend(quote!{
        impl <#fn_lifetime, #impl_tokens #(#optional_generics,)* ConsType> #builder_name <
            #fn_lifetime,
            #(#lifetimes,)*
            #ty_tokens
            #(#satisfied_generics),*,
            #async_generic,
            ::builder_pattern::setter::HavingAsyncValidator,
            ::builder_pattern::list::Cons<ConsType>
        >
            #where_clause
            {
                #[allow(dead_code)]
                #vis #kw_async fn build(self) -> ::std::result::Result<#ident <#(#lifetimes,)* #ty_tokens>, &'static str> {
                    #(#init_fields)*
                    #(#validated_init_fields)*
                    Ok(
                        #[allow(clippy::inconsistent_struct_constructor)]
                        #ident {
                            #(#struct_init_args),*
                        }
                    )
                }
            }
        });
    }
}
