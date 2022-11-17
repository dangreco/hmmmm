use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::helpers::non_enum_error;

pub(crate) fn derive_state_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
  let name = &ast.ident;
  let gen = &ast.generics;
  let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

  if gen.lifetimes().count() > 0 {
    return Err(syn::Error::new(
      Span::call_site(),
      "This macro doesn't support enums with lifetimes. \
       The resulting enums would be unbounded.",
    ));
  }
  let variants = match &ast.data {
    Data::Enum(v) => &v.variants,
    _ => return Err(non_enum_error()),
  };

  let n = variants.len();

  let mut arms = Vec::new();
  let mut idx = 0usize;
  for variant in variants {
    let ident = &variant.ident;
    let params = match &variant.fields {
      Fields::Unit => quote! {},
      Fields::Unnamed(fields) => {
        let defaults = ::core::iter::repeat(quote!(::core::default::Default::default()))
          .take(fields.unnamed.len());
        quote! { (#(#defaults),*) }
      }
      Fields::Named(fields) => {
        let fields = fields
          .named
          .iter()
          .map(|field| field.ident.as_ref().unwrap());
        quote! { {#(#fields: ::core::default::Default::default()),*} }
      }
    };

    arms.push(quote! {#idx => ::core::option::Option::Some(#name::#ident #params)});
    idx += 1;
  }

  Ok(quote! {
    impl #impl_generics hmmmm::prelude::State for #name #ty_generics #where_clause {
      const COUNT: usize = #n;

      fn as_usize(&self) -> usize {
        (*self as u8) as usize
      }

      fn from_usize(idx: usize) -> Self {
        match idx {
          #(#arms),*,
          _ => panic!()
        }.unwrap()
      }
    }
  })
}
