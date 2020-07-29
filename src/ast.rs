//! # AST module
//! Contains the structures used to represent the grammar.

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    BinOpLeft(Box<Expr>, BinOpLeftType, Box<Expr>),
    BinOpRight(Box<Expr>, BinOpRightType, Box<Expr>),
    UnaryOp(UnaryOpType, Box<Expr>),
    Paren(Box<Expr>),
    Number(Number),
    E,
    Pi,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BinOpLeftType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BinOpRightType {
    Pow,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum UnaryOpType {
    Negate,
    Noop,
}

pub type Number = f32;
