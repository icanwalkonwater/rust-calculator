//! # Eval module
//! Contains the meaning of the different variants of Expr and operators.

use crate::ast::{BinOpType, Expr, Number, UnaryOpType};

pub trait Eval {
    fn eval(&self) -> Number;
}

impl Eval for Expr {
    fn eval(&self) -> f32 {
        match self {
            Expr::Number(num) => *num,
            Expr::E => std::f32::consts::E,
            Expr::Pi => std::f32::consts::PI,
            Expr::UnaryOp(UnaryOpType::Negate, operand) => -operand.eval(),
            Expr::UnaryOp(UnaryOpType::Noop, operand) => operand.eval(),
            Expr::BinOp(left, BinOpType::Add, right) => left.eval() + right.eval(),
            Expr::BinOp(left, BinOpType::Sub, right) => left.eval() - right.eval(),
            Expr::BinOp(left, BinOpType::Mul, right) => left.eval() * right.eval(),
            Expr::BinOp(left, BinOpType::Div, right) => left.eval() / right.eval(),
            Expr::BinOp(left, BinOpType::Pow, right) => left.eval().powf(right.eval()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinOpType, Expr, UnaryOpType};
    use crate::eval::Eval;

    #[test]
    fn eval_atom() {
        let atom = Expr::Number(42.);
        assert_eq!(atom.eval(), 42.);

        let atom = Expr::E;
        assert_eq!(atom.eval(), std::f32::consts::E);

        let atom = Expr::Pi;
        assert_eq!(atom.eval(), std::f32::consts::PI);
    }

    #[test]
    fn eval_unary() {
        let negate = Expr::UnaryOp(UnaryOpType::Negate, Expr::Number(1.).boxed());
        assert_eq!(negate.eval(), -1.);

        let noop = Expr::UnaryOp(UnaryOpType::Noop, Expr::Number(1.).boxed());
        assert_eq!(noop.eval(), 1.);
    }

    #[test]
    fn eval_bin() {
        let one = Expr::Number(1.);
        let two = Expr::Number(2.);

        let add = Expr::BinOp(one.clone().boxed(), BinOpType::Add, two.clone().boxed());
        assert_eq!(add.eval(), 3.);
        let sub = Expr::BinOp(one.clone().boxed(), BinOpType::Sub, two.clone().boxed());
        assert_eq!(sub.eval(), -1.);
        let mul = Expr::BinOp(one.clone().boxed(), BinOpType::Mul, two.clone().boxed());
        assert_eq!(mul.eval(), 2.);
        let div = Expr::BinOp(one.clone().boxed(), BinOpType::Div, two.clone().boxed());
        assert_eq!(div.eval(), 0.5);
        let pow = Expr::BinOp(one.clone().boxed(), BinOpType::Pow, two.clone().boxed());
        assert_eq!(pow.eval(), 1.);
    }
}
