use crate::{
    AssembleError,
    ir::Expression,
    lexer::Token,
    symbol::SymbolTable
};

// TODO: Fix expression parser being right-associative
pub fn parse_expr(
    tokens: &[Token],
    symbols: &mut SymbolTable,
) -> Result<(Expression, usize), AssembleError> {
    let mut idx = 0;

    let mut lhs = match &tokens[idx] {
        Token::Number(num) => {
            idx += 1;
            Expression::Const(*num)
        },
        Token::Identifier(ident) => {
            idx += 1;
            Expression::Symbol(symbols.intern(ident))
        },
        _ => return Err(AssembleError::InvalidExpression)
    };

    while idx < tokens.len() {
        lhs = match &tokens[idx] {
            Token::Plus => {
                idx += 1;
                let (rhs, used) = parse_expr(&tokens[idx..], symbols)?;
                idx += used;
                Expression::Add(Box::new(lhs), Box::new(rhs))
            },
            Token::Minus => {
                idx += 1;
                let (rhs, used) = parse_expr(tokens, symbols)?;
                idx += used;
                Expression::Sub(Box::new(lhs), Box::new(rhs))
            },
            _ => break
        }
    }

    Ok((lhs, idx))
}
