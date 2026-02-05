#![doc = include_str!("../README.md")]

pub mod parse;

#[cfg(feature = "codegen")]
pub mod codegen;

pub use proc_macro2;
pub use quote;
