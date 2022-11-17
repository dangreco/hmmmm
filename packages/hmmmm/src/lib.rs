#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

//! # hmmmm 🤔
//! Yet another aptly-named Hidden-Markov Model library for Rust.
//!
//! **Important Note:** This crate depends on the feature `generic_const_exprs`. For now, usage of this crate is limited to use with the `nightly` Rust channel.

mod hmm;

/// HMM-related algorithms (e.g. Viterbi for MAP estimates)
pub mod algorithms;

/// Trait definitions
pub mod prelude;

/// Convenience functions for constructing HMMs
pub mod utils;

pub use hmm::*;

#[cfg(feature = "hmmmm_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate hmmmm_derive;
#[cfg(feature = "hmmmm_derive")]
#[doc(hidden)]
pub use hmmmm_derive::*;