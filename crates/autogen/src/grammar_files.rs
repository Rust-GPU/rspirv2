use spirv_grammar_parser::meta::{CoreGrammar, ExtInstSetGrammar, GrammarKind};
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::marker::PhantomData;
use std::path::Path;

#[derive(Clone)]
pub struct GrammarFile<'a, T: GrammarKind> {
    path: Cow<'a, str>,
    _phantom: PhantomData<T>,
}

impl<'a, T: GrammarKind> GrammarFile<'a, T> {
    pub const fn new(path: Cow<'a, str>) -> Self {
        Self {
            path,
            _phantom: PhantomData {},
        }
    }

    pub const fn new_const(path: &'a str) -> Self {
        Self::new(Cow::Borrowed(path))
    }

    pub fn as_path(&self) -> &Path {
        Path::new(self.path.as_ref())
    }

    pub fn read(&self) -> anyhow::Result<GrammarJson<T>> {
        Ok(GrammarJson {
            json: fs::read_to_string(self.as_path())?,
            _phantom: PhantomData {},
        })
    }
}

impl<'a, T: GrammarKind> Display for GrammarFile<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GrammarFile").field(&self.as_path()).finish()
    }
}

impl<'a, T: GrammarKind> Debug for GrammarFile<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a, T: GrammarKind> Eq for GrammarFile<'a, T> {}

impl<'a, T: GrammarKind> PartialEq for GrammarFile<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

#[derive(Clone, Debug)]
pub struct GrammarJson<T: GrammarKind> {
    json: String,
    _phantom: PhantomData<T>,
}

impl<T: GrammarKind> GrammarJson<T> {
    pub fn parse_grammar(&self) -> anyhow::Result<T::Grammar<'_>> {
        Ok(serde_json::from_str(&self.json)?)
    }
}

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
    use spirv_grammar_parser::meta::{CoreGrammar, ExtInstSetGrammar};
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
        let core: CoreGrammar = json.parse_grammar()?;
        println!("{core:?}");
        Ok(())
    }

    #[test]
    pub fn parse_all_extinst_grammars() -> anyhow::Result<()> {
        let mut extinst: Vec<GrammarFile<ExtInstSetGrammar>> = fs::read_dir(PATH_GRAMMAR_FOLDER)
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
            let grammar: ExtInstSetGrammar = data
                .parse_grammar()
                .map_err(|e| anyhow::anyhow!("{name}: {e}"))?;
            println!("{}: {} instructions", name, grammar.instructions.len());
        }
        Ok(())
    }
}
