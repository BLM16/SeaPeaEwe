use super::{Expression, Register};

#[derive(Debug)]
pub enum Operand {
    Register(Register),
    Immediate(Expression),
}
