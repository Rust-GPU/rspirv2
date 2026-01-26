# rspirv2

A strongly typed SPIR-V representation with a builder, disassembler and support for custom instruction sets. Build primarily for [rust-gpu](https://github.com/Rust-GPU/rust-gpu/). 

## Design
* strongly typed: Each instruction is their own struct with pub fields for each operant (including return word and return type, where required), to ensure each are assigned the correct type of operant at compile time. (Sort of like [`rspirv::sr::Op`](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/rspirv/sr/autogen_ops.rs#L67) but you can't encode them into binary)
* Bring your own builder: The `Builder` is entirely optional and only exists for convenience. Every instruction can be turned into a `Iterator<Type=u32>` and written to any buffer you want.
* Instruction metadata: For each instruction, you can query the metadata about expected argument types, [like in rspirv](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/rspirv/grammar/autogen_table.rs#L80). This type querying system is dynamic and separate from the generated structs. Needed for disassembly and `asm!` type inference in rust-gpu.
* Custom instruction sets: You can choose which instruction set extensions you operate on, whether it's `glsl_std_450`, `opencl_std_100` or a completely custom externally defined instruction set, e.g. for rust-gpu.
* Tooling: A disassembler that doesn't fail with unknown instruction sets and can support custom extensions.

### Compared to [rspirv](https://github.com/gfx-rs/rspirv/)

* Hasn't seen an update in [over 2 years](https://crates.io/crates/rspirv/versions), last commit 4 months ago. Still no Vulkan 1.4 support, only in an open [PRs](https://github.com/gfx-rs/rspirv/pull/262).
* [It forces you to use their builder to append instructions](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/rspirv/dr/build/autogen_norm_insts.rs#L21-L63), which is also a singleton cursor. Not having multiple cursors is causing pain [within rust-gpu](https://github.com/Rust-GPU/rust-gpu/blob/8b85962188181817aaaf3aee431677e078ff2a68/crates/rustc_codegen_spirv/src/builder_spirv.rs#L399-L430) for ages.
* If you're not using the builder, you must [construct instructions without argument type checking](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/rspirv/dr/build/autogen_norm_insts.rs#L21-L63)
* [Their instruction encode](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/rspirv/binary/assemble.rs#L4-L16) requires you to pass a `&mut Vec<u32>`, which can limit what kinds of buffers you can write to quite significantly.
* Every [Instruction contains a non-optional Vec](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/rspirv/dr/constructs.rs#L84-L95), with no pooling or arena, potentially leading to a lot of heap fragmentation.
* rust-gpu's custom [instructions](https://github.com/Rust-GPU/rust-gpu/blob/8b85962188181817aaaf3aee431677e078ff2a68/crates/rustc_codegen_spirv/src/custom_insts.rs) and [decorations](https://github.com/Rust-GPU/rust-gpu/blob/8b85962188181817aaaf3aee431677e078ff2a68/crates/rustc_codegen_spirv/src/custom_decorations.rs) are messy and `spirv-dis` fails to disassemble them. We need a new system that allows custom out-of-tree instruction sets.
