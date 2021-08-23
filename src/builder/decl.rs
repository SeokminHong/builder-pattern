use crate::structure::StructInput;

use proc_macro2::TokenStream;
use quote::ToTokens;

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
        let vis = &self.input.vis;
        let builder_name = self.input.builder_name();
        let where_clause = &self.input.generics.where_clause;

        let impl_tokens = self.input.tokenize_impl();
        let all_generics = self.input.all_generics().collect::<Vec<TokenStream>>();
        let ty_tokens = self.input.tokenize_types();

        let builder_fields = self.input.builder_fields();

        tokens.extend(quote! {
            #vis struct #builder_name <#impl_tokens #(#all_generics),*> #where_clause {
                _phantom: ::std::marker::PhantomData<(#ty_tokens #(#all_generics),*)>,
                #(#builder_fields),*
            }
        });
    }
}
