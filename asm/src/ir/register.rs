#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,

    Rbp,
    Rsp,

    // Registers that are not directly addressable, but rather for internal encoding.
    // The parse method should not return these registers.
    Rip,
}

impl Register {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "r0" => Some(Register::R0),
            "r1" => Some(Register::R1),
            "r2" => Some(Register::R2),
            "r3" => Some(Register::R3),
            "r4" => Some(Register::R4),
            "r5" => Some(Register::R5),
            "r6" => Some(Register::R6),
            "r7" => Some(Register::R7),
            "rbp" => Some(Register::Rbp),
            "rsp" => Some(Register::Rsp),
            _ => None,
        }
    }
}
