use rspirv2_tools_gen::dis::Profile;
use spv::TEXTURE_GRAD_OFFSET;

#[test]
fn test_texture_grad_offset_default() -> anyhow::Result<()> {
    TEXTURE_GRAD_OFFSET.test_disassembly("rspirv2", Profile::Default)
}

#[test]
fn test_texture_grad_offset_rspirv() -> anyhow::Result<()> {
    TEXTURE_GRAD_OFFSET.test_disassembly("rspirv_like", Profile::Rspirv)
}

#[test]
fn test_texture_grad_offset_spirv_tools() -> anyhow::Result<()> {
    TEXTURE_GRAD_OFFSET.test_disassembly("spirv_tools_like", Profile::SpirvTools)
}
