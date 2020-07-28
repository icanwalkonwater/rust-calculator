use crate::ast::{Add, AddOp, Atom, Expr, Literal, Mul, MulOp};
use std::f32;

type EvalOutput = f32;

pub trait Eval {
    fn eval(&self) -> EvalOutput;
}

impl Eval for Expr {
    fn eval(&self) -> EvalOutput {
        self.inner.eval()
    }
}

impl Eval for Add {
    fn eval(&self) -> EvalOutput {
        match self.op {
            None => self.lhs.eval(),
            Some(AddOp::Add) => self.lhs.eval() + self.rhs.as_ref().unwrap().eval(),
            Some(AddOp::Sub) => self.lhs.eval() - self.rhs.as_ref().unwrap().eval(),
        }
    }
}

impl Eval for Mul {
    fn eval(&self) -> EvalOutput {
        match self.op {
            None => self.lhs.eval(),
            Some(MulOp::Mul) => self.lhs.eval() * self.rhs.as_ref().unwrap().eval(),
            Some(MulOp::Div) => self.lhs.eval() / self.rhs.as_ref().unwrap().eval(),
        }
    }
}

impl Eval for Atom {
    fn eval(&self) -> EvalOutput {
        match self {
            Atom::Add(add) => add.eval(),
            Atom::Literal(lit) => lit.eval(),
        }
    }
}

impl Eval for Literal {
    fn eval(&self) -> EvalOutput {
        match self {
            Literal::Digit(d) => *d as f32,
            Literal::E => f32::consts::E,
            Literal::Pi => f32::consts::PI,
        }
    }
}
