//! # AST module
//! Contains the structures used to represent the grammar.

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub inner: Add,
}

#[derive(Debug, PartialEq)]
pub struct Add {
    pub lhs: Mul,
    pub op: Option<AddOp>,
    pub rhs: Option<Box<Add>>,
}

impl Add {
    pub fn small(lhs: Mul) -> Self {
        Self {
            lhs,
            op: None,
            rhs: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AddOp {
    Add,
    Sub,
}

#[derive(Debug, PartialEq)]
pub struct Mul {
    pub lhs: Atom,
    pub op: Option<MulOp>,
    pub rhs: Option<Box<Mul>>,
}

impl Mul {
    pub fn small(lhs: Atom) -> Self {
        Self {
            lhs,
            op: None,
            rhs: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MulOp {
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum Atom {
    Literal(Literal),
    Add(Box<Add>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Digit(Digit),
    E,
    Pi,
}

pub type Digit = i64;
