//! Rust structs containing metadata information about SPIR-V instructions, directly deserialized from the SPIR-V
//! grammar JSON.
//!
//! These structs are a derivative of:
//! * [rspirv](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/autogen/src/structs.rs)
//! * [spirt](https://github.com/Rust-GPU/spirt/blob/f6924328d604503445d470a6676ade916b6ba3dc/src/spv/spec.rs#L1104)

mod capability;
mod extension;
mod grammar;
mod instruction;
mod operand_kind;

pub use capability::*;
pub use extension::*;
pub use grammar::*;
pub use instruction::*;
pub use operand_kind::*;
