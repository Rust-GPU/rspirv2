# spirv-grammar

Provides the SPIR-V grammar to Rust, using `spirv-grammar-parser` to parse it. You'll find prepared paths to the SPIR-V grammar JSON files in `lib.rs`. 

The crate assumes it is exclusively used for codegen in build scripts (or other binaries within a dev workspace), and never distributed within an actual binary. Thus, it's just providing file paths to the grammar files, not actually `include_bytes!` them. The actual grammar files are located in `./headers` as a git submodule of the [SPIR-V Headers repo](https://github.com/KhronosGroup/SPIRV-Headers/).
