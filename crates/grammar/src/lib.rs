use rspirv2_grammar_parser::parse::{CoreGrammar, ExtInstSetGrammar, GrammarFile};

macro_rules! folder_path {
    () => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/headers/include/spirv/unified1/"
        )
    };
}

#[allow(unused)]
pub const PATH_GRAMMAR_FOLDER: &str = folder_path!();
pub const PATH_GRAMMAR_CORE: GrammarFile<'static, CoreGrammar<'static>> =
    GrammarFile::new_const(concat!(folder_path!(), "spirv.core.grammar.json"));
pub const PATH_GRAMMAR_GLSL_STD_450: GrammarFile<'static, ExtInstSetGrammar<'static>> =
    GrammarFile::new_const(concat!(folder_path!(), "extinst.glsl.std.450.grammar.json"));
pub const PATH_GRAMMAR_DEBUG_PRINTF: GrammarFile<'static, ExtInstSetGrammar<'static>> =
    GrammarFile::new_const(concat!(
        folder_path!(),
        "extinst.nonsemantic.debugprintf.grammar.json"
    ));

#[cfg(test)]
mod test {
    use super::*;
    use std::borrow::Cow;
    use std::fs;
    use std::path::Path;

    #[test]
    pub fn sanity_grammar_files_exist() {
        assert!(
            Path::new(PATH_GRAMMAR_FOLDER).is_dir(),
            "expected directory: {}",
            PATH_GRAMMAR_FOLDER
        );
        assert!(
            PATH_GRAMMAR_CORE.as_path().is_file(),
            "missing grammar core file: {}",
            PATH_GRAMMAR_CORE
        );
        assert!(
            PATH_GRAMMAR_GLSL_STD_450.as_path().is_file(),
            "missing extinst glsl std 450 file: {}",
            PATH_GRAMMAR_GLSL_STD_450
        );
        assert!(
            PATH_GRAMMAR_DEBUG_PRINTF.as_path().is_file(),
            "missing debug printf file: {}",
            PATH_GRAMMAR_DEBUG_PRINTF
        );
    }

    #[test]
    pub fn parse_core_grammar() -> anyhow::Result<()> {
        let json = PATH_GRAMMAR_CORE.read()?;
        let core: CoreGrammar<'_> = json.parse_grammar()?;
        println!("core has {} Instructions", core.insts.len());
        Ok(())
    }

    #[test]
    pub fn parse_all_extinst_grammars() -> anyhow::Result<()> {
        let mut extinst: Vec<GrammarFile<'_, ExtInstSetGrammar<'_>>> =
            fs::read_dir(PATH_GRAMMAR_FOLDER)
                .expect("failed to read SPIR-V headers directory")
                .filter_map(|entry| {
                    let path = entry.ok()?.path();
                    let name = path.file_name()?.to_str()?;
                    (name.starts_with("extinst.") && name.ends_with(".json")).then(|| {
                        let path_str = path.into_os_string().into_string().ok()?;
                        Some(GrammarFile::new(Cow::Owned(path_str)))
                    })?
                })
                .collect();
        extinst.sort_by(|a, b| a.as_path().cmp(b.as_path()));

        // equality on paths may fail on some platforms?
        assert!(extinst.contains(&PATH_GRAMMAR_GLSL_STD_450));
        assert!(extinst.contains(&PATH_GRAMMAR_DEBUG_PRINTF));

        for path in &extinst {
            let name = path.as_path().file_name().unwrap().to_str().unwrap();
            let data = path.read()?;
            let grammar: ExtInstSetGrammar<'_> = data
                .parse_grammar()
                .map_err(|e| anyhow::anyhow!("{name}: {e}"))?;
            println!("{}: {} instructions", name, grammar.insts.len());
        }
        Ok(())
    }
}
