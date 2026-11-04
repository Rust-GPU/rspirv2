use rspirv2_tools_gen::dis::Profile;
use spv::BLA;

#[test]
fn test_bla() -> anyhow::Result<()> {
    BLA.test_disassembly("rspirv2", Profile::Default)
}
