use spirv_grammar_parser::parse::{CoreGrammar, ExtInstSetGrammar, GrammarFile};

macro_rules! folder_path {
    () => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/headers/include/spirv/unified1/"
        )
    };
}

pub const PATH_GRAMMAR_FOLDER: &str = folder_path!();
pub const PATH_GRAMMAR_CORE: GrammarFile<CoreGrammar> =
    GrammarFile::new_const(concat!(folder_path!(), "spirv.core.grammar.json"));
pub const PATH_GRAMMAR_GLSL_STD_450: GrammarFile<ExtInstSetGrammar> =
    GrammarFile::new_const(concat!(folder_path!(), "extinst.glsl.std.450.grammar.json"));
pub const PATH_GRAMMAR_DEBUG_PRINTF: GrammarFile<ExtInstSetGrammar> =
    GrammarFile::new_const(concat!(
        folder_path!(),
        "extinst.nonsemantic.debugprintf.grammar.json"
    ));

#[cfg(test)]
mod test {
    use super::*;
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
}
