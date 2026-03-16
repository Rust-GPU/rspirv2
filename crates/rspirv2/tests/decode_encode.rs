use rspirv2::module::Module;
use rspirv2_types::inst::InstEncoding;
use rspirv2_types::module::SPIRV_MAGIC;
use spv::spv;

#[test]
fn test_bla() -> anyhow::Result<()> {
    roundtrip_spv(&std::fs::read(spv("bla"))?)
}

#[test]
fn test_bla_be() -> anyhow::Result<()> {
    let vec = std::fs::read(spv("bla"))?;
    let vec = vec
        .as_chunks()
        .0
        .iter()
        .flat_map(|a| u32::from_ne_bytes(*a).swap_bytes().to_ne_bytes())
        .collect::<Vec<_>>();
    assert_eq!(&vec[..4], &SPIRV_MAGIC.0.swap_bytes().to_le_bytes());
    roundtrip_spv(&vec)
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
