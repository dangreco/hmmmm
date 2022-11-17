extern crate proc_macro;

mod helpers;
mod observation;
mod state;

use observation::derive_observation_inner;
use state::derive_state_inner;
use syn::DeriveInput;

#[proc_macro_derive(State)]
pub fn derive_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse_macro_input!(input as DeriveInput);

  let toks = derive_state_inner(&ast).unwrap_or_else(|err| err.to_compile_error());

  toks.into()
}

#[proc_macro_derive(Observation)]
pub fn derive_observation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse_macro_input!(input as DeriveInput);

  let toks = derive_observation_inner(&ast).unwrap_or_else(|err| err.to_compile_error());

  toks.into()
}
