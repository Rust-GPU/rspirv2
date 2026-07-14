# rspirv2

[![Crates.io Version](https://img.shields.io/crates/v/rspirv2)](https://crates.io/crates/rspirv2)

SPIR-V is the binary format for Vulkan shaders (and `OpenCL` kernels). This crate provides a strongly typed SPIR-V representation for all instructions, efficient encoded instruction storage, manipulation tools, disassembler and support for custom instruction sets. Built primarily for [rust-gpu](https://github.com/Rust-GPU/rust-gpu/). 

Created and sponsored by [<img src="./docs/vectorware_logo.png" alt="Vectorware" style="max-width: 12pt; height: 12pt;"> Vectorware](https://www.vectorware.com/).

```rust
// an allocator for `IdResult`s (SSA value IDs)
let mut alloc = IdResultAlloc::new();
// a `Vec` for instructions that stores them in SPIR-V encoded form
let mut vec = InstVec::<CoreInstSet>::new();

// add some SPIR-V instructions
// declare u32 type
let u32 = vec.push_inst(OpTypeInt {
    id_result: alloc.alloc_id(),
    width: LiteralInteger::new(32),
    signedness: LiteralInteger::new(0),
});
// let a: u32 = 42;
let a = vec.push_inst(OpConstant {
    id_result_type: IdResultType(u32),
    id_result: alloc.alloc_id(),
    value: LiteralConst::from(42u32),
});
// let b: u32 = a + a;
let b = vec.push_inst(OpIAdd {
    id_result_type: IdResultType(u32),
    id_result: alloc.alloc_id(),
    operand_1: IdRef(a),
    operand_2: IdRef(a),
});

// replace `b = a + a` with `b = a * a`
let mut modified = vec
    .iter()
    .map(|inst| match inst {
        CoreInstSet::IAdd(OpIAdd {
            id_result,
            operand_1,
            operand_2,
            ..
        }) => OpIMul {
            id_result_type: IdResultType(u32),
            id_result,
            operand_1,
            operand_2,
        }
        .into(),
        inst => inst,
    })
    .collect::<InstVec<_>>();

// append InstVec to each other
let mut vec2 = InstVec::new();
vec2.push_inst(OpISub {
    id_result_type: IdResultType(u32),
    id_result: alloc.alloc_id(),
    operand_1: IdRef(a),
    operand_2: IdRef(b),
});
modified.append(&mut vec2);

// disassembly with various settings:
// `default()` for colorful terminal output
// `simple()` to remove color and padding
// `like_spirv_tools()` and `like_rspirv()` to mimic output of other disassemblers
let disassembly = format!("{}", modified.dis(DisOptions::simple()));
expect_test::expect![[r#"
    %u32 = OpTypeInt 32 0
    %u32_42 = OpConstant %u32 42
    %2 = OpIMul %u32 %u32_42 %u32_42
    %3 = OpISub %u32 %u32_42 %2
"#]]
.assert_eq(&disassembly);
```

## Features
* strongly typed: Each instruction has their own struct with pub fields for each operand, to make it easy to access and modify them. The `CoreInstSet` type is a plain enum of all available instructions, which you can just match on.
* efficient storage: A `Vec<CoreInstSet>` consumes a lot of memory as it needs to reserve space for the largest instruction. `InstVec` store instructions efficiently in their variable-length SPIR-V encoded form, but you can't trivially replace instructions.
* iteration and slicing: When you iterate over the `InstVec`, it'll decode instructions on the fly, so it doesn't look any different to a `Vec`. It also gives you various offsets to the instructions, so you can slice it as you'd slice a string.
* bring your own builder: You're not locked into using `InstVec` to encode instructions, you can implement `trait WordWriter` to write the binary form into your own data structure.
* custom instruction sets: We codegen all instructions from the SPIR-V grammar. This makes updating to newer versions easy, and by modifying the grammar or creating your own you can make custom instruction sets.  
* tooling: A disassembler that doesn't fail with unknown instruction and supports custom instruction sets as a generic.

### Caveats
* Assumes SPIR-V instructions can be encoded, decoded and processed independently without requiring context from any other instruction. This is true for almost all instructions, except for `OpSwitch` and `ExtInstSet`.
* `OpSwitch` has a custom implementation where anyone accessing must first define whether the labels are 32 or 64bit wide.
* Extended instruction sets (`ExtInstSet`) are not properly supported. We do of course support declaring `ExtInstSet` and calling their functions, but the API will say you're calling function id 69. You'll need to resolve that yourself, e.g. if the associated `ExtInstSet` declaration is `glsl.std.450`, it's the vector `Normalize()` function from glsl.
* No SPIR-V assembler
* Contributions welcome!

### Compared to [rspirv](https://github.com/gfx-rs/rspirv/)
* rspirv has a [structured representation](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/rspirv/sr/autogen_ops.rs#L67), which you can only load from a binary, but not write back into a binary.
* [rspirv's builder is a singleton cursor](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/rspirv/dr/build/autogen_norm_insts.rs#L21-L63), rust-gpu needs multiple cursors, so [working around this limitation has been awkward](https://github.com/Rust-GPU/rust-gpu/blob/8b85962188181817aaaf3aee431677e078ff2a68/crates/rustc_codegen_spirv/src/builder_spirv.rs#L399-L430). In rspirv2, you can have as many `InstVec` as you want and concat them together arbitrarily.
* Accessing operands of instructions is dynamic, so to [modify SPIR-V](https://github.com/Rust-GPU/rust-gpu/blob/c2de01373cc65372a2d438a8bf1dec1465fe450d/crates/rustc_codegen_spirv/src/linker/simple_passes.rs#L353-L367) you need to cast operands to concrete types manually. 
* [rspirv's instruction encode function](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/rspirv/binary/assemble.rs#L4-L16) takes an `&mut Vec<u32>` as an arg, limiting in the kinds of buffers you can write to. In rspirv2 you can write to anything that implements `WordWriter`.
* Their Module has a Vec of Instructions, and each [Instruction contains a Vec of parameters](https://github.com/gfx-rs/rspirv/blob/3e8814d838a49a98084ada2d1a8677e3281c6d33/rspirv/dr/constructs.rs#L84-L95), with no pooling or arena, leading to a lot of small individual allocations.
* rust-gpu's custom [instructions](https://github.com/Rust-GPU/rust-gpu/blob/8b85962188181817aaaf3aee431677e078ff2a68/crates/rustc_codegen_spirv/src/custom_insts.rs) and [decorations](https://github.com/Rust-GPU/rust-gpu/blob/8b85962188181817aaaf3aee431677e078ff2a68/crates/rustc_codegen_spirv/src/custom_decorations.rs) are messy and most disassemblers refuse to disassemble unknown instructions. We need tooling that supports custom instruction sets for rust-gpu internal IRs.

## Performance

Testing disassembler performance on an unreasonably large 87MiB SPIR-V file:
```text
# rspirv2
$ cargo b --bin rspirv2-dis --release
$ time ./target/release/rspirv2-dis huge.spv >/dev/null

real    0m1.363s
user    0m1.231s
sys     0m0.122s

# official C++ spirv tooling
$ time spirv-dis ./huge.spv >/dev/null

real    0m7.262s
user    0m6.873s
sys     0m0.333s

# rspirv
$ time ~/.cargo/bin/rspirv-dis ./huge.spv >/dev/null

# RSPIRV NUMBERS ARE NOT COMPARABLE
# rspirv's output is considerably simpler and cheaper to compute, using only ids (`%3`)
# instead of trying to resolve a human readable name (`%u32`).
real    0m4.679s
user    0m4.223s
sys     0m0.423s
```
