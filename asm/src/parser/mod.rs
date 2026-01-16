mod expr;

use crate::ir::Item;

use crate::{
    AssembleError,
    encoder,
    ir::{Instruction, Mnemonic, Operand},
    lexer::{self, Token},
    parser::expr::parse_expr,
    symbol::SymbolTable,
};

pub fn parse_line(
    line: &str,
    line_no: usize,
    symbols: &mut SymbolTable,
    pc: &mut i64,
) -> Result<Option<Item>, AssembleError> {
    // Ignore comments
    let code = line.split_once(';').map_or(line, |(c, _)| c).trim();
    if code.is_empty() {
        return Ok(None);
    }

    // Tokenize the line
    let tokens = lexer::lex(code);

    // Handle label definitions
    if matches!(tokens.get(1), Some(Token::Colon)) {
        let Token::Identifier(name) = tokens[0] else {
            return Err(AssembleError::Syntax);
        };
        let id = symbols.intern(name);
        symbols.define(id, *pc).map_err(|_| AssembleError::DuplicateSymbol)?;
        return Ok(Some(Item::Label(id)));
    }

    // Otherwise assume it's an instruction
    parse_instruction(&tokens, pc, symbols).map(Some)
}

fn parse_instruction(
    tokens: &[Token],
    pc: &mut i64,
    symbols: &mut SymbolTable,
) -> Result<Item, AssembleError> {
    let Token::Identifier(mnemonic) = tokens[0] else {
        return Err(AssembleError::Syntax);
    };

    let mnemonic = Mnemonic::from(mnemonic).ok_or(AssembleError::UnknownMnemonic)?;
    let operands = parse_operands(&tokens[1..], symbols)?;
    *pc += encoder::INSTRUCTION_SIZE;

    Ok(Item::Instruction(Instruction {
        mnemonic,
        operands,
    }))
}

fn parse_operands(
    tokens: &[Token],
    symbols: &mut SymbolTable,
) -> Result<Vec<Operand>, AssembleError> {
    let mut ops = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Register(reg) => {
                ops.push(Operand::Register(*reg));
                i += 1;
            },
            _ => {
                let (expr, consumed) = parse_expr(&tokens[i..], symbols)?;
                ops.push(Operand::Immediate(expr));
                i += consumed;
            }
        }

        // Ignore the comma between operands
        if matches!(tokens.get(i), Some(Token::Comma)) {
            i += 1;
        }
    }

    Ok(ops)
}
