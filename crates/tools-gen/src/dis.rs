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
        write!(stdout, "{}", module.dis::<ISA>(DisOptions::default())?)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use rspirv2::core::inst_set::CoreInstSet;
    use spv::{spv, spv_folder};
    use std::io::stdout;

    #[test]
    fn test_bla() -> anyhow::Result<()> {
        Args { path: spv("bla") }.run::<CoreInstSet>(&mut stdout())
    }

    #[test]
    fn test_dis_reference() -> anyhow::Result<()> {
        let args = Args {
            path: spv("dis_reference"),
        };
        let mut stdout = Vec::new();
        args.run::<CoreInstSet>(&mut stdout)?;
        let stdout = String::from_utf8(stdout)?;
        let reference = std::fs::read_to_string(spv_folder().join("dis_reference.rspirv2"))?;
        assert_eq!(
            reference, stdout,
            "`dis_reference.rspirv` and `rspirv2-dis dis_reference.spv` don't match, please run \
            `cargo dis ./dis/dis_reference.spv &> ./dis/dis_reference.rspirv2`"
        );
        Ok(())
    }
}
