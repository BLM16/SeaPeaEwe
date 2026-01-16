use crate::{
    error::AssembleError,
    ir::{Instruction, Item, Mnemonic, Operand},
    symbol::SymbolTable,
};

/// The size of each machine code instruction in bytes.
pub const INSTRUCTION_SIZE: i64 = 4;

pub struct Encoder<'a> {
    symbols: &'a SymbolTable,
    bytes: Vec<u8>,
}

impl<'a> Encoder<'a> {
    // TODO: Consider taking a size hint to preallocate the byte vector.
    pub fn new(symbols: &'a SymbolTable) -> Self {
        Encoder { symbols, bytes: Vec::new() }
    }

    pub fn encode_item(&mut self, item: &Item) -> Result<(), AssembleError> {
        match item {
            Item::Instruction(instr) => self.encode_instruction(instr)?,
            Item::Label(_) => {}
        }
        Ok(())
    }

    fn encode_instruction(&mut self, instr: &Instruction) -> Result<(), AssembleError> {
        match (instr.mnemonic, instr.operands.as_slice()) {
            (Mnemonic::Abs, [Operand::Register(reg)]) => {
                self.bytes.push(0x01);
                self.bytes.push(*reg as u8); // Register is #[repr(u8)] so this is safe
                self.bytes.extend_from_slice(&[0, 0]); // Padding
            },
            _ => return Err(AssembleError::Syntax)
        }

        Ok(())
    }

    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }
}
