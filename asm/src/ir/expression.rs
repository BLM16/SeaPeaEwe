use crate::symbol::SymbolId;

#[derive(Debug)]
pub enum Expression {
    Const(i64),
    Symbol(SymbolId),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
}
