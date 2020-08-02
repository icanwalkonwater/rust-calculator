//! # Parser module
//! Contains the functions used to parse the grammar.

use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::{BinOpType, Expr, UnaryOpType};
use crate::errors::{ParserError, Result};
use crate::token::Token;

#[derive(Debug)]
pub struct Parser {
    token_stream: Peekable<IntoIter<Token>>,
    output: Vec<Expr>,
    operators: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            token_stream: tokens.into_iter().peekable(),
            output: Vec::new(),
            operators: Vec::new(),
        }
    }
}

impl Parser {
    /// Consume the input and parse it using the Shunting-Yard algorithm implementation
    /// from [Wikipedia](https://en.wikipedia.org/wiki/Shunting-yard_algorithm) slightly modified.
    pub fn parse(mut self) -> Result<Expr> {
        while let Some(token) = self.token_stream.next() {
            // Number token
            if token.is_atom() {
                self.output.push(match token {
                    Token::Number(num) => Expr::Number(num),
                    Token::E => Expr::E,
                    Token::Pi => Expr::Pi,
                    _ => unreachable!(),
                });

            // Operator token
            } else if token.is_op() {
                // Consume every operators with higher precedence
                loop {
                    // Exit condition
                    if !self.operators.is_empty() {
                        let last = self.operators.last().unwrap();

                        if token != Token::ParenStart {
                            let last_prec = last.op_prec();
                            let current_prec = token.op_prec();

                            if last_prec < current_prec
                                || (last_prec == current_prec && !token.is_left_assoc())
                            {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }

                    // Loop content

                    // Apply the operator
                    let last = self.operators.pop().unwrap();
                    self.apply_op(last)?;
                }

                // Append to the operator stack
                self.operators.push(token);

            // Handle parenthesis
            } else if token == Token::ParenStart {
                self.operators.push(token);
            } else if token == Token::ParenEnd {
                loop {
                    // Loop condition
                    if let Some(Token::ParenStart) = self.operators.last() {
                        break;
                    } else if let None = self.operators.last() {
                        break;
                    }

                    // Loop content

                    // Apply the operator
                    let last = self.operators.pop().unwrap();
                    self.apply_op(last)?;
                }

                if let Some(Token::ParenStart) = self.operators.last() {
                    self.operators.pop().unwrap();
                } else {
                    Err(ParserError::MismatchedParenthesis)?
                }
            }
        }

        // Apply the remaining operators of the stack
        while let Some(op) = self.operators.pop() {
            if !op.is_paren() {
                self.apply_op(op)?;
            }
        }

        // Sanity check the output queue must contain only one item
        if self.output.len() == 1 {
            Ok(self.output.pop().unwrap())
        } else if self.output.is_empty() {
            Err(ParserError::NotEnoughOperands)?
        } else {
            Err(ParserError::TooMuchOperands)?
        }
    }

    fn apply_op(&mut self, op: Token) -> Result<()> {
        if op.is_unary_op() {
            let operand = self.pop_operand()?;
            let op = match op {
                Token::UnaryMinus => UnaryOpType::Negate,
                _ => UnaryOpType::Noop,
            };

            self.output.push(Expr::UnaryOp(op, Box::new(operand)));
        } else {
            /* else if op.is_bin_op()*/
            let right = self.pop_operand()?;
            let left = self.pop_operand()?;

            let expr_op = match op {
                Token::Plus => BinOpType::Add,
                Token::Minus => BinOpType::Sub,
                Token::Times => BinOpType::Mul,
                Token::Slash => BinOpType::Div,
                Token::TimesTimes => BinOpType::Pow,
                _ => unreachable!(),
            };

            self.output
                .push(Expr::BinOp(Box::new(left), expr_op, Box::new(right)))
        }

        Ok(())
    }

    fn pop_operand(&mut self) -> Result<Expr> {
        self.output
            .pop()
            .ok_or_else(|| ParserError::NotEnoughOperands)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinOpType, Expr, UnaryOpType};
    use crate::errors::ParserError;
    use crate::parser::Parser;
    use crate::token::tokenize;

    #[test]
    fn parse_unary_expr() -> Result<(), ParserError> {
        let parser = Parser::new(tokenize("+1".into())?);
        assert_eq!(
            parser.parse()?,
            Expr::UnaryOp(UnaryOpType::Noop, Box::new(Expr::Number(1.)))
        );

        let parser = Parser::new(tokenize("-1".into())?);
        assert_eq!(
            parser.parse()?,
            Expr::UnaryOp(UnaryOpType::Negate, Box::new(Expr::Number(1.)))
        );

        Ok(())
    }

    #[test]
    fn parse_bin_expr() {
        let parser = Parser::new(tokenize("1+2".into()).unwrap());
        assert_eq!(
            parser.parse().unwrap(),
            Expr::BinOp(
                Box::new(Expr::Number(1.)),
                BinOpType::Add,
                Box::new(Expr::Number(2.))
            )
        );

        let parser = Parser::new(tokenize("1-2".into()).unwrap());
        assert_eq!(
            parser.parse().unwrap(),
            Expr::BinOp(
                Box::new(Expr::Number(1.)),
                BinOpType::Sub,
                Box::new(Expr::Number(2.))
            )
        );

        let parser = Parser::new(tokenize("1*2".into()).unwrap());
        assert_eq!(
            parser.parse().unwrap(),
            Expr::BinOp(
                Box::new(Expr::Number(1.)),
                BinOpType::Mul,
                Box::new(Expr::Number(2.))
            )
        );

        let parser = Parser::new(tokenize("1/2".into()).unwrap());
        assert_eq!(
            parser.parse().unwrap(),
            Expr::BinOp(
                Box::new(Expr::Number(1.)),
                BinOpType::Div,
                Box::new(Expr::Number(2.))
            )
        );

        let parser = Parser::new(tokenize("1**2".into()).unwrap());
        assert_eq!(
            parser.parse().unwrap(),
            Expr::BinOp(
                Box::new(Expr::Number(1.)),
                BinOpType::Pow,
                Box::new(Expr::Number(2.))
            )
        );
    }

    #[test]
    fn bin_op_precedence() {
        let parser = Parser::new(tokenize("1**2**3/4*5+6".into()).unwrap());
        assert_eq!(
            parser.parse().unwrap(),
            Expr::BinOp(
                Expr::BinOp(
                    Expr::BinOp(
                        Expr::BinOp(
                            Expr::Number(1.).boxed(),
                            BinOpType::Pow,
                            Expr::BinOp(
                                Expr::Number(2.).boxed(),
                                BinOpType::Pow,
                                Expr::Number(3.).boxed(),
                            )
                            .boxed()
                        )
                        .boxed(),
                        BinOpType::Div,
                        Expr::Number(4.).boxed()
                    )
                    .boxed(),
                    BinOpType::Mul,
                    Expr::Number(5.).boxed()
                )
                .boxed(),
                BinOpType::Add,
                Expr::Number(6.).boxed()
            )
        );
    }

    #[test]
    fn parens_hell() {
        let parser = Parser::new(tokenize("((1+2)*((3/4)/(5**6))".into()).unwrap());
        assert_eq!(
            parser.parse().unwrap(),
            Expr::BinOp(
                Expr::BinOp(
                    Expr::Number(1.).boxed(),
                    BinOpType::Add,
                    Expr::Number(2.).boxed(),
                )
                .boxed(),
                BinOpType::Mul,
                Expr::BinOp(
                    Expr::BinOp(
                        Expr::Number(3.).boxed(),
                        BinOpType::Div,
                        Expr::Number(4.).boxed()
                    )
                    .boxed(),
                    BinOpType::Div,
                    Expr::BinOp(
                        Expr::Number(5.).boxed(),
                        BinOpType::Pow,
                        Expr::Number(6.).boxed(),
                    )
                    .boxed()
                )
                .boxed()
            )
        );
    }
}
