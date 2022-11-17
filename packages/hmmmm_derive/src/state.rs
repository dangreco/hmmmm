use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput};

use crate::helpers::{
  base_enum_implementation, copy_clone_implementation, enum_variants_to_match_arms,
  get_enum_variants, check_u8,
};

pub(crate) fn derive_state_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
  let name = &ast.ident;
  let gen = &ast.generics;
  let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

  check_u8(ast)?;

  let variants = get_enum_variants(ast)?;
  let n = variants.len();
  let arms = enum_variants_to_match_arms(name, variants);

  let base_implementation = base_enum_implementation(n, &arms);
  let copy_clone = copy_clone_implementation(name, gen);

  Ok(quote! {
    impl #impl_generics hmmmm::prelude::State for #name #ty_generics #where_clause {
      #base_implementation
    }

    #copy_clone
  })
}
