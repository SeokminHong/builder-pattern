use crate::struct_input::StructInput;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_quote;

/// Declaration of the builder structure.
pub struct BuilderDecl<'a> {
    input: &'a StructInput,
}

impl<'a> BuilderDecl<'a> {
    pub fn new(input: &'a StructInput) -> BuilderDecl<'a> {
        BuilderDecl { input }
    }
}

impl<'a> ToTokens for BuilderDecl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.input.ident;
        let vis = &self.input.vis;
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;

        let impl_tokens = self.input.tokenize_impl();
        let all_generics = self.input.all_generics().collect::<Vec<TokenStream>>();
        let ty_tokens = self.input.tokenize_types();

        let fn_lifetime = self.input.fn_lifetime();
        let builder_fields = self.input.builder_fields(&fn_lifetime);

        let docs = format!(" A builder for `{}`.", ident);
        let docs: TokenStream = parse_quote!(#[doc=#docs]);

        tokens.extend(quote! {
            #docs
            #vis struct #builder_name<
                #fn_lifetime,
                #impl_tokens
                #(#all_generics,)*
                AsyncFieldMarker,
                ValidatorOption,
                ValidatorDefault
            > #where_clause {
                _phantom: ::std::marker::PhantomData<(
                    #ty_tokens
                    #(#all_generics,)*
                    AsyncFieldMarker,
                    ValidatorOption,
                    ValidatorDefault
                )>,
                #(#builder_fields),*
            }
        });
    }
}
