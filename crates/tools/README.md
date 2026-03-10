# rspirv2-tools

Similar to the C++ SPIR-V tools, provides cmdline utilities for working with SPIR-V, but written in rust with `rspirv2`.

This crate provides the following binaries:
* `rspirv2-dis`

## Custom ISA

This crate exposes binaries using the default "SPIR-V core" instruction set. The binaries themselves are just thin 
wrappers, the actual implementations are situated in the `spirv2-tools-gen` crate and are **gen**eric over the 
instruction set they use. To create binaries that work with your custom ISA, copy this crate and change the 
`type ToolsISA` in `lib.rs` to your custom ISA.

These tools should function on any custom ISA you define, as long as you *extend* the core instruction set. If you're 
modifying or removing things from core SPIR-V ISA, you may encounter issues with some tooling, as they may expect 
certain "core" instructions to be present in your instruction set. 
