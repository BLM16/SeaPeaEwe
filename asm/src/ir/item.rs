use super::Instruction;

#[derive(Debug)]
pub enum Item {
    Label(u32),
    Instruction(Instruction),
    // TODO: (consider adding stuff like) .org, .align, .data, etc.
}
