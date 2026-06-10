use expect_test::ExpectFile;
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2_tools_gen::dis::{Args, Profile};
use spv::TEXTURE_GRAD_OFFSET;
use std::path::PathBuf;

#[test]
fn test_texture_grad_offset_default_be() -> anyhow::Result<()> {
    test_disassembly(
        TEXTURE_GRAD_OFFSET.spv(),
        TEXTURE_GRAD_OFFSET.expect("rspirv2"),
        Profile::Default,
        true,
    )
}

#[test]
fn test_texture_grad_offset_rspirv() -> anyhow::Result<()> {
    test_disassembly(
        TEXTURE_GRAD_OFFSET.spv(),
        TEXTURE_GRAD_OFFSET.expect("rspirv_like"),
        Profile::Rspirv,
        false,
    )
}

#[test]
fn test_texture_grad_offset_rspirv_be() -> anyhow::Result<()> {
    test_disassembly(
        TEXTURE_GRAD_OFFSET.spv(),
        TEXTURE_GRAD_OFFSET.expect("rspirv_like"),
        Profile::Rspirv,
        true,
    )
}

#[test]
fn test_texture_grad_offset_spirv_tools() -> anyhow::Result<()> {
    test_disassembly(
        TEXTURE_GRAD_OFFSET.spv(),
        TEXTURE_GRAD_OFFSET.expect("spirv_tools_like"),
        Profile::SpirvTools,
        false,
    )
}

#[test]
fn test_texture_grad_offset_spirv_tools_be() -> anyhow::Result<()> {
    test_disassembly(
        TEXTURE_GRAD_OFFSET.spv(),
        TEXTURE_GRAD_OFFSET.expect("spirv_tools_like"),
        Profile::SpirvTools,
        false,
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
