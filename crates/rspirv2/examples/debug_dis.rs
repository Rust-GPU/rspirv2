use clap::Parser;
use rspirv2::module::Module;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// path to SPIR-V file
    path: PathBuf,
}

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let binary = std::fs::read(args.path)?;
    let module = Module::from_bytes(binary.as_slice())?;
    let mut module_reader = module.reader();
    while let Some(mut inst) = module_reader.next()? {
        rspirv2::core::inst_dyn::decode_dyn(&mut inst, |inst| println!("{:?}", inst))?;
    }
    Ok(())
}
