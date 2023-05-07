use crate::{
    attributes::{ident_add_underscore_tree, FieldVisibility, Setters},
    field::Field,
    struct_input::StructInput,
};

use core::str::FromStr;
use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{parse_quote, spanned::Spanned, Attribute};

pub struct BuilderFunctions<'a> {
    pub input: &'a StructInput,
}

impl<'a> ToTokens for BuilderFunctions<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let all_builder_fields = self
            .input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .map(|f| {
                let ident = &f.ident;
                if f.attrs.late_bound_default {
                    quote! { #ident: None }
                } else if !f.attrs.use_inferred.is_empty() {
                    quote! { #ident: None }
                    // let Some((expr, _)) = f.attrs.default.as_ref() else {
                    //     return quote!{ #ident: compile_error!("#[use_inferred] without #[default]"), };
                    // };

                    // rebind to the #[infer]red type by stamping out the default's syntactic
                    // representation again in this setter method. e.g. `None` can fit in a slot
                    // for many different Option<T> types. If you change `T` from the default (e.g.
                    // f64) to an #[infer]red type (e.g. i32) then you as long as the macro writes
                    // `None` again, we're good.
                    // quote! {
                    //     #ident: match self.#ident {
                    //         Some(::builder_pattern::setter::Setter::Default(_)) => {
                    //             Some(::builder_pattern::setter::Setter::Value(#expr))
                    //         }
                    //         Some(::builder_pattern::setter::Setter::Value(val)) => {
                    //             Some(::builder_pattern::setter::Setter::Value(val))
                    //         }
                    //         _ => unreachable!(),
                    //     }
                    // }
                } else {
                    quote! { #ident: self.#ident }
                }
            })
            .collect::<Vec<_>>();

        let mut index = 0;
        self.input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .for_each(move |f| {
                if f.attrs.vis == FieldVisibility::Hidden {
                    index += 1;
                    return;
                }
                let mut builder_fields = all_builder_fields.clone();

                if !(f.attrs.setters & Setters::VALUE).is_empty() {
                    self.write_value_setter(tokens, f, index, &mut builder_fields);
                }
                if !(f.attrs.setters & Setters::LAZY).is_empty() {
                    self.write_lazy_setter(tokens, f, index, &mut builder_fields);
                }
                if !(f.attrs.setters & Setters::ASYNC).is_empty() {
                    self.write_async_setter(tokens, f, index, &mut builder_fields);
                }
                index += 1;
            });
    }
}

pub fn replace_type_params_in(
    stream: TokenStream,
    replacements: &[Ident],
    with: &impl Fn(&Ident) -> TokenTree,
) -> TokenStream {
    stream
        .into_iter()
        .map(|tt| match tt {
            TokenTree::Group(g) => {
                let delim = g.delimiter();
                let stream = replace_type_params_in(g.stream(), replacements, with);
                TokenTree::Group(Group::new(delim, stream))
            }
            TokenTree::Ident(ident) if replacements.contains(&ident) => with(&ident),
            x => x,
        })
        .collect()
}

impl<'a> BuilderFunctions<'a> {
    pub fn new(input: &'a StructInput) -> Self {
        Self { input }
    }

    fn documents(f: &Field, _setter: Setters) -> Vec<Attribute> {
        let mut docs: Vec<Attribute> = Vec::new();

        let default = match f.attrs.default.as_ref() {
            Some((expr, _)) => format!("\n - Default: `{}`", expr.into_token_stream().to_string()),
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

    fn get_generics(
        &self,
        f: &Field,
        index: usize,
    ) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
        let all_generics = self.input.all_generics().collect::<Vec<_>>();
        let ty = &f.ty;
        let mut other_generics = all_generics.clone();
        other_generics.remove(index);
        let mut before_generics = all_generics.clone();
        before_generics[index] = TokenStream::from_str("()").unwrap();
        let mut after_generics = all_generics;
        after_generics[index] = quote! {#ty};

        (other_generics, before_generics, after_generics)
    }

    fn write_value_setter(
        &self,
        tokens: &mut TokenStream,
        f: &Field,
        index: usize,
        builder_fields: &mut Vec<TokenStream>,
    ) {
        let (ident, orig_ty, vis) = (&f.ident, &f.ty, &f.vis);
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;
        let lifetimes = self.input.lifetimes();
        let fn_lifetime = self.input.fn_lifetime();
        let impl_tokens = self.input.tokenize_impl(&[]);
        let ty_tokens = self.input.tokenize_types(&[], false);
        let ty_tokens_ = self.input.tokenize_types(&f.attrs.infer, false);
        let fn_where_clause = self.input.setter_where_clause(&f.attrs.infer);
        let (other_generics, before_generics, mut after_generics) = self.get_generics(f, index);
        let replaced_ty = replace_type_params_in(
            quote! { #orig_ty },
            &f.attrs.infer,
            &ident_add_underscore_tree,
        );
        after_generics
            .iter_mut()
            .for_each(|ty_tokens: &mut TokenStream| {
                let tokens = std::mem::take(ty_tokens);
                *ty_tokens =
                    replace_type_params_in(tokens, &f.attrs.infer, &ident_add_underscore_tree);
            });
        let into_generics = if f.attrs.use_into {
            vec![quote! {IntoType: Into<#replaced_ty>}]
        } else {
            vec![]
        };
        let fn_generics = f.tokenize_replacement_params(&into_generics);
        let arg_type = if f.attrs.use_into {
            quote! { IntoType }
        } else {
            quote! { #replaced_ty }
        };
        let documents = Self::documents(f, Setters::VALUE);

        let (ret_type, ret_expr) = match &f.attrs.validator {
            Some(v) => {
                builder_fields[index] = quote! {
                    #ident: Some(
                        ::builder_pattern::setter::Setter::Value(value)
                    )
                };
                (
                    quote! {
                        Result<#builder_name <
                            #fn_lifetime,
                            #(#lifetimes,)*
                            #ty_tokens_
                            #(#after_generics,)*
                            AsyncFieldMarker,
                            ValidatorOption
                        >, String>
                    },
                    quote_spanned! { v.span() =>
                        #[allow(clippy::useless_conversion)]
                        match #v (value.into()) {
                            Ok(value) => Ok(
                                #builder_name {
                                    __builder_phantom: ::core::marker::PhantomData,
                                    #(#builder_fields),*
                                }),
                            Err(e) => Err(format!("Validation failed: {:?}", e))
                        }
                    },
                )
            }
            None => {
                builder_fields[index] = quote! {
                    #ident: Some(
                        ::builder_pattern::setter::Setter::Value(value.into())
                    )
                };
                (
                    quote! {
                        #builder_name <
                            #fn_lifetime,
                            #(#lifetimes,)*
                            #ty_tokens_
                            #(#after_generics,)*
                            AsyncFieldMarker,
                            ValidatorOption
                        >
                    },
                    quote! {
                        #builder_name {
                            __builder_phantom: ::core::marker::PhantomData,
                            #(#builder_fields),*
                        }
                    },
                )
            }
        };

        tokens.extend(quote! {
            impl <
                #fn_lifetime,
                #impl_tokens
                #(#other_generics,)*
                AsyncFieldMarker,
                ValidatorOption
            > #builder_name <
                #fn_lifetime,
                #(#lifetimes,)*
                #ty_tokens
                #(#before_generics,)*
                AsyncFieldMarker,
                ValidatorOption
            >
                #where_clause
            {
                #(#documents)*
                #vis fn #ident #fn_generics(self, value: #arg_type) -> #ret_type
                #fn_where_clause
                {
                    #ret_expr
                }
            }
        });
    }

    fn write_lazy_setter(
        &self,
        tokens: &mut TokenStream,
        f: &Field,
        index: usize,
        builder_fields: &mut Vec<TokenStream>,
    ) {
        let (ident, ty, vis) = (&f.ident, &f.ty, &f.vis);
        let seter_name = Ident::new(&format!("{}_lazy", &ident.to_string()), Span::call_site());
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;
        let lifetimes = self.input.lifetimes();
        let fn_lifetime = self.input.fn_lifetime();
        let impl_tokens = self.input.tokenize_impl(&[]);
        let ty_tokens = self.input.tokenize_types(&[], false);
        let (other_generics, before_generics, after_generics) = self.get_generics(f, index);
        let arg_type_gen = if f.attrs.use_into {
            quote! {<IntoType: Into<#ty>, ValType: #fn_lifetime + ::core::ops::Fn() -> IntoType>}
        } else {
            quote! {<ValType: #fn_lifetime + ::core::ops::Fn() -> #ty>}
        };
        let arg_type = quote! {ValType};
        let documents = Self::documents(f, Setters::VALUE);

        builder_fields[index] = match &f.attrs.validator {
            Some(v) => quote_spanned! { v.span() =>
                #ident: Some(
                    ::builder_pattern::setter::Setter::LazyValidated(
                        Box::new(move || #v((value)().into()))
                    )
                )
            },
            None => quote! {
                #ident: Some(
                    ::builder_pattern::setter::Setter::Lazy(
                        Box::new(move || (value)().into())
                    )
                )
            },
        };
        let ret_expr_val = quote! {
            #builder_name {
                __builder_phantom: ::core::marker::PhantomData,
                #(#builder_fields),*
            }
        };

        let validator_option = if f.attrs.validator.is_some() {
            quote! {::builder_pattern::setter::HavingLazyValidator}
        } else {
            quote! {ValidatorOption}
        };

        let ret_type = quote! {
            #builder_name <
                #fn_lifetime,
                #(#lifetimes,)*
                #ty_tokens
                #(#after_generics,)*
                AsyncFieldMarker,
                #validator_option
            >
        };

        tokens.extend(quote! {
            impl <
                #fn_lifetime,
                #impl_tokens
                #(#other_generics,)*
                AsyncFieldMarker,
                ValidatorOption
            > #builder_name <
                #fn_lifetime,
                #(#lifetimes,)*
                #ty_tokens
                #(#before_generics,)*
                AsyncFieldMarker,
                ValidatorOption
            >
                #where_clause
            {
                #(#documents)*
                #vis fn #seter_name #arg_type_gen(self, value: #arg_type) -> #ret_type {
                    #[allow(useless_conversion)]
                    #ret_expr_val
                }
            }
        });
    }

    fn write_async_setter(
        &self,
        tokens: &mut TokenStream,
        f: &Field,
        index: usize,
        builder_fields: &mut Vec<TokenStream>,
    ) {
        let (ident, ty, vis) = (&f.ident, &f.ty, &f.vis);
        let seter_name = Ident::new(&format!("{}_async", &ident.to_string()), Span::call_site());
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;
        let lifetimes = self.input.lifetimes();
        let fn_lifetime = self.input.fn_lifetime();
        let impl_tokens = self.input.tokenize_impl(&[]);
        let ty_tokens = self.input.tokenize_types(&[], false);
        let (other_generics, before_generics, after_generics) = self.get_generics(f, index);
        let arg_type_gen = if f.attrs.use_into {
            quote! {<
                IntoType: Into<#ty>,
                ReturnType: #fn_lifetime + ::core::future::Future<Output = IntoType>,
                ValType: #fn_lifetime + ::core::ops::Fn() -> ReturnType
            >}
        } else {
            quote! {<
                ReturnType: #fn_lifetime + ::core::future::Future<Output = #ty>,
                ValType: #fn_lifetime + ::core::ops::Fn() -> ReturnType
            >}
        };
        let arg_type = quote! {ValType};
        let documents = Self::documents(f, Setters::VALUE);

        builder_fields[index] = match &f.attrs.validator {
            Some(v) => quote_spanned! { v.span() =>
                #ident: Some(
                    ::builder_pattern::setter::Setter::AsyncValidated(
                        Box::new(move || {
                            Box::pin(async move { #v((value)().await.into()) })
                        })
                    )
                )
            },
            None => quote! {
                #ident: Some(
                    ::builder_pattern::setter::Setter::Async(
                        Box::new(move || Box::pin(async move { (value)().await.into() }))
                    )
                )
            },
        };
        let ret_expr_val = quote! {
            #builder_name {
                __builder_phantom: ::core::marker::PhantomData,
                #(#builder_fields),*
            }
        };

        let validator_option = if f.attrs.validator.is_some() {
            quote! {::builder_pattern::setter::HavingLazyValidator}
        } else {
            quote! {ValidatorOption}
        };

        let ret_type = quote! {
            #builder_name <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#after_generics,)* ::builder_pattern::setter::AsyncBuilderMarker, #validator_option>
        };

        tokens.extend(quote! {
            impl <
                #fn_lifetime,
                #impl_tokens
                #(#other_generics,)*
                AsyncFieldMarker,
                ValidatorOption
            > #builder_name <
                #fn_lifetime,
                #(#lifetimes,)*
                #ty_tokens
                #(#before_generics,)*
                AsyncFieldMarker,
                ValidatorOption
            >
                #where_clause
            {
                #(#documents)*
                #vis fn #seter_name #arg_type_gen(self, value: #arg_type) -> #ret_type {
                    #[allow(useless_conversion)]
                    #ret_expr_val
                }
            }
        });
    }
}
