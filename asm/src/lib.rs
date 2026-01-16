mod config;
mod encoder;
mod error;
mod ir;
mod lexer;
mod parser;
mod symbol;

pub use config::Config;
pub use error::AssembleError;

use encoder::Encoder;
use symbol::SymbolTable;
use std::io::{BufRead, Write};

pub fn assemble<R: BufRead, W: Write>(
    mut input: R,
    mut output: W,
    config: &Config,
) -> Result<(), AssembleError> {
    let mut items = Vec::new();
    let mut symbols = SymbolTable::new();

    // ---------- PASS 1: parse + layout ----------
    let mut pc: i64 = 0;
    let mut line = String::new();
    let mut line_no = 1;

    while input.read_line(&mut line)? != 0 {
        if let Some(item) = parser::parse_line(
            &line,
            line_no,
            &mut symbols,
            &mut pc,
        )? {
            items.push(item);
        }

        line.clear();
        line_no += 1;
    }

    // ---------- PASS 2: resolve + encode ----------
    let mut encoder = Encoder::new(&symbols);

    for item in &items {
        encoder.encode_item(item)?;
    }

    output.write_all(&encoder.finish())?;
    Ok(())
}
