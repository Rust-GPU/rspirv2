use crate::binary::{DecodeError, WordWriter};
use crate::inst::SpvInstEncoding;
use crate::vec::InstVec;
use crate::{Word, cast_words_to_ne_bytes};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

pub const SPIRV_MAGIC: Word = Word(0x07230203);

/// The version of the SPIR-V module
#[repr(C, align(4))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpirvVersion {
    /// reserved, must be 0
    pub reserved0: u8,
    /// major SPIR-V version
    pub major: u8,
    /// minor SPIR-V version
    pub minor: u8,
    /// reserved, must be 0
    pub reserved1: u8,
}

impl SpirvVersion {
    pub fn new(major: u8, minor: u8) -> SpirvVersion {
        Self {
            reserved0: 0,
            major,
            minor,
            reserved1: 0,
        }
    }

    #[inline]
    pub fn from_word(value: Word) -> Self {
        Self {
            reserved0: value.0 as u8,
            major: (value.0 >> 8) as u8,
            minor: (value.0 >> 16) as u8,
            reserved1: (value.0 >> 24) as u8,
        }
    }

    #[inline]
    pub fn to_word(self) -> Word {
        Word(
            (self.reserved0 as u32)
                | (self.major as u32) << 8
                | (self.minor as u32) << 16
                | (self.reserved1 as u32) << 24,
        )
    }

    pub fn validate(self) -> Result<(), ParseError> {
        if self.reserved0 != 0 || self.reserved1 != 0 {
            Err(ParseError::VersionReservedNotZero(self))
        } else {
            Ok(())
        }
    }
}

/// A SPIR-V header
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpirvHeader {
    /// The SPIR-V magic, must be [`SPIRV_MAGIC`]
    pub magic: Word,
    /// The SPIR-V version
    pub version: SpirvVersion,
    /// The magic of the program that generated this SPIR-V, may be 0
    pub generator_magic: Word,
    /// The highest result id of the module
    pub bound: Word,
    /// reserved, must be 0
    pub reserved: Word,
}

const HEADER_WORDS: usize = const {
    assert!(size_of::<SpirvHeader>().is_multiple_of(4));
    size_of::<SpirvHeader>() / 4
};

impl SpirvHeader {
    pub fn new(version: SpirvVersion, generator_magic: Word, bound: Word) -> Self {
        Self {
            magic: SPIRV_MAGIC,
            version,
            generator_magic,
            bound,
            reserved: Word(0),
        }
    }

    pub fn from_array(value: [Word; 5]) -> Self {
        Self::from_slice(&value)
    }

    pub fn from_slice(value: &[Word]) -> Self {
        Self {
            magic: value[0],
            version: SpirvVersion::from_word(value[1]),
            generator_magic: value[2],
            bound: value[3],
            reserved: value[4],
        }
    }

    pub fn to_array(self) -> [Word; 5] {
        [
            self.magic,
            self.version.to_word(),
            self.generator_magic,
            self.bound,
            self.reserved,
        ]
    }

    pub fn validate(self) -> Result<(), ParseError> {
        if self.magic != SPIRV_MAGIC {
            return Err(ParseError::MismatchedMagic(self.magic));
        }
        self.version.validate()?;
        if self.reserved != Word(0) {
            return Err(ParseError::HeaderReservedNotZero(self));
        }
        Ok(())
    }
}

/// A SPIR-V Module is an [`InstVec`] with an optional [`SpirvHeader`].
///
/// See <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#_physical_layout_of_a_spir_v_module_and_instruction>
#[derive(Clone, Debug, Default)]
pub struct Module<ISA: SpvInstEncoding> {
    pub header: Option<SpirvHeader>,
    pub inst: InstVec<ISA>,
}

