use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
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
            unimplemented!();
        };
        let fields = if let Fields::Named(f) = data_struct.fields {
            f
        } else {
            unimplemented!();
        };
        let mut optional_fields: Vec<Field> = vec![];
        let mut required_fields: Vec<Field> = vec![];
        for f in fields.named.into_iter() {
            println!("{}", f.ident.to_token_stream().to_string());
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
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let builder_name = Ident::new(&format!("{}Builder", self.ident), Span::call_site());
        tokens.extend(quote! {
            struct #builder_name {}
        });
    }
}
