# rspirv2-types

Defines types and traits to describe SPIR-V Instructions and Operands. Part of [rspirv2](https://crates.io/crates/rspirv2).

The crate doesn't include the core grammar or any of the extended instruction sets defined by the SPIR-V grammar json, it merely defines the types to represent them. The main crate `rspirv2` reexports this crate and includes auto generated definitions of the core grammar, with some select Vulkan-specific extended instruction sets.
