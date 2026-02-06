use crate::binary::DecodeError;
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

    pub fn next(&mut self) -> Result<Option<InstructionReader<'_>>, DecodeError> {
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
        let params = self.data.get(inst_offset..(inst_offset + op_len)).ok_or(
            DecodeError::InstructionTooLong {
                inst_offset,
                op_len,
                module_remaining: self.data.len(),
            },
        )?;
        self.offset += inst_offset;
        Ok(Some(InstructionReader {
            opcode,
            params,
            params_offset: 0,
            inst_offset,
        }))
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
    /// Informational, only for error reporting
    inst_offset: usize,
}

impl<'a> InstructionReader<'a> {
    pub fn opcode(&self) -> u16 {
        self.opcode
    }

    pub fn peek(&self) -> Result<Word, DecodeError> {
        Ok(*self.params.get(self.params_offset).ok_or(
            DecodeError::InstructionDecodePulledTooManyWords {
                inst_offset: self.inst_offset,
                op_len: self.params.len(),
            },
        )?)
    }

    pub fn pull(&mut self) -> Result<Word, DecodeError> {
        let result = self.peek();
        self.params_offset += 1;
        result
    }
}

impl<'a> Iterator for InstructionReader<'a> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        self.pull().ok()
    }
}
