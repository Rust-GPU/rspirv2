#![cfg(feature = "serde")]

use crate::meta::{Category, InstructionPrintingClass, OperandKind};
use serde::de;
use std::borrow::Cow;
use std::fmt;

pub fn num_or_hex<'de, D: de::Deserializer<'de>>(d: D) -> Result<u32, D::Error> {
    struct NumOrStr;

    impl de::Visitor<'_> for NumOrStr {
        type Value = u32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "either a number or a hex string")
        }

        fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
            value
                .try_into()
                .map_err(|_e| de::Error::invalid_value(de::Unexpected::Unsigned(value), &self))
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            u32::from_str_radix(&value[2..], 16)
                .map_err(|_e| de::Error::invalid_value(de::Unexpected::Str(value), &self))
        }
    }

    d.deserialize_any(NumOrStr)
}

/// Deserializes a JSON string like `"Miscellaneous"` into an `InstructionPrintingClass` with that
/// tag and no heading.
pub fn printing_class_from_str<'a, 'de: 'a, D: de::Deserializer<'de>>(
    d: D,
) -> Result<Option<Cow<'a, InstructionPrintingClass<'a>>>, D::Error> {
    let tag: Option<Cow<'de, str>> = de::Deserialize::deserialize(d)?;
    Ok(tag.map(|tag| Cow::Owned(InstructionPrintingClass { tag, heading: None })))
}

/// Deserializes a JSON string like `"IdRef"` into an `OperandKind` with that name, `Id` category,
/// and no doc.
pub fn operand_kind_from_str<'a, 'de: 'a, D: de::Deserializer<'de>>(
    d: D,
) -> Result<Cow<'a, OperandKind<'a>>, D::Error> {
    let name: Cow<'de, str> = de::Deserialize::deserialize(d)?;
    Ok(Cow::Owned(OperandKind {
        name,
        category: Category::Id,
        doc: Cow::Borrowed(""),
    }))
}

/// Deserializes a JSON array of strings like `["LiteralInteger", "IdRef"]` into a slice of
/// `OperandKind`s, each with `Id` category and no doc.
pub fn operand_kinds_from_strs<'a, 'de: 'a, D: de::Deserializer<'de>>(
    d: D,
) -> Result<Cow<'a, [OperandKind<'a>]>, D::Error> {
    let names: Vec<Cow<'de, str>> = de::Deserialize::deserialize(d)?;
    Ok(Cow::Owned(
        names
            .into_iter()
            .map(|name| OperandKind {
                name,
                category: Category::Id,
                doc: Cow::Borrowed(""),
            })
            .collect(),
    ))
}
