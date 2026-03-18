use crate::Word;
use crate::binary::DecodeError;
use crate::meta::InstMeta;
use std::cmp::Ordering;
use std::ops::Deref;

/// Reader for an entire module
pub struct ModuleReader<'a> {
    words: &'a [Word],
    offset: usize,
}

impl<'a> ModuleReader<'a> {
    pub fn new(words: &'a [Word]) -> Self {
        Self { words, offset: 0 }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<Option<InstReader<'a>>, DecodeError> {
        let words = &self.words[self.offset..];
        let first = match words.first() {
            None => {
                // out of instructions
                return Ok(None);
            }
            Some(first) => *first,
        };
        let (opcode, op_len) = first.to_op();
        if op_len == 0 {
            // len must at least be 1, as it includes the op Word itself
            return Err(DecodeError::InstructionZeroSized);
        }
        let params = words
            .get(1..op_len)
            .ok_or(DecodeError::InstructionTooLong {
                op_len,
                module_remaining: words.len(),
            })?;
        self.offset += op_len;
        Ok(Some(InstReader::new(opcode, params)))
    }
}

/// Reader for a single instruction
#[derive(Copy, Clone, Debug)]
pub struct InstReader<'a> {
    /// opcode of the instruction
    opcode: u16,
    /// slice to the parameters of the instruction
    params: &'a [Word],
}

impl<'a> InstReader<'a> {
    pub fn new(opcode: u16, params: &'a [Word]) -> Self {
        Self { opcode, params }
    }

    pub fn opcode(&self) -> u16 {
        self.opcode
    }

    pub fn check_opcode(&self, meta: &InstMeta) -> Result<OperandReader<'a>, DecodeError> {
        if self.opcode != meta.opcode {
            Err(DecodeError::WrongOpCode {
                name: meta.opname,
                expected: meta.opcode,
                actual: self.opcode,
            })
        } else {
            Ok(self.operand_reader())
        }
    }

    pub fn operand_reader(&self) -> OperandReader<'a> {
        OperandReader {
            inst: *self,
            params_offset: 0,
        }
    }
}

/// Reader of Operand Words for a single instruction
///
/// Not `Copy` to prevent accidental copies.
#[derive(Clone, Debug)]
pub struct OperandReader<'a> {
    inst: InstReader<'a>,
    /// advancing offset pointing into params
    params_offset: usize,
}

impl<'a> Deref for OperandReader<'a> {
    type Target = InstReader<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inst
    }
}

impl<'a> OperandReader<'a> {
    /// Peek at the next [`Word`] in the [`InstReader`] without advancing the `params_offset`.
    ///
    /// Calling this again will yield the same value, advance the [`Self::params_offset`] by [`Self::pull`]ing the
    /// [`Word`].
    #[inline]
    pub fn peek(&self) -> Result<Word, DecodeError> {
        Ok(*self
            .params
            .get(self.params_offset)
            .ok_or(self.err_too_many_words())?)
    }

    /// Pull a single [`Word`] from the [`InstReader`], advancing the `params_offset`.
    ///
    /// Calling this again will yield the next [`Word`].
    #[inline]
    pub fn pull(&mut self) -> Result<Word, DecodeError> {
        let result = self.peek();
        // always advance, even if we're out of words, so `finalize()` fails
        self.advance();
        result
    }

    /// Finalize an [`OperandReader`] to verify *exactly* all operands have been consumed
    pub fn finalize(&self) -> Result<(), DecodeError> {
        let remaining = self.remaining();
        match 0.cmp(&remaining) {
            Ordering::Less => Err(DecodeError::InstructionWithAdditionalOperants {
                op_len: self.params.len(),
                remaining,
            }),
            Ordering::Equal => Ok(()),
            Ordering::Greater => Err(self.err_too_many_words()),
        }
    }

    fn err_too_many_words(&self) -> DecodeError {
        DecodeError::InstructionDecodePulledTooManyWords { op_len: self.len() }
    }

    /// View the *remaining* Words as a slice, does not advance the `params_offset`.
    #[inline]
    pub fn as_slice(&self) -> &'a [Word] {
        &self.inst.params[self.params_offset..]
    }

    /// Advance the `params_offset` by 1
    #[inline]
    pub fn advance(&mut self) {
        self.advance_by(1);
    }

    /// Advance the `params_offset` by an arbitrary amount
    #[inline]
    pub fn advance_by(&mut self, count: usize) {
        self.params_offset += count;
    }

    /// len of the params
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// current offset of the params, in [`Word`]s
    #[inline]
    pub fn params_offset(&self) -> usize {
        self.params_offset
    }

    /// amount of remaining param [`Word`]s
    #[inline]
    pub fn remaining(&self) -> usize {
        self.params.len() - self.params_offset
    }
}

impl Iterator for OperandReader<'_> {
    type Item = Word;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.pull().ok()
    }
}
