use clap::Parser;
use rspirv2::core::inst_set::CoreInstSet;
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
        let module = Module::<CoreInstSet>::from_bytes(binary.as_slice())?;
        for inst in module.iter() {
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
        Args {
            path: spv("bla").spv(),
        }
        .run()?;
        Args {
            path: spv("const_specs").spv(),
        }
        .run()?;
        Ok(())
    }
}
