//! # Parser module
//! Contains the functions used to parse the grammar.

use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::Expr;
use crate::errors::{Result, ParserError};
use crate::token::Token;
use std::string::ParseError;

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
    pub fn parse(&mut self) -> Result<()> {
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

                            if last_prec > current_prec
                                || (last_prec == current_prec && token.is_left_assoc())
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
                    self.append_op(last)?;
                }

            // Handle parenthesis
            } else if token == Token::ParenStart {
                self.operators.push(token);
            } else if token == Token::ParenEnd {
                loop {
                    // Loop condition
                    if let Some(Token::ParenStart) = self.operators.last() {
                        break;
                    } else if None = self.operators.last() {
                        break;
                    }

                    // Loop content

                    // Apply the operator
                    let last = self.operators.pop().unwrap();
                    self.append_op(last)?;
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
            self.append_op(op)?;
        }

        // Sanity check the output queue must contain only one item
        if self.output.len() != 1 {
            Err(ParserError::TooMuchOperands)?
        }

        Ok(())
    }

    fn append_op(&mut self, op: Token) -> Result<()> {
        todo!("Build AST here with the output queue");
        Ok(())
    }
}