impl<ISA: SpvInstEncoding> Module<ISA> {
    /// Parse a SPIR-V module from bytes, endianness is automatically detected and instructions checked for validity.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        profiling::function_scope!();
        let (header, inst_words) = Self::from_bytes_inner(bytes)?;
        Ok(Self {
            header: Some(header),
            inst: InstVec::from_words(inst_words)?,
        })
    }

    /// Parse a SPIR-V module from bytes, endianness is automatically detected.
    ///
    /// Instructions are not checked for validity, and you may get panics due to invalid instructions later on.
    pub fn from_bytes_unchecked(bytes: &[u8]) -> Result<Self, ParseError> {
        profiling::function_scope!();
        let (header, inst_words) = Self::from_bytes_inner(bytes)?;
        Ok(Self {
            header: Some(header),
            inst: InstVec::from_words_unchecked(inst_words),
        })
    }

    fn from_bytes_inner(bytes: &[u8]) -> Result<(SpirvHeader, Vec<Word>), ParseError> {
        profiling::function_scope!();
        let (chunks, remainder) = bytes.as_chunks();
        if !remainder.is_empty() {
            return Err(ParseError::BytesNotMultipleOfFour(chunks.len()));
        }
        if chunks.len() < HEADER_WORDS {
            return Err(ParseError::BytesTooShort(chunks.len()));
        }

        let magic = Word::from_le_bytes(chunks[0]);
        let requires_swap = magic == Word(SPIRV_MAGIC.0.swap_bytes());
        if (magic != SPIRV_MAGIC) && !requires_swap {
            return Err(ParseError::MismatchedMagic(magic));
        }

        let header = SpirvHeader::from_array(core::array::from_fn(|i| {
            if requires_swap {
                Word::from_be_bytes(chunks[i])
            } else {
                Word::from_le_bytes(chunks[i])
            }
        }));

        let inst_words = chunks
            .iter()
            .skip(HEADER_WORDS)
            .copied()
            .map(|w| {
                if requires_swap {
                    Word::from_be_bytes(w)
                } else {
                    Word::from_le_bytes(w)
                }
            })
            .collect::<Vec<_>>();
        Ok((header, inst_words))
    }

    /// Parse a SPIR-V [`Module`] from a word slice that may or may not contain a [`SpirvHeader`].
    pub fn from_words_maybe_header(words: &[Word]) -> Result<Self, ParseError> {
        match Self::from_bytes(cast_words_to_ne_bytes(words)) {
            Ok(e) => Ok(e),
            Err(ParseError::MismatchedMagic(..)) => Ok(Self {
                header: None,
                inst: InstVec::from_words(words.to_vec())?,
            }),
            Err(e) => Err(e),
        }
    }

    /// Parse a SPIR-V [`Module`] from a word slice that may or may not contain a [`SpirvHeader`].
    ///
    /// Instructions are not checked for validity, and you may get panics due to invalid instructions later on.
    pub fn from_words_maybe_header_unchecked(words: &[Word]) -> Result<Self, ParseError> {
        match Self::from_bytes_unchecked(cast_words_to_ne_bytes(words)) {
            Ok(e) => Ok(e),
            Err(ParseError::MismatchedMagic(..)) => Ok(Self {
                header: None,
                inst: InstVec::from_words_unchecked(words.to_vec()),
            }),
            Err(e) => Err(e),
        }
    }

    /// Write this module including the header to a [`WordWriter`]
    pub fn write_words(&self, writer: &mut impl WordWriter) {
        if let Some(header) = self.header {
            writer.write_iter(header.to_array());
        }
        writer.write_iter(self.inst.as_words().iter().copied());
    }

    /// Write this module including the header to a [`std::io::Write`], which could be a [`std::fs::File`]
    pub fn write_bytes(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        if let Some(header) = self.header {
            writer.write_all(cast_words_to_ne_bytes(&header.to_array()))?;
        }
        writer.write_all(cast_words_to_ne_bytes(self.inst.as_words()))?;
        Ok(())
    }
}

impl<ISA: SpvInstEncoding> Deref for Module<ISA> {
    type Target = InstVec<ISA>;

    fn deref(&self) -> &Self::Target {
        &self.inst
    }
}

#[derive(Clone, PartialEq)]
pub enum ParseError {
    MismatchedMagic(Word),
    VersionReservedNotZero(SpirvVersion),
    HeaderReservedNotZero(SpirvHeader),
    BytesTooShort(usize),
    BytesNotMultipleOfFour(usize),
    DecodeError(DecodeError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MismatchedMagic(magic) => {
                write!(f, "Expected SPIR-V magic {SPIRV_MAGIC:?}, got `{magic:?}`")
            }
            ParseError::VersionReservedNotZero(version) => write!(
                f,
                "The SPIR-V version `{version:?}` reserved fields are not 0"
            ),
            ParseError::HeaderReservedNotZero(header) => {
                write!(f, "The header `{header:?}` reserved fields are not 0")
            }
            ParseError::BytesTooShort(len) => write!(
                f,
                "The byte array of length {len} must be at least {HEADER_WORDS} words / {} bytes",
                HEADER_WORDS * 4
            ),
            ParseError::BytesNotMultipleOfFour(len) => {
                write!(f, "The byte array of length {len} must be a multiple of 4")
            }
            ParseError::DecodeError(e) => write!(f, "{e}"),
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for ParseError {}

impl From<DecodeError> for ParseError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spirv_version_roundtrip() {
        let test_corpus = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 16, 32, 64, 255];
        for expected in test_corpus.iter().flat_map(|major| {
            test_corpus
                .iter()
                .map(move |minor| SpirvVersion::new(*major, *minor))
        }) {
            let parsed = SpirvVersion::from_word(expected.to_word());
            assert_eq!(expected, parsed);
        }
    }

    #[test]
    fn test_spirv_header_roundtrip() {
        let test = |array: [u32; 5]| {
            assert_eq!(
                SpirvHeader::from_array(array.map(Word))
                    .to_array()
                    .map(|v| v.0),
                array
            );
        };
        test([0, 1, 2, 3, 5]);
        test([0x11111111, 0x22222222, 0x33333333, 0x44444444, 0x55555555]);
        test([0; 5]);
        test([!0; 5]);
    }

    #[test]
    fn test_spirv_header_validate() {
        let valid = SpirvHeader::new(SpirvVersion::new(1, 6), Word(0), Word(0));
        assert_eq!(valid.validate(), Ok(()));

        let mut wrong_magic = valid;
        wrong_magic.magic = Word(12345678);
        assert_eq!(
            wrong_magic.validate(),
            Err(ParseError::MismatchedMagic(Word(12345678)))
        );
    }

    #[test]
    fn test_from_bytes() {
        let valid = SpirvHeader::new(SpirvVersion::new(1, 6), Word(0), Word(0));
        assert_eq!(valid.validate(), Ok(()));

        let test = |bytes: &[u8]| {
            let module = Module::<()>::from_bytes(bytes);
            assert!(module.is_ok(), "{:?}", module);
            assert_eq!(module.unwrap().header, Some(valid));
        };

        let ne_bytes = valid
            .to_array()
            .iter()
            .flat_map(|v| v.0.to_ne_bytes())
            .collect::<Vec<_>>();
        test(&ne_bytes);
        let re_bytes = valid
            .to_array()
            .iter()
            .flat_map(|v| v.0.swap_bytes().to_ne_bytes())
            .collect::<Vec<_>>();
        test(&re_bytes);
    }
}
