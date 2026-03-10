use clap::Parser;
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2::inst::InstEncoding;
use rspirv2::module::Module;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// path to SPIR-V file
    path: PathBuf,
}

impl Args {
    pub fn run(&self) -> anyhow::Result<()> {
        let binary = std::fs::read(&self.path)?;
        let module = Module::from_bytes(binary.as_slice())?;
        let mut module_reader = module.reader();
        while let Some(mut inst) = module_reader.next()? {
            let inst = CoreInstSet::decode(&mut inst)?;
            println!("{:?}", inst);
        }
        Ok(())
    }
}

pub fn main() -> anyhow::Result<()> {
    Args::parse().run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spv::spv;

    #[test]
    pub fn test() -> anyhow::Result<()> {
        Args { path: spv("bla") }.run()?;
        Args {
            path: spv("const_specs"),
        }
        .run()?;
        Ok(())
    }
}
