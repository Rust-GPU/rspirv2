use rspirv2_grammar_parser::parse::{Import, Source};
use std::fs;
use std::path::PathBuf;

/// The generated files in `rspirv2`, when imported, should be equivalent to the spec.
///
/// Unfortunately, we can't `Eq::eq` them due to
/// [`SmallVec<[T; N]>` being invariant over T](https://github.com/servo/rust-smallvec/issues/146), so the imported
/// `CoreGrammar<'static>` can't be downcast to spec's `CoreGrammar<'spec_str>`. As a workaround, we're pretty debug
/// formatting the grammars and comparing that.
#[test]
#[cfg_attr(miri, ignore)]
fn test_import_roundtrip() -> anyhow::Result<()> {
    let spec_str = rspirv2_grammar::PATH_GRAMMAR_CORE.read()?;
    let spec = spec_str.parse_grammar()?;
    let spec_dbg = format!("{spec:#?}");

    let mut import = rspirv2::core::grammar::GRAMMAR_CORE.import();
    // importing sets `Source::Import`, change to `Source::Declare` to match grammar parsing
    for i in &mut import.inst_class {
        i.source = Source::Declare;
    }
    for i in &mut import.insts {
        i.source = Source::Declare;
    }
    for i in &mut import.operand_kinds {
        i.source = Source::Declare;
    }
    let import_dbg = format!("{import:#?}");

    if spec_dbg != import_dbg {
        // dump output into files instead of the console, so you can diff them properly
        let temp_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
        fs::write(temp_dir.join("spec.txt"), spec_dbg)?;
        fs::write(temp_dir.join("import.txt"), import_dbg)?;
        panic!(
            "spec and imported grammar did not match, diff files in `{}`!",
            temp_dir.display()
        )
    }
    Ok(())
}
