//! Module for Disassembly

use crate::binary::{DecodeError, ModuleReader};
use crate::inst::InstEncoding;
use crate::operand::Word;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Options for disassembly
#[derive(Clone, Debug, Default)]
pub struct DisOptions {}

impl DisOptions {
    pub fn like_rspirv() -> Self {
        Self::default()
    }

    pub fn like_spirv_tools() -> Self {
        Self::default()
    }
}

/// Context object for disassembly generation
#[derive(Clone, Debug, Default)]
pub struct DisContext {
    opt: DisOptions,
}

impl DisContext {
    pub fn new(opt: DisOptions) -> Self {
        Self { opt }
    }
}

impl Deref for DisContext {
    type Target = DisOptions;
    fn deref(&self) -> &Self::Target {
        &self.opt
    }
}

impl DerefMut for DisContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.opt
    }
}

/// A sequence of words that has been pre-processed and may be [`Display`]ed.
///
/// The `ISA: `[`InstEncoding`] generic determines for which instruction set these Words are disassembled.
pub struct DisModule<'a, ISA: InstEncoding> {
    words: &'a [Word],
    dis: DisContext,
    _phantom: PhantomData<ISA>,
}

impl<'a, ISA: InstEncoding> DisModule<'a, ISA> {
    pub fn new(words: &'a [Word], opt: DisOptions) -> Result<Self, DecodeError> {
        Ok(Self {
            words,
            dis: DisContext::new(opt),
            _phantom: PhantomData,
        })
    }
}

impl<'a, ISA: InstEncoding> Display for DisModule<'a, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut reader = ModuleReader::new(self.words);
        while let Some(mut inst) = reader.next().map_err(|_| std::fmt::Error)? {
            let inst = ISA::decode(&mut inst).map_err(|_| std::fmt::Error)?;
            writeln!(f, "{}", inst.dis(&self.dis))?;
        }
        Ok(())
    }
}
