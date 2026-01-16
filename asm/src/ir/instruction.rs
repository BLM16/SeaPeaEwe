use super::{Mnemonic, Operand};

#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operands: Vec<Operand>,
}
