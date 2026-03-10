use clap::Parser;
use rspirv2::dis::DisOptions;
use rspirv2::inst::InstEncoding;
use rspirv2::module::Module;
use std::io::Write;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// path to SPIR-V file
    path: PathBuf,
}

impl Args {
    pub fn run<ISA: InstEncoding>(&self, stdout: &mut impl Write) -> anyhow::Result<()> {
        let module = Module::from_bytes(std::fs::read(&self.path)?.as_slice())?;
        writeln!(stdout, "{}", module.dis::<ISA>(DisOptions::default())?)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use rspirv2::core::inst_set::CoreInstSet;
    use spv::spv;
    use std::io::stdout;

    #[test]
    fn test() -> anyhow::Result<()> {
        Args { path: spv("bla") }.run::<CoreInstSet>(&mut stdout())
    }
}
