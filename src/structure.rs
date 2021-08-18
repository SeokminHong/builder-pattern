use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{
    AttrStyle, Data, DeriveInput, Expr, Fields, GenericParam, Generics, Token, Type, Visibility,
};

#[derive(Clone)]
pub struct Field {
    pub vis: Visibility,
    pub ident: Ident,
    pub ty: Type,
    pub expr: Option<Expr>,
}

pub struct StructureInput {
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub required_fields: Vec<Field>,
    pub optional_fields: Vec<Field>,
}

impl Parse for StructureInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let input: DeriveInput = input.parse()?;
        let vis = input.vis;
        let ident = input.ident;
        let generics = input.generics;
        let data_struct = if let Data::Struct(d) = input.data {
            d
        } else {
            unimplemented!("Only structures are supported!");
        };
        let fields = if let Fields::Named(f) = data_struct.fields {
            f
        } else {
            unimplemented!("Only structures are supported!");
        };
        let mut optional_fields: Vec<Field> = vec![];
        let mut required_fields: Vec<Field> = vec![];
        for f in fields.named.into_iter() {
            // Having "default" attribute
            match f.attrs.iter().find(|attr| attr.path.is_ident("default")) {
                Some(attr) => optional_fields.push(Field {
                    vis: f.vis,
                    ident: f.ident.unwrap(),
                    ty: f.ty,
                    expr: Some(attr.parse_args().unwrap()),
                }),
                None => required_fields.push(Field {
                    vis: f.vis,
                    ident: f.ident.unwrap(),
                    ty: f.ty,
                    expr: None,
                }),
            };
        }
        Ok(StructureInput {
            vis,
            ident,
            generics,
            required_fields,
            optional_fields,
        })
    }
}

impl ToTokens for StructureInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let vis = &self.vis;

        let impl_tokens = self.tokenize_impl();
        let ty_tokens = self.tokenize_types();
        let (_, ty_generics, where_clause) = self.generics.split_for_impl();
        let lifetimes = self
            .generics
            .lifetimes()
            // Remove bounds
            .map(|f| f.lifetime.to_token_stream())
            .collect::<Vec<TokenStream>>();

        let builder_name = Ident::new(&format!("{}Builder", self.ident), Span::call_site());

        let all_generics = self.all_generics().collect::<Vec<TokenStream>>();
        let empty_generics = self.empty_generics();

        let optional_generics = self.optional_generics();
        let satisfied_generics = self.satified_generics();

        let builder_fields = self.builder_fields();
        let builder_init_args = self.builder_init_args();

        let struct_init_args = self.struct_init_args();

        let builder_functions = self.builder_functions(&builder_name, &lifetimes, &ty_tokens);

        tokens.extend(quote! {
            #vis struct #builder_name <#impl_tokens #(#all_generics),*> #where_clause {
                _phantom: ::std::marker::PhantomData<(#ty_tokens #(#all_generics),*)>,
                #(#builder_fields),*
            }
            impl <#impl_tokens> #ident #ty_generics #where_clause {
                #vis fn new() -> #builder_name<#(#lifetimes,)* #ty_tokens #(#empty_generics),*> {
                    #builder_name {
                        _phantom: ::std::marker::PhantomData,
                        #(#builder_init_args),*
                    }
                }
            }
            impl <#impl_tokens #(#optional_generics,)*> #builder_name <#(#lifetimes,)* #ty_tokens #(#satisfied_generics),*>
                #where_clause
            {
                #vis fn build(self) -> #ident #ty_generics {
                    #ident {
                        #(#struct_init_args),*
                    }
                }
            }
            #(#builder_functions)*
        });
    }
}

impl StructureInput {
    /// An iterator for generics like [U1, U2, ...].
    fn all_generics(&self) -> impl Iterator<Item = TokenStream> {
        (0..(self.required_fields.len() + self.optional_fields.len()))
            .into_iter()
            .map(|i| TokenStream::from_str(&format!("U{}", i + 1)).unwrap())
    }

    /// An iterator to describe initial state of builder.
    fn empty_generics(&self) -> impl Iterator<Item = TokenStream> {
        (0..(self.required_fields.len() + self.optional_fields.len()))
            .into_iter()
            .map(|_| TokenStream::from_str("()").unwrap())
    }

    /// An iterator for optional fields.
    fn optional_generics(&self) -> impl Iterator<Item = TokenStream> {
        let offset = self.required_fields.len() + 1;
        (0..self.optional_fields.len())
            .into_iter()
            .map(move |i| TokenStream::from_str(&format!("U{}", i + offset)).unwrap())
    }

    /// An iterator to describe when the builder has enough types to build the struct.
    fn satified_generics(&'_ self) -> impl '_ + Iterator<Item = TokenStream> {
        self.required_fields
            .iter()
            .map(|f| {
                let ty = &f.ty;
                quote! {#ty}
            })
            .chain(self.optional_generics())
    }

    /// An iterator for fields of the builder.
    fn builder_fields(&'_ self) -> impl '_ + Iterator<Item = TokenStream> {
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

    /// An iterator for initialize arguments of the builder.
    /// Required fields are filled with `None`, optional fields are filled with given value via `default` attribute.
    fn builder_init_args(&'_ self) -> impl '_ + Iterator<Item = TokenStream> {
        self.required_fields
            .iter()
            .map(|f| {
                let ident = &f.ident;
                quote! {
                    #ident: None
                }
            })
            .chain(self.optional_fields.iter().map(|f| {
                let (ident, expr) = (&f.ident, &f.expr);
                quote_spanned! { expr.span() =>
                    #ident: Some(#expr)
                }
            }))
    }

    /// An iterator to express initialize statements.
    fn struct_init_args(&'_ self) -> impl '_ + Iterator<Item = TokenStream> {
        self.required_fields
            .iter()
            .chain(self.optional_fields.iter())
            .map(|f| {
                let ident = &f.ident;
                quote! {
                    #ident: self.#ident.unwrap()
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
                quote! {
                    impl <#impl_tokens #(#other_generics,)*> #builder_name <#(#lifetimes,)* #ty_tokens #(#before_generics),*>
                        #where_clause
                    {
                        #vis fn #ident<InToType: Into<#ty>>(mut self, value: InToType) -> #builder_name <#(#lifetimes,)* #ty_tokens #(#after_generics),*> {
                            #builder_name {
                                _phantom: ::std::marker::PhantomData,
                                #(#builder_fields),*
                            }
                        }
                    }
                }
            })
    }

    fn tokenize_types(&self) -> TokenStream {
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
    fn tokenize_impl(&self) -> TokenStream {
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
