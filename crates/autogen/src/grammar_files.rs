use std::path::PathBuf;

pub struct SpirvGrammarFiles {
    pub folder: PathBuf,
    pub core: PathBuf,
    pub glsl_std_450: PathBuf,
    pub non_semantic_debug_printf: PathBuf,
}

impl SpirvGrammarFiles {
    pub fn new() -> Self {
        let folder = PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/headers/include/spirv/unified1"
        ));
        let core = folder.join("spirv.core.grammar.json");
        let glsl_std_450 = folder.join("extinst.glsl.std.450.grammar.json");
        let non_semantic_debug_printf = folder.join("extinst.nonsemantic.debugprintf.grammar.json");
        Self {
            folder,
            core,
            glsl_std_450,
            non_semantic_debug_printf,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use spirv_grammar_parser::meta::{CoreGrammar, ExtInstSetGrammar};
    use std::fs;

    #[test]
    pub fn parse_core_grammar() -> anyhow::Result<()> {
        let files = SpirvGrammarFiles::new();
        let core = fs::read(&files.core)?;
        let core_grammar: CoreGrammar = serde_json::from_slice(&core)?;
        println!("{core_grammar:?}");
        Ok(())
    }

    #[test]
    pub fn parse_all_extinst_grammars() -> anyhow::Result<()> {
        let files = SpirvGrammarFiles::new();
        let mut extinst: Vec<PathBuf> = fs::read_dir(&files.folder)
            .expect("failed to read SPIR-V headers directory")
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                let name = path.file_name()?.to_str()?;
                (name.starts_with("extinst.") && name.ends_with(".json")).then(|| path)
            })
            .collect();
        extinst.sort();
        assert!(extinst.contains(&files.glsl_std_450));
        assert!(extinst.contains(&files.non_semantic_debug_printf));

        for path in &extinst {
            let name = path.file_name().unwrap().to_str().unwrap();
            let data = fs::read(path)?;
            let grammar: ExtInstSetGrammar =
                serde_json::from_slice(&data).map_err(|e| anyhow::anyhow!("{name}: {e}"))?;
            println!("{}: {} instructions", name, grammar.instructions.len());
        }
        Ok(())
    }
}
