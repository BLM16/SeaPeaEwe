/// Represents a mnemonic in the SeaPeaEwe assembly language.
#[derive(Clone, Copy, Debug)]
pub enum Mnemonic {
    /// Absolute value instruction.
    Abs,
    /// Addition instruction.
    Add,
    /// Bitwise and instruction.
    And,
    /// Function call instruction.
    Call,
    /// Compare instruction.
    Cmp,
    /// Decrement instruction.
    Dec,
    /// Unsigned division instruction.
    Div,
    /// Halt instruction.
    Hlt,
    /// Signed division instruction.
    IDiv,
    /// Signed multiplication instruction.
    IMul,
    /// Increment instruction.
    Inc,
    /// Unconditional jump instruction.
    Jmp,
    /// Load effective address instruction.
    Lea,
    /// Move instruction.
    Mov,
    /// Unsigned multiplication instruction.
    Mul,
    /// Two's complement negation instruction.
    Neg,
    /// No operation instruction.
    Nop,
    /// Bitwise not (one's complement) instruction.
    Not,
    /// Bitwise or instruction.
    Or,
    /// Stack pop instruction.
    Pop,
    /// Stack push instruction.
    Push,
    /// Function return instruction.
    Ret,
    /// Rotate left instruction. Shifts bits to the left, wrapping around.
    Rol,
    /// Rotate right instruction. Shifts bits to the right, wrapping around.
    Ror,
    /// Arithmetic shift left instruction.
    Sal,
    /// Arithmetic shift right instruction.
    Sar,
    /// Logical shift left instruction.
    Shl,
    /// Logical shift right instruction.
    Shr,
    /// Subtraction instruction.
    Sub,
    /// Bitwise exclusive or instruction.
    Xor,
}

impl Mnemonic {
    pub fn from(s: &str) -> Option<Self> {
        Some(match s {
            "abs" => Mnemonic::Abs,
            "add" => Mnemonic::Add,
            "and" => Mnemonic::And,
            "call" => Mnemonic::Call,
            "cmp" => Mnemonic::Cmp,
            "dec" => Mnemonic::Dec,
            "div" => Mnemonic::Div,
            "hlt" => Mnemonic::Hlt,
            "idiv" => Mnemonic::IDiv,
            "imul" => Mnemonic::IMul,
            "inc" => Mnemonic::Inc,
            "jmp" => Mnemonic::Jmp,
            "lea" => Mnemonic::Lea,
            "mov" => Mnemonic::Mov,
            "mul" => Mnemonic::Mul,
            "neg" => Mnemonic::Neg,
            "nop" => Mnemonic::Nop,
            "not" => Mnemonic::Not,
            "or" => Mnemonic::Or,
            "pop" => Mnemonic::Pop,
            "push" => Mnemonic::Push,
            "ret" => Mnemonic::Ret,
            "rol" => Mnemonic::Rol,
            "ror" => Mnemonic::Ror,
            "sal" => Mnemonic::Sal,
            "sar" => Mnemonic::Sar,
            "shl" => Mnemonic::Shl,
            "shr" => Mnemonic::Shr,
            "sub" => Mnemonic::Sub,
            "xor" => Mnemonic::Xor,
            _ => return None,
        })
    }
}
