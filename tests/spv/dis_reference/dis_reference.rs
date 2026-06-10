// compile-flags: -C target-feature=+StorageImageWriteWithoutFormat
// compile-flags: -C llvm-args=--disassemble
// normalize-stderr-test "OpSource .*\n" -> ""
// normalize-stderr-test "OpLine .*\n" -> ""
// normalize-stderr-test "%\d+ = OpString .*\n" -> ""
// normalize-stderr-test "; .*\n" -> ""
// normalize-stderr-test "OpCapability VulkanMemoryModel\n" -> ""
// normalize-stderr-test "OpMemoryModel Logical Vulkan" -> "OpMemoryModel Logical Simple"
// ignore-vulkan1.0
// ignore-vulkan1.1
// ignore-spv1.0
// ignore-spv1.1
// ignore-spv1.2
// ignore-spv1.3

use spirv_std::glam::*;
use spirv_std::spirv;
use spirv_std::Integer;
use spirv_std::memory::*;
use core::arch::asm;

#[spirv(compute(threads(32)))]
pub fn main(
    #[spirv(local_invocation_id)] tid: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 42)] offset_buffer: &[u32],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 69)] output_buffer: &mut u32,
    output: &mut f32,
) {
    unsafe {
        let offset = compute_offset(tid.x);
        let buffer_load = offset_buffer[offset];
        let value = (buffer_load as f32 * 1.7) as u32;
        atomic_i_add::<
            _,
            { Scope::QueueFamily as u32 },
            { Semantics::CROSS_WORKGROUP_MEMORY.union(Semantics::ACQUIRE_RELEASE).union(Semantics::MAKE_AVAILABLE).union(Semantics::MAKE_VISIBLE).bits() },
        >(output_buffer, value);
    }
}

/// I hacked rustc_codegen_spirv to emit this function as `DontInline|Const` instead of just `DontInline` to get
/// spirv-dis to show how it displays bitmasks: `A|B`
#[inline(never)]
pub const fn compute_offset(input: u32) -> usize {
    1 + input as usize
}

/// local function to not bloat the spv with spirv_std::arch::atomic src code
#[inline]
pub unsafe fn atomic_i_add<I: Integer, const SCOPE: u32, const SEMANTICS: u32>(
    ptr: &mut I,
    value: I,
) -> I {
    unsafe {
        let mut old = I::default();
        asm! {
            "%u32 = OpTypeInt 32 0",
            "%scope = OpConstant %u32 {scope}",
            "%semantics = OpConstant %u32 {semantics}",
            "%value = OpLoad _ {value}",
            "%old = OpAtomicIAdd _ {ptr} %scope %semantics %value",
            "OpStore {old} %old",
            scope = const SCOPE,
            semantics = const SEMANTICS,
            ptr = in(reg) ptr,
            old = in(reg) &mut old,
            value = in(reg) &value
        }
        old
    }
}
