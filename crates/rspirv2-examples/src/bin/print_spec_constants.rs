use clap::Parser;
use rspirv2::binary::DecodeError;
use rspirv2::core::inst::{OpName, OpSpecConstant};
use rspirv2::inst::InstEncoding;
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
        let module = Module::from_bytes(std::fs::read(&self.path)?.as_slice())?;

        let mut names = HashMap::<IdResult, String>::new();
        let mut module_reader = module.reader();
        while let Some(mut inst) = module_reader.next()? {
            match OpName::decode(&mut inst) {
                Ok(inst) => {
                    names.insert(inst.target.0, inst.name.0);
                }
                Err(DecodeError::WrongOpCode { .. }) => (),
                Err(err) => return Err(anyhow::Error::from(err)),
            };
        }

        let mut spec_const_name_to_value = HashMap::<String, _>::new();
        let mut module_reader = module.reader();
        while let Some(mut inst) = module_reader.next()? {
            match OpSpecConstant::decode(&mut inst) {
                Ok(inst) => {
                    if let Some(name) = names.get(&inst.id_result.unwrap()) {
                        spec_const_name_to_value.insert(name.clone(), inst.value.as_u32()?);
                    }
                }
                Err(DecodeError::WrongOpCode { .. }) => {}
                Err(err) => return Err(anyhow::Error::from(err)),
            };
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
