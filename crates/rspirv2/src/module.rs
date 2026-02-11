use crate::binary::ModuleReader;
use crate::operand::Word;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

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

/// A SPIR-V Module with a valid header.
///
/// See https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#_physical_layout_of_a_spir_v_module_and_instruction
#[derive(Clone, Debug)]
pub struct Module(Vec<Word>);

impl Module {
    pub fn new(header: SpirvHeader) -> Self {
        Self(header.to_array().to_vec())
    }

    pub fn from_words(words: Vec<Word>) -> Result<Self, ParseError> {
        let slf = Self(words);
        slf.header().validate()?;
        Ok(slf)
    }

    /// Parse a SPIR-V module from bytes. Endianness is automatically detected.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ParseError> {
        if bytes.len() % 4 != 0 {
            return Err(ParseError::BytesNotMultipleOfFour(bytes.len()));
        }
        let word_count = bytes.len() / 4;
        if bytes.len() < size_of::<SpirvHeader>() {
            return Err(ParseError::BytesTooShort(bytes.len()));
        }

        let magic = u32::from_ne_bytes(bytes[..4].try_into().unwrap());
        let stream = (0..word_count).map(|i| {
            Word(u32::from_ne_bytes([
                bytes[i * 4],
                bytes[i * 4 + 1],
                bytes[i * 4 + 2],
                bytes[i * 4 + 3],
            ]))
        });
        let words = if magic == SPIRV_MAGIC.0 {
            stream.collect()
        } else if magic.swap_bytes() == SPIRV_MAGIC.0 {
            stream.map(|i| Word(i.0.swap_bytes())).collect()
        } else {
            return Err(ParseError::MismatchedMagic(Word(magic)));
        };
        Self::from_words(words)
    }

    pub fn header(&self) -> SpirvHeader {
        SpirvHeader::from_slice(self.0.as_slice())
    }

    pub fn instructions(&self) -> &[Word] {
        &self.0[5..]
    }

    pub fn reader(&self) -> ModuleReader<'_> {
        ModuleReader::new(self.instructions())
    }
}

const HEADER_BYTES: usize = size_of::<SpirvHeader>();

#[derive(Clone, PartialEq)]
pub enum ParseError {
    MismatchedMagic(Word),
    VersionReservedNotZero(SpirvVersion),
    HeaderReservedNotZero(SpirvHeader),
    BytesTooShort(usize),
    BytesNotMultipleOfFour(usize),
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
                "The byte array of length {len} must be at least {HEADER_BYTES} bytes"
            ),
            ParseError::BytesNotMultipleOfFour(len) => {
                write!(f, "The byte array of length {len} must be a multiple of 4")
            }
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoke_magic() {
        assert_eq!(
            SPIRV_MAGIC.0,
            crate::core::grammar::GRAMMAR_CORE.magic_number
        );
    }

    #[test]
    fn test_spirv_version_roundtrip() {
        for expected in
            (0..=255).flat_map(|major| (0..=255).map(move |minor| SpirvVersion::new(major, minor)))
        {
            let parsed = SpirvVersion::from_word(expected.to_word());
            assert_eq!(expected, parsed);
        }
    }

    #[test]
    fn test_spirv_header_roundtrip() {
        let test = |array: [u32; 5]| {
            assert_eq!(
                SpirvHeader::from_array(array.map(|v| Word(v)))
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
            let module = Module::from_bytes(bytes);
            assert!(matches!(module, Ok(_)), "{:?}", module);
            assert_eq!(module.unwrap().header(), valid);
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
