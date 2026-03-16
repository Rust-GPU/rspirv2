#![doc = include_str!("../README.md")]

pub mod binary;
pub mod dis;
pub mod inst;
pub mod meta;
pub mod module;
pub mod operand;
mod word;

pub use word::*;
