// Simple single entrypoint function test.
// build-pass

use spirv_std::spirv;

#[spirv(fragment)]
pub fn main(
    #[spirv(spec_constant(id = 123, default = 456))] my_const: u32,
    #[spirv(spec_constant(id = 123, default = 789))] my_second: u32,
) {

}
