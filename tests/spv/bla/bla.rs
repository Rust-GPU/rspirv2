// build-pass
// compile-flags: -C target-feature=+StorageImageWriteWithoutFormat
// compile-flags: -C llvm-args=--disassemble
// normalize-stderr-test \"OpSource .*\\n\" -> \"\"
// normalize-stderr-test \"OpLine .*\\n\" -> \"\"
// normalize-stderr-test \"%\\d+ = OpString .*\\n\" -> \"\"
// normalize-stderr-test \"; .*\\n\" -> \"\"
// normalize-stderr-test \"OpCapability VulkanMemoryModel\\n\" -> \"\"
// normalize-stderr-test \"OpMemoryModel Logical Vulkan\" -> \"OpMemoryModel Logical Simple\"
// ignore-vulkan1.0
// ignore-vulkan1.1
// ignore-spv1.0
// ignore-spv1.1
// ignore-spv1.2
// ignore-spv1.3

use spirv_std::glam::*;
use spirv_std::spirv;
use spirv_std::{Image, arch};

#[spirv(fragment)]
pub fn main(var_out: &mut f32) {
    let add = 1u32 + 1u32;
    let u_to_f = add as f32;
    *var_out = u_to_f;
}
