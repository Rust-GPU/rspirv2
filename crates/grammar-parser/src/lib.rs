#![doc = include_str!("../README.md")]

pub mod parse;
pub mod timer;

#[cfg(feature = "codegen")]
pub mod codegen;

#[cfg(feature = "codegen")]
pub use proc_macro2;
#[cfg(feature = "codegen")]
pub use quote;
