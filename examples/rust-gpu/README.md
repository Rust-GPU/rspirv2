# rust-gpu custom ISA example

This example showcases how to create a custom rspirv2 instruction set (ISA) and use rspirv2's retargetable disassembler.
We'll use rust-gpu's custom spirv instructions as an example and disassemble intermediary artifacts of rust-gpu. 

> [!WARNING]
> Rust-GPU's ISA is considered unstable and may change at any point. This example has been written against version 
> [`v0.10.0-alpha.1`](https://crates.io/crates/spirv-std/0.10.0-alpha.1). As rspirv2 matures and is integrated into 
> rust-gpu, it is highly likely that the internal ISA will change *a lot*.

## Background on rust-gpu compilation

rust-gpu's compiler consists of 3 steps:
* codegen: function calls from `rustc_codegen_ssa` are translated to spirv
* linker: a custom spirv linker links multiple spv modules into one spv module with some deduplication
* post-link legalization: many passes to legalize the emitted spirv, many of which require full-code analysis

Only the final post-link legalization step emits "valid" spirv code according to the spirv spec. Many of these passes do
require additional information from codegen, and only some of those are encodable using standard spirv instructions. The
remaining are encoded in custom instructions emitted at codegen and cleaned up during the post-link analysis, that 
variant of spirv is called the "rust-gpu custom ISA". 

Since the standard C++ spirv tools were only build to deal with whatever is in the spirv standard, they can't 
disassemble our custom instructions. Technically they could skip over unknown instructions and keep decoding, but 
instead they like to just error out completely, making inspection of any intermediary step difficult.

One of the foundational reasons rspirv2 was created is to ease this friction by:
* Providing a grammar declaration of the custom instructions instead of relying upon hacks to define custom instructions
  and modify existing enumerants
* Provide tooling to disassemble custom instruction sets, given the grammar of the ISA, and gracefully handle unknown 
  instructions



## Getting internal artifacts

You will find some internal artifacts in `./dump`, or dump one yourself with:

`RUSTGPU_CODEGEN_ARGS="--dump-pre-inline=dump" cargo run --bin example-runner-ash`

Setting this env var will dump the spirv module into the `<cwd>/dump` after merging, deduplication and the first DCE, 
but before any post-link legalization is run. There's a [number of other options] to dump at different points in 
the pipeline, but we've chosen pre-inline as all the custom instructions haven't been removed yet and the first DCE 
doesn't blow up the dump size.



## Custom Decorations

Rust-GPU has two [custom decorations], but doesn't store them as actual decorations. Instead, it uses the existing 
`OpDecorateString` with the `UserTypeGOOGLE` decorator that accepts a string, and uses a single-char prefix to determine
what kind of string it is. This makes it rather uninteresting for this use-case.



## Custom instructions

We'll use `./dump/sky_shader.spv` as an example, dump it with `cargo dis ./dump/sky_shader.spv > sky_shader.txt`.

```
%52 = OpExtInstImport "Rust.rustc_codegen_spirv.0_10_0.27bf7c44931bdc5f7dac04afcc1229f2"
%298 = OpExtInst %void %52 0 %9 %u32_132 %u32_132 %u32_18 %u32_37
```

Rust-GPU defines its custom instructions as an "Extended Instruction Set" (`ExtInstSet` in rspirv2), which all have a
unique name, this one being put together with some hashes to check for validity. They're then used in `OpExtInst` 
instructions.
 
There are [5 instructions] Rust-GPU defines. The most common one is `0`, declaring the source location of the following
instruction, encoded as `file, line_start, line_end, col_start, col_end`. By searching for `%52 4` you'll also find a 
single use of `OpAbort`, which indicates a Rust panic.



[number of other options]: https://github.com/Rust-GPU/rust-gpu/blob/v0.10.0-alpha.1/docs/src/codegen-args.md#debugging-codegen-args-flagsoptions
[custom decorations]: https://github.com/Rust-GPU/rust-gpu/blob/e2e4d529a5ead1364228530d301ebbca4ef90262/crates/rustc_codegen_spirv/src/custom_decorations.rs#L18-L31
[5 instructions]: https://github.com/Rust-GPU/rust-gpu/blob/v0.10.0-alpha.1/crates/rustc_codegen_spirv/src/custom_insts.rs#L161-L202
