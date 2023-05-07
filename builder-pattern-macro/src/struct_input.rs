use crate::attributes::{ident_add_underscore, FieldAttributes, FieldVisibility};
use crate::builder::{
    builder_decl::BuilderDecl, builder_functions::BuilderFunctions, builder_impl::BuilderImpl,
};
use crate::field::Field;
use crate::struct_impl::StructImpl;

use core::str::FromStr;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Result},
    AttrStyle, Attribute, Data, DeriveInput, Fields, GenericParam, Generics, Lifetime, Token,
    VisPublic, Visibility,
};
use syn::{Path, PredicateType, Type, TypePath, WhereClause, WherePredicate};

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
                vis: if attrs.vis == FieldVisibility::Public {
                    let v = <Token![pub]>::default();
                    Visibility::Public(VisPublic { pub_token: v })
                } else {
                    f.vis
                },
                ident: f
                    .ident
                    .unwrap_or_else(|| unimplemented!("Fields must have an identifier!")),
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

        let builder_func = BuilderFunctions::new(self);
        builder_func.to_tokens(tokens);
    }
}

impl StructInput {
    pub fn num_fields(&self) -> usize {
        self.required_fields.len() + self.optional_fields.len()
    }

    /// Name of the builder structure.
    pub fn builder_name(&self) -> Ident {
        Ident::new(&format!("{}Builder", self.ident), Span::call_site())
    }

    pub fn fn_lifetime(&self) -> Lifetime {
        Lifetime::new("'fn_lifetime", Span::call_site())
    }

    /// Get token stream for lifetimes.
    pub fn lifetimes(&self) -> Vec<TokenStream> {
        self.generics
            .lifetimes()
            // Remove bounds
            .map(|f| f.lifetime.to_token_stream())
            .collect()
    }

    /// An iterator for generics like [U1, U2, ...].
    pub fn all_generics(&self) -> impl Iterator<Item = TokenStream> {
        (0..(self.num_fields()))
            .into_iter()
            .map(|i| TokenStream::from_str(&format!("TyBuilderPattern{}", i + 1)).unwrap())
    }

    /// An iterator for fields of the builder.
    pub fn builder_fields<'a>(
        &'a self,
        fn_lifetime: &'a Lifetime,
    ) -> impl 'a + Iterator<Item = TokenStream> {
        self.required_fields
            .iter()
            .chain(self.optional_fields.iter())
            .map(move |f| {
                let (ident, ty) = (&f.ident, &f.ty);
                quote! {
                    #ident: Option<::builder_pattern::setter::Setter<#fn_lifetime, #ty>>
                }
            })
    }

    /// Tokenize type parameters.
    /// It skips lifetimes and has no outer brackets.
    pub fn tokenize_types(&self, replace_generics: &[Ident], omit_replaced: bool) -> TokenStream {
        let generics = &self.generics;
        let mut tokens = TokenStream::new();

        if generics.params.is_empty() {
            return tokens;
        }
        if omit_replaced
            && generics.params.iter().all(|x| match x {
                GenericParam::Type(param) => replace_generics.contains(&param.ident),
                GenericParam::Const(_) => false,
                _ => true,
            })
        {
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
                    if replace_generics.contains(&param.ident) {
                        if omit_replaced {
                            continue;
                        }
                        ident_add_underscore(&param.ident).to_tokens(&mut tokens);
                    } else {
                        param.ident.to_tokens(&mut tokens);
                    }
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

    pub fn setter_where_clause(&self, replace_generics: &[Ident]) -> TokenStream {
        let mut stream = TokenStream::new();
        if replace_generics.is_empty() {
            return stream;
        }
        let clauses = self
            .generics
            .where_clause
            .iter()
            .flat_map(|where_clause: &WhereClause| {
                where_clause
                    .predicates
                    .iter()
                    .filter_map(|x: &WherePredicate| match x {
                        WherePredicate::Type(PredicateType {
                            bounded_ty: Type::Path(TypePath { qself: None, path }),
                            bounds,
                            lifetimes,
                            colon_token,
                            ..
                        }) => {
                            if let Some(ident) = path.get_ident() {
                                let ident_ = ident.to_string() + "_";
                                let replacement_ident = Ident::new(&ident_, ident.span());
                                let pred = WherePredicate::Type(PredicateType {
                                    bounded_ty: Type::Path(TypePath {
                                        qself: None,
                                        path: Path::from(replacement_ident),
                                    }),
                                    bounds: bounds.clone(),
                                    lifetimes: lifetimes.clone(),
                                    colon_token: colon_token.clone(),
                                });
                                Some(quote! { #pred })
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
            });
        stream.extend(quote! { where });
        stream.append_terminated(clauses, quote! { , });
        stream
    }

    /// Tokenize parameters for `impl` blocks.
    /// It doesn't contain outer brackets, but lifetimes and trait bounds.
    pub fn tokenize_impl(&self, filter_out: &[Ident]) -> TokenStream {
        let mut tokens = TokenStream::new();
        let generics = &self.generics;

        if generics.params.is_empty() {
            return tokens;
        }
        if generics.params.iter().all(|x| match x {
            GenericParam::Type(param) => filter_out.contains(&param.ident),
            GenericParam::Const(_) => false,
            _ => true,
        }) {
            return tokens;
        }

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
                    if filter_out.contains(&param.ident) {
                        continue;
                    }
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
