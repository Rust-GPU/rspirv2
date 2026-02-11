use rspirv2::module::Module;

const BLA_SPV: &[u8] = include_bytes!("./spv/bla.spv");

#[test]
fn decode_spv() -> anyhow::Result<()> {
    let module = Module::from_bytes(BLA_SPV)?;
    let mut module_reader = module.reader();
    while let Some(mut inst) = module_reader.next()? {
        rspirv2::core::inst_dyn::decode_dyn(&mut inst, |inst| println!("{:?}", inst))?;
    }
    Ok(())
}
