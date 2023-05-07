use crate::{attributes::Setters, struct_input::StructInput};

use core::str::FromStr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Type};

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

        let impl_tokens = self.input.tokenize_impl(&[]);
        let optional_generics = self.optional_generics().collect::<Vec<_>>();
        let satisfied_generics = self.satified_generics().collect::<Vec<_>>();
        let ty_tokens = self.input.tokenize_types(&[], false);

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
                let ty = &f.ty;
                struct_init_args.push(ident.to_token_stream());
                let mk_default_case =
                    |wrap: fn(TokenStream, &Type) -> TokenStream| match &f.attrs.default.as_ref() {
                        Some((expr, setters)) => {
                            let expr = match *setters {
                                Setters::VALUE => quote_spanned! { expr.span() => #expr },
                                Setters::LAZY => quote_spanned! { expr.span() => (#expr)() },
                                _ => unimplemented!(),
                            };
                            let wrapped_expr = wrap(expr, ty);
                            quote! { None => #wrapped_expr, }
                        }
                        None => quote! { None => unreachable!(), },
                    };

                if f.attrs.validator.is_some()
                    && !(f.attrs.setters & (Setters::LAZY | Setters::ASYNC)).is_empty()
                {
                    let async_case = if is_async {
                        quote! {
                            Some(::builder_pattern::setter::Setter::Async(f)) => Ok(f().await),
                            Some(::builder_pattern::setter::Setter::AsyncValidated(f)) => f().await,
                        }
                    } else {
                        quote! {_ => unimplemented!()}
                    };
                    let default_case = mk_default_case(|expr, ty| quote! { Ok((#expr) as #ty) });
                    validated_init_fields.push(quote! {
                        let #ident = match match self.#ident {
                            #default_case
                            Some(::builder_pattern::setter::Setter::Value(v)) => Ok(v),
                            Some(::builder_pattern::setter::Setter::Lazy(f)) => Ok(f()),
                            Some(::builder_pattern::setter::Setter::LazyValidated(f)) => f(),
                            #async_case
                        } {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                    });
                } else {
                    let async_case = if is_async {
                        quote! {
                            Some(::builder_pattern::setter::Setter::Async(f)) => f().await,
                            _ => unimplemented!(),
                        }
                    } else {
                        quote! {_ => unimplemented!()}
                    };
                    let default_case = mk_default_case(|expr, ty| quote! { (#expr) as #ty });
                    init_fields.push(quote! {
                        let #ident = match self.#ident {
                            #default_case
                            Some(::builder_pattern::setter::Setter::Value(v)) => v,
                            Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
                            #async_case
                        };
                    });
                }
                let async_case = if is_async {
                    quote! {
                        Some(::builder_pattern::setter::Setter::Async(f)) => f().await,
                        _ => unimplemented!(),
                    }
                } else {
                    quote! {_ => unimplemented!()}
                };
                let default_case = mk_default_case(|expr, ty| quote! { (#expr) as #ty });
                no_lazy_validation_fields.push(quote! {
                    let #ident = match self.#ident {
                        Some(::builder_pattern::setter::Setter::Value(v)) => v,
                        Some(::builder_pattern::setter::Setter::Lazy(f)) => f(),
                        #default_case
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
        impl <#fn_lifetime, #impl_tokens #(#optional_generics,)*> #builder_name
            <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#satisfied_generics),*, #async_generic, ()>
            #where_clause
            {
                #[allow(dead_code)]
                #[allow(clippy::redundant_closure_call)]
                #vis #kw_async fn build(self) -> #ident <#(#lifetimes,)* #ty_tokens> {
                    #(#no_lazy_validation_fields)*
                    #ident {
                        #(#struct_init_args),*
                    }
                }
            }
        });

        // Check is there any validator may be evaluated lazily.
        let mut having_lazy_validator = false;
        self.input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .for_each(|f| {
                let default_setters = match f.attrs.default {
                    Some((_, s)) => s,
                    None => Setters::empty(),
                };
                let setters = f.attrs.setters | default_setters;
                if f.attrs.validator.is_some()
                    && !(setters & (Setters::LAZY | Setters::ASYNC)).is_empty()
                {
                    having_lazy_validator = true;
                }
            });
        if having_lazy_validator {
            tokens.extend(quote!{
                impl <#fn_lifetime, #impl_tokens #(#optional_generics,)*> #builder_name <
                    #fn_lifetime,
                    #(#lifetimes,)*
                    #ty_tokens
                    #(#satisfied_generics),*,
                    #async_generic,
                    ::builder_pattern::setter::HavingLazyValidator
                >
                    #where_clause
                {
                    #[allow(dead_code)]
                    #vis #kw_async fn build(self) -> Result<#ident <#(#lifetimes,)* #ty_tokens>, &'static str> {
                        #(#init_fields)*
                        #(#validated_init_fields)*
                        Ok(
                            #ident {
                                #(#struct_init_args),*
                            }
                        )
                    }
                }
            });
        }
    }
}
