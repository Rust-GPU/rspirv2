use rspirv2::module::Module;
use rspirv2_types::inst::InstEncoding;

#[test]
fn test_bla() -> anyhow::Result<()> {
    roundtrip_spv(&std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/spv/bla.spv"
    ))?)
}

fn roundtrip_spv(spv: &[u8]) -> anyhow::Result<()> {
    let module = Module::from_bytes(spv)?;
    let mut module_reader = module.reader();
    let mut writer = Vec::new();
    while let Some(mut inst) = module_reader.next()? {
        let inst = rspirv2::core::inst_set::CoreInstSet::decode(&mut inst)?;
        inst.encode(&mut writer)?;
    }
    assert_eq!(module.instructions(), writer.as_slice());
    Ok(())
}
