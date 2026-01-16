use std::io;

#[derive(Debug)]
pub enum AssembleError {
    Io(io::Error),
    Syntax,
    UnknownMnemonic,
    DuplicateSymbol,
    UndefinedSymbol,
    InvalidExpression,
}

impl From<io::Error> for AssembleError {
    fn from(e: io::Error) -> Self {
        AssembleError::Io(e)
    }
}
