use clap::Parser;
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2::module::Module;
use rspirv2::operand::IdResult;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    /// path to SPIR-V file
    path: PathBuf,
}

impl Args {
    pub fn run(&self, f: &mut impl std::io::Write) -> anyhow::Result<()> {
        let module = Module::<CoreInstSet>::from_bytes(std::fs::read(&self.path)?.as_slice())?;

        let mut names = HashMap::<IdResult, String>::new();
        for inst in module.iter() {
            if let CoreInstSet::Name(inst) = inst {
                names.insert(inst.target.0, inst.name.0);
            }
        }

        let mut spec_const_name_to_value = HashMap::<String, _>::new();
        for inst in module.iter() {
            if let CoreInstSet::SpecConstant(inst) = inst
                && let Some(name) = names.get(&inst.id_result.unwrap())
            {
                spec_const_name_to_value.insert(name.clone(), inst.value.as_u32()?);
            }
        }

        let mut vec = spec_const_name_to_value.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, value) in vec {
            writeln!(f, "{name}: {value}")?;
        }

        Ok(())
    }
}

pub fn main() -> anyhow::Result<()> {
    Args::parse().run(&mut std::io::stdout().lock())
}

#[cfg(test)]
mod tests {
    use super::*;
    use spv::spv;
    #[test]
    pub fn test() -> anyhow::Result<()> {
        let mut stdout = Vec::new();
        Args {
            path: spv("const_specs"),
        }
        .run(&mut stdout)?;
        assert_eq!(
            String::from_utf8(stdout)?,
            "my_const: 456\nmy_second: 789\n"
        );
        Ok(())
    }
}
