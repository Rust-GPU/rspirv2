use crate::Word;
use crate::binary::{DecodeError, EncodeError, OperandReader, WordWriter};
use crate::dis::DisContext;
use crate::meta::{Category, OperandKind};
use crate::operand::{Operand, OperandEncoding};
use anstyle::AnsiColor;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

pub const OPERAND_KIND_LITERAL_STRING: OperandKind = OperandKind {
    name: "LiteralString",
    category: Category::Literal,
    doc: "A null-terminated stream of characters consuming an integral number of words",
};

/// A SPIR-V String literal. Defined as a sequence of UTF-8, so we can just use an ordinary [`String`].
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LiteralString(pub String);

impl LiteralString {
    #[inline]
    pub fn new(str: String) -> Self {
        Self(str)
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.0
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

unsafe impl Operand for LiteralString {
    const KIND: &OperandKind = &OPERAND_KIND_LITERAL_STRING;
}

unsafe impl OperandEncoding for LiteralString {
    const FIXED_LEN: Option<usize> = None;

    #[inline]
    fn word_len(&self) -> usize {
        (self.0.len() + 1).div_ceil(4)
    }

    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let (chunks, remainder) = self.0.as_bytes().as_chunks::<4>();
        let mut last = [0; 4];
        last[..remainder.len()].copy_from_slice(remainder);
        writer.write_iter(chunks.iter().copied().map(Word::from_le_bytes));
        writer.write(Word::from_le_bytes(last));
        Ok(())
    }

    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let mut found_null_terminator = false;
        let bytes = reader
            .flat_map(|w| w.to_le_bytes().into_iter())
            .take_while(|c| {
                found_null_terminator |= *c == 0;
                !found_null_terminator
            })
            .collect::<Vec<_>>();
        if found_null_terminator {
            Ok(Self(String::from_utf8(bytes)?))
        } else {
            Err(DecodeError::StringNotNulTerminated)
        }
    }

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        let color = ctx.color(AnsiColor::Green.on_default());
        let str = ctx.literal_string_escape.escape(&self.0);
        write!(f, " {color}\"{str}\"{color:#}")
    }
}

/// Describes how to escape string sequences
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum LiteralStringEscape {
    /// Don't escape anything
    Noop,
    /// Replace newlines with `\n` to keep strings on a single line
    EscapeNewlines,
    /// Keep newlines intact, allowing strings to span multiple lines
    #[default]
    MultiLine,
}

impl LiteralStringEscape {
    pub fn escape<'a>(&self, str: &'a str) -> Cow<'a, str> {
        match self {
            LiteralStringEscape::Noop => str.into(),
            LiteralStringEscape::EscapeNewlines => str
                .replace("\\", "\\\\")
                .replace("\"", "\\\"")
                .replace("\n", "\\n")
                .into(),
            LiteralStringEscape::MultiLine => {
                str.replace("\\", "\\\\").replace("\"", "\\\"").into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(str: &str, expected_spirv: &[[u8; 4]]) -> anyhow::Result<()> {
        let mut spirv = Vec::<Word>::default();
        LiteralString(str.to_string()).encode(&mut spirv)?;
        let read = LiteralString::decode(&mut OperandReader::new(spirv.as_slice()))?;
        assert_eq!(str, read.as_str());
        assert_eq!(
            expected_spirv
                .iter()
                .map(|w| Word::from_le_bytes(*w))
                .collect::<Vec<_>>(),
            spirv
        );
        Ok(())
    }

    #[test]
    fn test_empty_str() -> anyhow::Result<()> {
        roundtrip("", &[[0; 4]])
    }

    #[test]
    fn test_str() -> anyhow::Result<()> {
        roundtrip("abc", &[[b'a', b'b', b'c', 0]])?;
        roundtrip("123", &[[b'1', b'2', b'3', 0]])?;
        roundtrip("abcd", &[[b'a', b'b', b'c', b'd'], [0, 0, 0, 0]])?;
        roundtrip(
            "abcdefg",
            &[[b'a', b'b', b'c', b'd'], [b'e', b'f', b'g', 0]],
        )?;
        roundtrip(
            "abcdefgh",
            &[
                [b'a', b'b', b'c', b'd'],
                [b'e', b'f', b'g', b'h'],
                [0, 0, 0, 0],
            ],
        )?;
        Ok(())
    }

    #[test]
    pub fn reject_bad_str() {
        let test = |bytes: &[[u8; 4]], str: Option<&str>| {
            let words = bytes
                .iter()
                .copied()
                .map(Word::from_le_bytes)
                .collect::<Vec<_>>();
            let read = LiteralString::decode(&mut OperandReader::new(words.as_slice())).ok();
            assert_eq!(read.as_ref().map(|s| s.as_str()), str);
        };

        test(&[[b'a', 0, 0, 0]], Some("a"));
        test(&[[b'a', b'b', 0, 0]], Some("ab"));
        test(&[[b'a', b'b', b'c', 0]], Some("abc"));
        test(&[[b'a', b'b', b'c', b'd'], [0, 0, 0, 0]], Some("abcd"));
        test(&[[b'a', b'b', b'c', b'd'], [b'e', 0, 0, 0]], Some("abcde"));
        test(
            &[[b'a', b'b', b'c', b'd'], [b'e', b'f', b'g', 0]],
            Some("abcdefg"),
        );
        test(
            &[[b'a', b'b', b'c', b'd'], [b'e', b'f', b'g', b'h'], [0; 4]],
            Some("abcdefgh"),
        );

        // missing null terminator
        test(&[], None);
        test(&[[b'a', b'b', b'c', b'd']], None);
        test(&[[b'a', b'b', b'c', b'd'], [b'e', b'f', b'g', b'h']], None);

        test(&[[0, 0, 0, 0]], Some(""));
    }
}
