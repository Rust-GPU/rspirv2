use rspirv2::core::inst_set::CoreInstSet;
use rspirv2::module::Module;
use rspirv2_tools_gen::dis::Profile;
use spv::SPEC_CONSTANT_OP;

#[test]
fn test_spec_constant_op_dis() -> anyhow::Result<()> {
    SPEC_CONSTANT_OP.test_disassembly("rspirv2", Profile::Default)
}

#[test]
fn test_spec_constant_op_checked() -> anyhow::Result<()> {
    Module::<CoreInstSet>::from_bytes(&std::fs::read(SPEC_CONSTANT_OP.spv())?)?;
    Ok(())
}
