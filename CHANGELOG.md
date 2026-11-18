# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- [PR#55](https://github.com/Rust-GPU/rspirv2/pull/55) make disassembly failures colorful and exit with `-1`
- [PR#51](https://github.com/Rust-GPU/rspirv2/pull/51) impl Eq and Hash on `InstSlice` and `InstVec`
- [PR#50](https://github.com/Rust-GPU/rspirv2/pull/50) support `OpSpecConstantOp`
- [PR#48](https://github.com/Rust-GPU/rspirv2/pull/48) allow `OpNop {}` to be written as `OpNop`

### Changed

- [PR#62](https://github.com/Rust-GPU/rspirv2/pull/62) `Word` is now an alias for `u32`, makes many `Word(id)` and `word.0` unnecessary
- [PR#53](https://github.com/Rust-GPU/rspirv2/pull/53) rename `rspirv2::custom_inst` to `op_switch`
- [PR#52](https://github.com/Rust-GPU/rspirv2/pull/52) change `InstEncoding::id_result_type()` return type from `IdResult` to `IdResultType`
- [PR#60](https://github.com/Rust-GPU/rspirv2/pull/60) `LiteralExtInstInteger` and `LiteralSpecConstantOpInteger` are now just aliases of `LiteralInteger`
- [PR#61](https://github.com/Rust-GPU/rspirv2/pull/61) `InstEncoding::encode` cannot error anymore

### Fixed

- [PR#57](https://github.com/Rust-GPU/rspirv2/pull/57) fix `SpirvVersion` encoding
- [PR#58](https://github.com/Rust-GPU/rspirv2/pull/58) IdResult 0 is reserved, start allocation at 1

## [0.1.1+sdk-1.4.363](https://github.com/Rust-GPU/rspirv2/compare/v0.1.0+sdk-1.4.357...v0.1.1+sdk-1.4.363) - 2026-09-22

- Update Vulkan sdk to [vulkan-sdk-1.4.363](https://github.com/KhronosGroup/Vulkan-Headers/tree/vulkan-sdk-1.4.363)

## [0.1.0+sdk-1.4.357](https://github.com/Rust-GPU/rspirv2/releases/tag/v0.1.0+sdk-1.4.357) - 2026-07-29

- Initial release of rspirv2
