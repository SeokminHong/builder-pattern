use crate::attributes::FieldAttributes;
use crate::builder::{builder_decl::BuilderDecl, builder_impl::BuilderImpl};
use crate::field::Field;
use crate::struct_impl::StructImpl;

use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{
    AttrStyle, Attribute, Data, DeriveInput, Fields, GenericParam, Generics, Token, Visibility,
};

pub struct StructInput {
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<Attribute>,
    pub required_fields: Vec<Field>,
    pub optional_fields: Vec<Field>,
}

impl Parse for StructInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let input: DeriveInput = input.parse()?;
        // Visibility of the sturcture.
        let vis = input.vis;
        // Name of the structure.
        let ident = input.ident;
        // Generics of the structure.
        let generics = input.generics;
        // Attributes of the structure.
        let attrs = input.attrs;

        // Fields of the structure.
        let fields = if let Data::Struct(d) = input.data {
            if let Fields::Named(f) = d.fields {
                f
            } else {
                unimplemented!("Only named structures are supported!");
            }
        } else {
            unimplemented!("Only structures are supported!");
        };

        let mut optional_fields: Vec<Field> = Vec::new();
        let mut required_fields: Vec<Field> = Vec::new();
        for f in fields.named.into_iter() {
            let attrs: FieldAttributes = f.attrs.into();
            let fields = if attrs.default.is_some() {
                &mut optional_fields
            } else {
                &mut required_fields
            };
            fields.push(Field {
                vis: f.vis,
                ident: f.ident.unwrap(),
                ty: f.ty,
                attrs,
            });
        }
        // Sort by ident.
        optional_fields.sort();
        required_fields.sort();

        Ok(StructInput {
            vis,
            ident,
            generics,
            attrs,
            required_fields,
            optional_fields,
        })
    }
}

impl ToTokens for StructInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Implement the `new` function creating the builder.
        let struct_impl = StructImpl::new(self);
        struct_impl.to_tokens(tokens);

        // Declare builder structure.
        let builder_decl = BuilderDecl::new(self);
        builder_decl.to_tokens(tokens);

        // Implement the `build` function for the builder.
        let builder_impl = BuilderImpl::new(self);
        builder_impl.to_tokens(tokens);

        // Parse generic parameters.
        let ty_tokens = self.tokenize_types();

        let lifetimes = self.lifetimes();

        let builder_name = Ident::new(&format!("{}Builder", self.ident), Span::call_site());

        let builder_functions = self.builder_functions(&builder_name, &lifetimes, &ty_tokens);

        tokens.extend(quote! {
            #(#builder_functions)*
        });
    }
}

impl StructInput {
    /// Name of the builder structure.
    pub fn builder_name(&self) -> Ident {
        Ident::new(&format!("{}Builder", self.ident), Span::call_site())
    }

    /// Get token stream for lifetimes.
    pub fn lifetimes(&self) -> Vec<TokenStream> {
        self.generics
            .lifetimes()
            // Remove bounds
            .map(|f| f.lifetime.to_token_stream())
            .collect::<Vec<TokenStream>>()
    }

    /// An iterator for generics like [U1, U2, ...].
    pub fn all_generics(&self) -> impl Iterator<Item = TokenStream> {
        (0..(self.required_fields.len() + self.optional_fields.len()))
            .into_iter()
            .map(|i| TokenStream::from_str(&format!("TyBuilderPattern{}", i + 1)).unwrap())
    }

