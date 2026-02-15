use crate::binary::{DecodeError, EncodeError, InstructionWriter, OperandReader};
use crate::meta::OperandKind;
use crate::operand::{Operand, OperandEncoding, Word};
use std::ops::{Deref, DerefMut};

/// A SPIR-V String literal. Defined as a sequence of UTF-8, so we can just use an ordinary [`String`].
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LiteralString(pub String);

impl LiteralString {
    pub fn new(str: String) -> Self {
        Self(str)
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for LiteralString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LiteralString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl Operand for LiteralString {
    const KIND: &OperandKind = &crate::core::operand_kinds::OPERAND_KIND_LITERAL_STRING;
}

unsafe impl OperandEncoding for LiteralString {
    const FIXED_LEN: Option<usize> = None;

    #[inline]
    fn word_len(&self) -> usize {
        (self.0.len() + 1).div_ceil(4)
    }

    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
        let words = self.word_len();
        let load = |i, o| *self.as_bytes().get(i * 4 + o).unwrap_or(&0);
        writer.write_iter((0..words).map(|i| {
            Word(u32::from_ne_bytes([
                load(i, 0),
                load(i, 1),
                load(i, 2),
                load(i, 3),
            ]))
        }));
        Ok(())
    }

    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bytes = reader
            .flat_map(|w| u32::to_ne_bytes(w.0).into_iter())
            .take_while(|p| *p != 0)
            .collect::<Vec<_>>();
        Ok(Self(String::from_utf8(bytes)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::InstReader;

    fn roundtrip(str: &str, expected_spirv: &[u32]) -> anyhow::Result<()> {
        let mut spirv = Vec::<Word>::new();
        LiteralString(str.to_string()).encode(&mut spirv)?;
        let read =
            LiteralString::decode(&mut InstReader::new(0, spirv.as_slice(), 0).operand_reader())?;
        assert_eq!(str, read.as_str());
        assert_eq!(
            spirv.into_iter().map(|w| w.0).collect::<Vec<_>>(),
            expected_spirv
        );
        Ok(())
    }

    #[test]
    fn test_empty_str() -> anyhow::Result<()> {
        roundtrip("", &[0])
    }

    #[test]
    fn test_str() -> anyhow::Result<()> {
        roundtrip("abc", &[u32::from_ne_bytes([b'a', b'b', b'c', 0])])?;
        roundtrip("123", &[u32::from_ne_bytes([b'1', b'2', b'3', 0])])?;
        roundtrip("abcd", &[u32::from_ne_bytes([b'a', b'b', b'c', b'd']), 0])?;
        roundtrip(
            "abcdefg",
            &[
                u32::from_ne_bytes([b'a', b'b', b'c', b'd']),
                u32::from_ne_bytes([b'e', b'f', b'g', 0]),
            ],
        )?;
        roundtrip(
            "abcdefgh",
            &[
                u32::from_ne_bytes([b'a', b'b', b'c', b'd']),
                u32::from_ne_bytes([b'e', b'f', b'g', b'h']),
                0,
            ],
        )?;
        Ok(())
    }
}
