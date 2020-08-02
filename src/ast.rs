//! # AST module
//! Contains the structures used to represent the grammar.

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    BinOp(Box<Expr>, BinOpType, Box<Expr>),
    UnaryOp(UnaryOpType, Box<Expr>),
    Number(Number),
    E,
    Pi,
}

impl Expr {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BinOpType {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum UnaryOpType {
    Negate,
    Noop,
}

pub type Number = f32;
