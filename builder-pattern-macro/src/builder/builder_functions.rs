use crate::{attributes::Setters, field::Field, struct_input::StructInput};

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use std::str::FromStr;
use syn::spanned::Spanned;
use syn::{parse_quote, Attribute};

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
                quote! { #ident: self.#ident }
            })
            .collect::<Vec<TokenStream>>();

        let mut index = 0;
        self.input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .for_each(move |f| {
                if f.attrs.hidden {
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
                index += 1;
            });
    }
}

impl<'a> BuilderFunctions<'a> {
    pub fn new(input: &'a StructInput) -> Self {
        Self { input }
    }

    fn documents(f: &Field, _setter: Setters) -> Vec<Attribute> {
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

    fn get_generics(
        &self,
        f: &Field,
        index: usize,
    ) -> (Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>) {
        let all_generics = self.input.all_generics().collect::<Vec<TokenStream>>();
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
        let (ident, ty) = (&f.ident, &f.ty);
        let vis = &self.input.vis;
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;
        let lifetimes = self.input.lifetimes();
        let fn_lifetime = self.input.fn_lifetime();
        let impl_tokens = self.input.tokenize_impl();
        let ty_tokens = self.input.tokenize_types();
        let (other_generics, before_generics, after_generics) = self.get_generics(f, index);
        let (arg_type_gen, arg_type) = if f.attrs.use_into {
            (
                Some(quote! {<IntoType: Into<#ty>>}),
                TokenStream::from_str("IntoType").unwrap(),
            )
        } else {
            (None, quote! {#ty})
        };
        let documents = Self::documents(f, Setters::VALUE);

        builder_fields[index] = quote! {
            #ident: Some(
                ::builder_pattern::setter::Setter::Value(value.into())
            )
        };
        let ret_expr = quote! {
            #builder_name {
                _phantom: ::std::marker::PhantomData,
                #(#builder_fields),*
            }
        };

        let (ret_type, ret_expr) = match &f.attrs.validator {
            Some(v) => (
                quote! {
                    ::std::result::Result<#builder_name <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#after_generics,)* AsyncFieldMarker>, String>
                },
                quote_spanned! { v.span() =>
                    #[allow(clippy::useless_conversion)]
                    match #v (value.into()) {
                        ::std::result::Result::Ok(value) => ::std::result::Result::Ok(#ret_expr),
                        ::std::result::Result::Err(e) => ::std::result::Result::Err(format!("Validation failed: {:?}", e))
                    }
                },
            ),
            None => (
                quote! {
                    #builder_name <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#after_generics,)* AsyncFieldMarker>
                },
                ret_expr,
            ),
        };

        tokens.extend(quote! {
            impl <#fn_lifetime, #impl_tokens #(#other_generics,)* AsyncFieldMarker> #builder_name <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#before_generics,)* AsyncFieldMarker>
                #where_clause
            {
                #(#documents)*
                #vis fn #ident #arg_type_gen(self, value: #arg_type) -> #ret_type {
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
        let (ident, ty) = (&f.ident, &f.ty);
        let seter_name = Ident::new(&format!("{}_lazy", &ident.to_string()), Span::call_site());
        let vis = &self.input.vis;
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;
        let lifetimes = self.input.lifetimes();
        let fn_lifetime = self.input.fn_lifetime();
        let impl_tokens = self.input.tokenize_impl();
        let ty_tokens = self.input.tokenize_types();
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
                    ::builder_pattern::setter::ValidatedSetter::Lazy(
                        Box::new(move || #v((value)()))
                    )
                )
            },
            None => quote! {
                #ident: Some(
                    ::builder_pattern::setter::Setter::Lazy(Box::new(value))
                )
            },
        };
        let ret_expr_val = quote! {
            #builder_name {
                _phantom: ::std::marker::PhantomData,
                #(#builder_fields),*
            }
        };

        let ret_type_val = quote! {
            #builder_name <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#after_generics,)* AsyncFieldMarker>
        };

        tokens.extend(quote! {
            impl <#fn_lifetime, #impl_tokens #(#other_generics,)* AsyncFieldMarker> #builder_name <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#before_generics,)* AsyncFieldMarker>
                #where_clause
            {
                #(#documents)*
                #vis fn #seter_name #arg_type_gen(self, value: #arg_type) -> #ret_type_val {
                    #ret_expr_val
                }
            }
        });
    }
}
