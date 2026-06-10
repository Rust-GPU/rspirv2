use expect_test::ExpectFile;
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2_tools_gen::dis::{Args, Profile};
use spv::DIS_REFERENCE;
use std::path::PathBuf;

#[test]
fn test_dis_reference_default() -> anyhow::Result<()> {
    test_disassembly(
        DIS_REFERENCE.spv(),
        DIS_REFERENCE.expect("rspirv2"),
        Profile::Default,
        false,
    )
}

#[test]
fn test_dis_reference_default_be() -> anyhow::Result<()> {
    test_disassembly(
        DIS_REFERENCE.spv(),
        DIS_REFERENCE.expect("rspirv2"),
        Profile::Default,
        true,
    )
}

#[test]
fn test_dis_reference_rspirv() -> anyhow::Result<()> {
    test_disassembly(
        DIS_REFERENCE.spv(),
        DIS_REFERENCE.expect("rspirv_like"),
        Profile::Rspirv,
        false,
    )
}

#[test]
fn test_dis_reference_rspirv_be() -> anyhow::Result<()> {
    test_disassembly(
        DIS_REFERENCE.spv(),
        DIS_REFERENCE.expect("rspirv_like"),
        Profile::Rspirv,
        true,
    )
}

#[test]
fn test_dis_reference_spirv_tools() -> anyhow::Result<()> {
    test_disassembly(
        DIS_REFERENCE.spv(),
        DIS_REFERENCE.expect("spirv_tools_like"),
        Profile::SpirvTools,
        false,
    )
}

#[test]
fn test_dis_reference_spirv_tools_be() -> anyhow::Result<()> {
    test_disassembly(
        DIS_REFERENCE.spv(),
        DIS_REFERENCE.expect("spirv_tools_like"),
        Profile::SpirvTools,
        true,
    )
}

fn test_disassembly(
    path: PathBuf,
    expect: ExpectFile,
    profile: Profile,
    module_swap_bytes: bool,
) -> anyhow::Result<()> {
    let args = Args {
        path,
        profile,
        module_swap_bytes,
        color: clap::ColorChoice::Never,
    };
    let mut stdout = Vec::new();
    args.run_inner::<CoreInstSet>(&mut stdout)?;
    let stdout = String::from_utf8(stdout)?;
    expect.assert_eq(&stdout);
    Ok(())
}
