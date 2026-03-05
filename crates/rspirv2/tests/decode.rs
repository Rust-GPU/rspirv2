use rspirv2::module::Module;
use rspirv2_types::inst::InstEncoding;

#[test]
fn test_decode_bla() -> anyhow::Result<()> {
    decode_spv(&std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/spv/bla.spv"
    ))?)
}

fn decode_spv(spv: &[u8]) -> anyhow::Result<()> {
    let module = Module::from_bytes(spv)?;
    let mut module_reader = module.reader();
    while let Some(mut inst) = module_reader.next()? {
        rspirv2::core::inst_set::CoreInstSet::decode(&mut inst)?;
    }
    Ok(())
}
