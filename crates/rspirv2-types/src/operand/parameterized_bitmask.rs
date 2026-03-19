use crate::Word;
use crate::binary::{DecodeError, EncodeError, OperandReader, WordSliceWriter, WordWriter};
use crate::operand::{OperandDisContext, OperandEncoding};
use smallvec::SmallVec;
use std::fmt::{Debug, Formatter};
use std::ops::Index;

pub const PARAMETERIZED_BITMASK_REQUIRES_FIXED_LEN: &str =
    "Parameterized bitmask requires operands to have a fixed size";

pub trait ParameterizedBitmaskBits: Copy + OperandEncoding + bitflags::Flags<Bits = u32> {
    const BIT_TO_EXTRA_LEN: &[usize];

    /// How many bits this bitmask has
    fn bits_cnt() -> u32 {
        Self::BIT_TO_EXTRA_LEN.len() as u32
    }

    fn assert_no_extra(bit: u32) {
        assert_eq!(Self::BIT_TO_EXTRA_LEN[bit as usize], 0);
    }

    /// Offset of the extra words for the given bit
    fn extra_offset(&self, bit: u32) -> usize {
        (0..bit)
            .map(|b| {
                if self.contains(Self::from_bits_retain(1 << b)) {
                    Self::BIT_TO_EXTRA_LEN[b as usize]
                } else {
                    0
                }
            })
            .sum()
    }

    /// Offset and len of the extra words for the given bit
    fn extra_offset_len(&self, bit: u32) -> (usize, usize) {
        (self.extra_offset(bit), Self::BIT_TO_EXTRA_LEN[bit as usize])
    }

    /// total len of words
    fn extra_total_len(&self) -> usize {
        self.extra_offset(Self::bits_cnt())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct ParameterizedBitmask<T: ParameterizedBitmaskBits> {
    bits: T,
    extra: SmallVec<[Word; 4]>,
}

impl<T: ParameterizedBitmaskBits> ParameterizedBitmask<T> {
    pub fn empty() -> Self {
        Self {
            bits: T::empty(),
            extra: SmallVec::new_const(),
        }
    }

    pub fn get_bool(&self, bit: u32) -> bool {
        T::assert_no_extra(bit);
        self.bits.contains(T::from_bits_retain(1 << bit))
    }

    pub fn get<P: OperandEncoding>(&self, bit: u32) -> Option<P> {
        let (offset, len) = self.bits.extra_offset_len(bit);
        assert_eq!(len, P::FIXED_LEN.unwrap());
        if self.bits.contains(T::from_bits_retain(1 << bit)) {
            let words = self.extra.index(offset..offset + len);
            Some(P::decode(&mut OperandReader::new(words)).unwrap())
        } else {
            None
        }
    }

    pub fn set_bool(&mut self, bit: u32, enabled: bool) {
        T::assert_no_extra(bit);
        self.bits.set(T::from_bits_retain(1 << bit), enabled);
    }

    pub fn set<P: OperandEncoding>(&mut self, bit: u32, param: Option<P>) {
        self.verify_consistency();
        let (offset, len) = self.bits.extra_offset_len(bit);
        assert_eq!(len, P::FIXED_LEN.unwrap());
        let mask = T::from_bits_retain(1 << bit);
        match param {
            None => {
                if self.bits.contains(mask) {
                    // remove
                    self.bits.remove(mask);
                    let range = offset..offset + len;
                    let mut index = 0;
                    self.extra.retain(|_| {
                        let retain = !range.contains(&index);
                        index += 1;
                        retain
                    });
                }
            }
            Some(param) => {
                if !self.bits.contains(mask) {
                    // insert
                    self.bits.insert(mask);
                    self.extra.insert_many(offset, (0..len).map(|_| Word(0)));
                }
                let mut writer = WordSliceWriter::new(&mut self.extra[offset..offset + len]);
                param.encode(&mut writer).unwrap();
                writer.finalize();
            }
        }
        self.verify_consistency();
    }

    #[inline]
    fn verify_consistency(&self) {
        debug_assert_eq!(self.extra.len(), self.bits.extra_total_len());
    }
}

unsafe impl<T: ParameterizedBitmaskBits> OperandEncoding for ParameterizedBitmask<T> {
    const FIXED_LEN: Option<usize> = None;

    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        self.bits.encode(&mut *writer)?;
        writer.write_iter(self.extra.iter().copied());
        Ok(())
    }

    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let bits = T::decode(&mut *reader)?;
        let extra = reader.take(bits.extra_total_len()).collect();
        let ret = Self { bits, extra };
        ret.verify_consistency();
        Ok(ret)
    }

    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        self.bits.dis_fmt(f, ctx)
    }
}
