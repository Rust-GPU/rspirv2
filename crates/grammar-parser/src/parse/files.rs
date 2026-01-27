use crate::parse::GrammarKind;
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
