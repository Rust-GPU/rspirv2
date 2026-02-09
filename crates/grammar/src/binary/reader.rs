use crate::binary::DecodeError;
use crate::meta::InstMeta;
use crate::operand::Word;

/// Reader for an entire module
pub struct ModuleReader<'a> {
    data: &'a [Word],
    offset: usize,
}

impl<'a> ModuleReader<'a> {
    pub fn new(data: &'a [Word]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn next(&mut self) -> Result<Option<InstructionReader<'a>>, DecodeError> {
        let inst_offset = self.offset;
        let first = match self.data.get(inst_offset) {
            None => {
                // out of instructions
                return Ok(None);
            }
            Some(first) => first.0,
        };
        let opcode = first as u16;
        let op_len = (first >> 16) as usize;
        let params = self
            .data
            .get((inst_offset + 1)..(inst_offset + 1 + op_len))
            .ok_or(DecodeError::InstructionTooLong {
                inst_offset,
                op_len,
                module_remaining: self.data.len(),
            })?;
        self.offset += op_len + 1;
        Ok(Some(InstructionReader::new(opcode, params, inst_offset)))
    }
}

/// Reader for a single instruction
pub struct InstructionReader<'a> {
    /// opcode of the instruction
    opcode: u16,
    /// slice to the parameters of the instruction
    params: &'a [Word],
    /// advancing offset pointing into params
    params_offset: usize,
    inst_offset: usize,
}

impl<'a> InstructionReader<'a> {
    pub fn new(opcode: u16, params: &'a [Word], inst_offset: usize) -> Self {
        Self {
            opcode,
            params,
            params_offset: 0,
            inst_offset,
        }
    }

    pub fn opcode(&self) -> u16 {
        self.opcode
    }

    pub fn check_opcode(&self, meta: &InstMeta) -> Result<(), DecodeError> {
        if self.opcode != meta.opcode {
            Err(DecodeError::WrongOpCode {
                name: meta.opname,
                expected: meta.opcode,
                actual: self.opcode,
            })
        } else {
            Ok(())
        }
    }

    /// Peek at the next [`Word`] in the [`InstructionReader`] without advancing the [`Self::params_offset`].
    ///
    /// Calling this again will yield the same value, advance the [`Self::params_offset`] by [`Self::pull`]ing the
    /// [`Word`].
    pub fn peek(&self) -> Result<Word, DecodeError> {
        Ok(*self.params.get(self.params_offset).ok_or(
            DecodeError::InstructionDecodePulledTooManyWords {
                inst_offset: self.inst_offset,
                op_len: self.params.len(),
            },
        )?)
    }

    /// Pull a single [`Word`] from the [`InstructionReader`], advancing the [`Self::params_offset`].
    ///
    /// Calling this again will yield the next [`Word`].
    pub fn pull(&mut self) -> Result<Word, DecodeError> {
        let result = self.peek();
        self.params_offset += 1;
        result
    }

    /// Offset of the instruction, purely informational, for error reporting
    pub fn inst_offset(&self) -> usize {
        self.params_offset
    }

    /// len of the params
    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// current offset of the params, in [`Word`]s
    pub fn params_offset(&self) -> usize {
        self.params_offset
    }

    /// amount of remaining param [`Word`]s
    pub fn remaining(&self) -> usize {
        self.params.len() - self.params_offset
    }
}

impl<'a> Iterator for InstructionReader<'a> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        self.pull().ok()
    }
}
