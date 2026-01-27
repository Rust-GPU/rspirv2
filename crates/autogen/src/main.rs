#![doc = include_str!("../README.md")]

use spirv_grammar_parser::meta::CoreGrammar;
use std::fs;
use std::path::PathBuf;

pub struct SpirvGrammarFiles {
    pub folder: PathBuf,
    pub core: PathBuf,
}

impl SpirvGrammarFiles {
    pub fn new() -> Self {
        let folder = PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/headers/include/spirv/unified1"
        ));
        let core = folder.join("spirv.core.grammar.json");
        Self { folder, core }
    }
}

pub fn main() -> anyhow::Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn parse_core_grammar() -> anyhow::Result<()> {
        let files = SpirvGrammarFiles::new();
        let core = fs::read(&files.core)?;
        let core_grammar: CoreGrammar = serde_json::from_slice(&core)?;
        println!("{core_grammar:?}");
        Ok(())
    }
}
