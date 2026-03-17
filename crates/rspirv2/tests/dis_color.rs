use rspirv2::core::inst_set::CoreInstSet;
use rspirv2_types::dis::DisOptions;
use rspirv2_types::module::Module;
use spv::spv;
use std::io::Write;

#[test]
pub fn test_disabled_color() -> anyhow::Result<()> {
    let module =
        Module::<CoreInstSet>::from_bytes(std::fs::read(spv("dis_reference"))?.as_slice())?;

    let mut color_stripped = Vec::new();
    write!(
        anstream::AutoStream::never(&mut color_stripped),
        "{}",
        module.dis(DisOptions {
            color: true,
            ..Default::default()
        })
    )?;

    let mut color_disabled = Vec::new();
    write!(
        &mut color_disabled,
        "{}",
        module.dis(DisOptions {
            color: false,
            ..Default::default()
        })
    )?;

    assert_eq!(
        color_stripped, color_disabled,
        "Disabling color doesn't remove all ansi escape sequences"
    );
    Ok(())
}
