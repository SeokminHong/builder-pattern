use crate::{attributes::Setters, field::Field, struct_input::StructInput};

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
        self.input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .for_each(|f| {
                if !(f.attrs.setters & Setters::ASYNC).is_empty() {
                    async_fields.push(f);
                }
                if !(f.attrs.setters & (Setters::LAZY | Setters::VALUE)).is_empty() {
                    sync_fields.push(f);
                }
            });
        // All of fields have synchronous setters.
        // The structure can be build synchronously.
        if sync_fields.len() == self.input.num_fields() {
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

    /// An iterator to express initialize statements.
    fn struct_init_args(&'_ self) -> impl '_ + Iterator<Item = TokenStream> {
        self.input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .map(move |f| {
                let ident = &f.ident;
                quote! {
                    #ident: match self.#ident.unwrap() {
                        ::builder_pattern::setter::Setter::Value(v) => v,
                        ::builder_pattern::setter::Setter::Lazy(f) => f(),
                        _ => unreachable!(),
                    }
                }
            })
    }

    fn get_setter_meta(&'a self, struct_init_args: &'a mut Vec<TokenStream>) -> Vec<&'a Field> {
        let mut fields_need_lazy_validation = vec![];
        let mut index = 0;
        self.input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .for_each(|f| {
                if !(f.attrs.setters & (Setters::LAZY | Setters::ASYNC)).is_empty()
                    && f.attrs.validator.is_some()
                {
                    fields_need_lazy_validation.push(f);
                    struct_init_args[index] = f.ident.to_token_stream();
                }
                index += 1;
            });
        fields_need_lazy_validation
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
        let mut struct_init_args = self.struct_init_args().collect::<Vec<TokenStream>>();

        let fields_need_lazy_validation = self.get_setter_meta(&mut struct_init_args);
        if fields_need_lazy_validation.is_empty() {
            tokens.extend(quote! {
                impl <#fn_lifetime, #impl_tokens #(#optional_generics,)*> #builder_name <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#satisfied_generics),*, ()>
                #where_clause
                {
                    #vis fn build(self) -> #ident <#(#lifetimes,)* #ty_tokens> {
                        #ident {
                            #(#struct_init_args),*
                        }
                    }
                }
            })
        } else {
            let validated = fields_need_lazy_validation.iter().map(|f| {
                let ident = &f.ident;
                quote! {
                    let #ident = match match self.#ident.unwrap() {
                        ::builder_pattern::setter::ValidatedSetter::Lazy(f) => f(),
                        ::builder_pattern::setter::ValidatedSetter::Value(v) => Ok(v),
                        _ => unreachable!()
                    } {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    };
                }
            });
            tokens.extend(quote!{
                impl <#fn_lifetime, #impl_tokens #(#optional_generics,)*> #builder_name <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#satisfied_generics),*, ()>
                #where_clause
                {
                    #vis fn build(self) -> ::std::result::Result<#ident <#(#lifetimes,)* #ty_tokens>, &'static str> {
                        #(#validated)*
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
        let init_fields = self
            .input
            .required_fields
            .iter()
            .chain(self.input.optional_fields.iter())
            .map(|f| {
                let ident = &f.ident;
                struct_init_args.push(ident.to_token_stream());
                if !(f.attrs.setters & (Setters::LAZY | Setters::ASYNC)).is_empty()
                    && f.attrs.validator.is_some()
                {
                    quote! {
                        let #ident = match match self.#ident.unwrap() {
                            ::builder_pattern::setter::ValidatedSetter::Lazy(f) => f(),
                            ::builder_pattern::setter::ValidatedSetter::Value(v) => Ok(v),
                            ::builder_pattern::setter::ValidatedSetter::Async(f) => f().await,
                        } {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        };
                    }
                } else {
                    quote! {
                        let #ident = match self.#ident.unwrap() {
                            ::builder_pattern::setter::Setter::Lazy(f) => f(),
                            ::builder_pattern::setter::Setter::Value(v) => v,
                            ::builder_pattern::setter::Setter::Async(f) => f().await,
                        };
                    }
                }
            });
        tokens.extend(quote!{
           impl <#fn_lifetime, #impl_tokens #(#optional_generics,)*> #builder_name
               <#fn_lifetime, #(#lifetimes,)* #ty_tokens #(#satisfied_generics),*, ::builder_pattern::setter::AsyncBuilderMarker>
           #where_clause
           {
               #vis async fn build(self) -> ::std::result::Result<#ident <#(#lifetimes,)* #ty_tokens>, &'static str> {
                   #(#init_fields)*
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
