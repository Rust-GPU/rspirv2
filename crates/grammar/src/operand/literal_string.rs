use crate::binary::{DecodeError, InstructionReader, InstructionWriter};
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

impl Operand for LiteralString {
    const KIND: OperandKind = crate::core::operand_kinds::OPERAND_KIND_LITERAL_STRING;
}

impl OperandEncoding for LiteralString {
    const FIXED_LEN: Option<usize> = None;

    fn encode(&self, writer: &mut impl InstructionWriter) {
        let bytes = self.0.as_bytes();
        let words = bytes.len().div_ceil(4);
        writer.extend((0..words).map(|i| {
            Word(u32::from_ne_bytes([
                bytes[i],
                bytes[i + 1],
                bytes[i + 2],
                bytes[i + 3],
            ]))
        }))
    }

    fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
        let bytes = reader
            .flat_map(|w| u32::to_ne_bytes(w.0).into_iter())
            .take_while(|p| *p == 0)
            .collect::<Vec<_>>();
        Ok(Self(String::from_utf8(bytes)?))
    }
}