    /// An iterator for fields of the builder.
    pub fn builder_fields(&'_ self) -> impl '_ + Iterator<Item = TokenStream> {
        let iters = self
            .required_fields
            .iter()
            .chain(self.optional_fields.iter());
        iters.map(|f| {
            let (ident, ty) = (&f.ident, &f.ty);
            quote! {
                #ident: Option<#ty>
            }
        })
    }

    /// An iterator to describe builder functions.
    fn builder_functions<'a>(
        &'a self,
        builder_name: &'a Ident,
        lifetimes: &'a [TokenStream],
        ty_tokens: &'a TokenStream,
    ) -> impl 'a + Iterator<Item = TokenStream> {
        let vis = &self.vis;
        let where_clause = &self.generics.where_clause;
        let impl_tokens = self.tokenize_impl();
        let all_generics = self.all_generics().collect::<Vec<TokenStream>>();
        let all_builder_fields = self
            .required_fields
            .iter()
            .chain(self.optional_fields.iter())
            .map(|f| {
                let ident = &f.ident;
                quote! { #ident: self.#ident }
            })
            .collect::<Vec<TokenStream>>();

        let mut index = 0;
        self.required_fields
            .iter()
            .chain(self.optional_fields.iter())
            .map(move |f| {
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
                quote! {
                    impl <#impl_tokens #(#other_generics,)*> #builder_name <#(#lifetimes,)* #ty_tokens #(#before_generics),*>
                        #where_clause
                    {
                        #vis fn #ident #arg_type_gen(mut self, value: #arg_type) -> #ret_type {
                            #ret_expr
                        }
                    }
                }
            })
    }

    /// Tokenize type parameters.
    /// It skips lifetimes and has no outer brackets.
    pub fn tokenize_types(&self) -> TokenStream {
        let generics = &self.generics;
        let mut tokens = TokenStream::new();

        if generics.params.is_empty() {
            return tokens;
        }

        let mut trailing_or_empty = true;
        for param in generics.params.pairs() {
            if let GenericParam::Lifetime(_) = *param.value() {
                trailing_or_empty = param.punct().is_some();
            }
        }
        for param in generics.params.pairs() {
            if let GenericParam::Lifetime(_) = **param.value() {
                continue;
            }
            if !trailing_or_empty {
                <Token![,]>::default().to_tokens(&mut tokens);
                trailing_or_empty = true;
            }
            match *param.value() {
                GenericParam::Lifetime(_) => unreachable!(),
                GenericParam::Type(param) => {
                    // Leave off the type parameter defaults
                    param.ident.to_tokens(&mut tokens);
                }
                GenericParam::Const(param) => {
                    // Leave off the const parameter defaults
                    param.ident.to_tokens(&mut tokens);
                }
            }
            param.punct().to_tokens(&mut tokens);
        }
        <Token![,]>::default().to_tokens(&mut tokens);
        tokens
    }

    /// Tokenize parameters for `impl` blocks.
    /// It doesn't contain outer brackets, but lifetimes and trait bounds.
    pub fn tokenize_impl(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        let generics = &self.generics;

        let mut trailing_or_empty = true;
        for param in generics.params.pairs() {
            if let GenericParam::Lifetime(_) = **param.value() {
                param.to_tokens(&mut tokens);
                trailing_or_empty = param.punct().is_some();
            }
        }
        for param in generics.params.pairs() {
            if let GenericParam::Lifetime(_) = **param.value() {
                continue;
            }
            if !trailing_or_empty {
                <Token![,]>::default().to_tokens(&mut tokens);
                trailing_or_empty = true;
            }
            match *param.value() {
                GenericParam::Lifetime(_) => unreachable!(),
                GenericParam::Type(param) => {
                    // Leave off the type parameter defaults
                    tokens.append_all(param.attrs.iter().filter(|attr| match attr.style {
                        AttrStyle::Outer => true,
                        AttrStyle::Inner(_) => false,
                    }));
                    param.ident.to_tokens(&mut tokens);
                    if !param.bounds.is_empty() {
                        if let Some(t) = &param.colon_token {
                            t.to_tokens(&mut tokens)
                        }

                        param.bounds.to_tokens(&mut tokens);
                    }
                }
                GenericParam::Const(param) => {
                    // Leave off the const parameter defaults
                    tokens.append_all(param.attrs.iter().filter(|attr| match attr.style {
                        AttrStyle::Outer => true,
                        AttrStyle::Inner(_) => false,
                    }));
                    param.const_token.to_tokens(&mut tokens);
                    param.ident.to_tokens(&mut tokens);
                    param.colon_token.to_tokens(&mut tokens);
                    param.ty.to_tokens(&mut tokens);
                }
            }
            param.punct().to_tokens(&mut tokens);
        }
        if !tokens.is_empty() {
            <Token![,]>::default().to_tokens(&mut tokens);
        }
        tokens
    }
}
