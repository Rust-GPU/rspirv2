use crate::binary::{DecodeError, EncodeError, OperandReader, WordWriter};
use crate::dis::DisContext;
use crate::meta::{Category, OperandKind};
use crate::operand::{Operand, OperandEncoding, Word};
use anstyle::AnsiColor;
use std::fmt::Formatter;

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
        let words = self.word_len();
        let load = |i, o| *self.0.as_bytes().get(i * 4 + o).unwrap_or(&0);
        writer.write_iter(
            (0..words)
                .map(|i| Word::from_le_bytes([load(i, 0), load(i, 1), load(i, 2), load(i, 3)])),
        );
        Ok(())
    }

    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bytes = reader
            .flat_map(|w| w.to_le_bytes().into_iter())
            .take_while(|p| *p != 0)
            .collect::<Vec<_>>();
        Ok(Self(String::from_utf8(bytes)?))
    }

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        let color = ctx.color(AnsiColor::Green.on_default());
        write!(f, "{color}\"{}\"{color:#}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::InstReader;

    fn roundtrip(str: &str, expected_spirv: &[[u8; 4]]) -> anyhow::Result<()> {
        let mut spirv = Vec::<Word>::default();
        LiteralString(str.to_string()).encode(&mut spirv)?;
        let read =
            LiteralString::decode(&mut InstReader::new(0, spirv.as_slice(), 0).operand_reader())?;
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
}
