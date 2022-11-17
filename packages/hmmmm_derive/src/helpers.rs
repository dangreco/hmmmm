use proc_macro2::{Delimiter, Ident, Span, TokenStream, TokenTree};
use quote::quote;
use syn::{
  punctuated::Punctuated, token::Comma, AttrStyle, Data, DeriveInput, Fields, Generics, Variant,
};

fn non_enum_error() -> syn::Error {
  syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

fn non_u8_error() -> syn::Error {
  syn::Error::new(
    Span::call_site(),
    "This macro only supports enums with the #[repr(u8)] flag.",
  )
}

fn lifetimes_error() -> syn::Error {
  syn::Error::new(
    Span::call_site(),
    "This macro doesn't support enums with lifetimes. \
     The resulting enums would be unbounded.",
  )
}

pub fn check_u8<'a>(ast: &'a DeriveInput) -> syn::Result<()> {
  let has_u8 = ast
    .attrs
    .iter()
    .find(|attr| {
      let tokens: Vec<TokenTree> = attr.tokens.clone().into_iter().collect();

      attr.style == AttrStyle::Outer
        && attr.path.leading_colon == None
        && attr.path.segments.len() == 1
        && attr.path.segments[0].ident.to_string() == "repr"
        && tokens.len() == 1
        && match &tokens[0] {
          TokenTree::Group(group) => {
            let stream: Vec<TokenTree> = group.stream().into_iter().collect();

            group.delimiter() == Delimiter::Parenthesis
              && stream.len() == 1
              && match &stream[0] {
                TokenTree::Ident(ident) => ident.to_string() == "u8",
                _ => false,
              }
          }
          _ => false,
        }
    })
    .is_some();

  if has_u8 {
    Ok(())
  } else {
    Err(non_u8_error())
  }
}

pub fn get_enum_variants<'a>(ast: &'a DeriveInput) -> syn::Result<&'a Punctuated<Variant, Comma>> {
  if ast.generics.lifetimes().count() > 0 {
    Err(lifetimes_error())
  } else {
    match &ast.data {
      Data::Enum(v) => Ok(&v.variants),
      _ => Err(non_enum_error()),
    }
  }
}

pub fn enum_variants_to_match_arms(
  name: &Ident,
  variants: &Punctuated<Variant, Comma>,
) -> Vec<TokenStream> {
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

  arms
}

pub fn base_enum_implementation(n: usize, arms: &Vec<TokenStream>) -> TokenStream {
  quote! {
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

    fn as_u8(&self) -> u8 {
      *self as u8
    }

    fn from_u8(value: u8) -> Self {
      Self::from_usize(value as usize)
    }
  }
}

pub fn copy_clone_implementation(name: &Ident, generics: &Generics) -> TokenStream {
  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  quote! {
    impl #impl_generics Copy for #name #ty_generics #where_clause { }

    impl #impl_generics Clone for #name #ty_generics #where_clause {
      fn clone(&self) -> #name {
          *self
      }
    }
  }
}
