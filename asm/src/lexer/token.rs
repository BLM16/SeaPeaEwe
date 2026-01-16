use crate::ir::Register;

#[derive(Clone, Copy, Debug)]
pub enum Token<'a> {
    Identifier(&'a str),
    Register(Register),
    Number(i64),
    Plus,
    Minus,
    Colon,
    Comma,
}
