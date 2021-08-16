use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Expr, Fields, Generics, Type, Visibility};

#[derive(Clone)]
pub struct Field {
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
                    ident: f.ident.unwrap(),
                    ty: f.ty,
                    expr: Some(attr.parse_args().unwrap()),
                }),
                None => required_fields.push(Field {
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
        let builder_name = Ident::new(&format!("{}Builder", self.ident), Span::call_site());
        let all_generics = self.all_generics().collect::<Vec<TokenStream>>();
        tokens.extend(quote! {
            struct #builder_name <#(#all_generics),*> {
                _phantom: ::std::marker::PhantomData<(#(#all_generics),*)>,
            }
        });
        println!(
            "{} {}",
            self.required_fields.len(),
            self.optional_fields.len()
        );
    }
}

// Generate an iterator for generics like [T1, T2, ...]
impl StructureInput {
    fn all_generics(&self) -> impl Iterator<Item = TokenStream> {
        (0..(self.required_fields.len() + self.optional_fields.len()))
            .into_iter()
            .map(|i| TokenStream::from_str(&format!("T{}", i + 1)).unwrap())
    }
}
