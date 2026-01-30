use crate::parse::serde_helper::num_or_hex;
use crate::parse::{Capability, Extension, OperandMeta};
use smallvec::SmallVec;
use std::borrow::Cow;

/// See [`spirv_grammar::meta::OperandKind`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct OperandKind<'a> {
    #[serde(borrow, rename = "kind")]
    pub name: Cow<'a, str>,
    #[serde(flatten)]
    pub category: Category<'a>,
    #[serde(borrow, default)]
    pub doc: Cow<'a, str>,
}

/// See [`spirv_grammar::meta::Category`]
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(tag = "category")]
pub enum Category<'a> {
    BitEnum {
        #[serde(borrow)]
        enumerants: Vec<Enumerant<'a>>,
    },
    Composite {
        /// The name of the [`OperandKind`]s out of which this [`OperandKind`] is composed out of,
        /// references `Grammar.operand_kinds`
        #[serde(borrow)]
        bases: Vec<Cow<'a, str>>,
    },
    Id,
    Literal,
    ValueEnum {
        #[serde(borrow)]
        enumerants: Vec<Enumerant<'a>>,
    },
}

/// See [`spirv_grammar::meta::Enumerant`]
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Enumerant<'a> {
    #[serde(borrow, rename = "enumerant")]
    pub symbol: Cow<'a, str>,
    #[serde(deserialize_with = "num_or_hex")]
    pub value: u32,
    #[serde(borrow, default)]
    pub parameters: SmallVec<[OperandMeta<'a>; 1]>,
    #[serde(borrow, default)]
    pub capabilities: SmallVec<[Capability<'a>; 2]>,
    #[serde(borrow, default)]
    pub extensions: SmallVec<[Extension<'a>; 2]>,
    #[serde(borrow, default)]
    pub version: Option<Cow<'a, str>>,
    #[serde(borrow, default, rename = "lastVersion")]
    pub last_version: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    pub aliases: SmallVec<[Cow<'a, str>; 1]>,
    #[serde(default)]
    pub provisional: bool,
}
