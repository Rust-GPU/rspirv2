use rspirv2_tools_gen::dis::Profile;
use spv::DIS_REFERENCE;

#[test]
fn test_dis_reference_default() -> anyhow::Result<()> {
    DIS_REFERENCE.test_disassembly("rspirv2", Profile::Default)
}

#[test]
fn test_dis_reference_rspirv() -> anyhow::Result<()> {
    DIS_REFERENCE.test_disassembly("rspirv_like", Profile::Rspirv)
}

#[test]
fn test_dis_reference_spirv_tools() -> anyhow::Result<()> {
    DIS_REFERENCE.test_disassembly("spirv_tools_like", Profile::SpirvTools)
}
