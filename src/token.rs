//! # Tokenizer module
//! Contains the logic to transform a [String] into a [Vec] of [Token]s.
//!
//! ## Example
//! ```rust
//! # fn main() {
//! # use rust_calculator::token::{tokenize, Token};
//! let tokens = tokenize("1+2*3".into()).unwrap();
//!
//! assert_eq!(
//!     tokens,
//!     vec![
//!         Token::Number(1.),
//!         Token::Plus,
//!         Token::Number(2.),
//!         Token::Times,
//!         Token::Number(3.)
//!     ]
//! );
//! # }
//! ```

use std::iter::Peekable;
use std::str::Chars;

use crate::errors::{ParserError, Result};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Token {
    Plus,
    Minus,
    UnaryPlus,
    UnaryMinus,
    Times,
    Pow,
    Slash,
    ParenStart,
    ParenEnd,
    E,
    Pi,
    Number(f32),

    Ignore,
}

// Meaning of these tokens
impl Token {
    pub fn is_atom(&self) -> bool {
        match self {
            Self::E | Self::Pi | Self::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_op(&self) -> bool {
        match self {
            Self::Plus
            | Self::Minus
            | Self::UnaryPlus
            | Self::UnaryMinus
            | Self::Times
            | Self::Slash
            | Self::Pow => true,
            _ => false,
        }
    }

    /// Assumes [Token#is_op] returned true.
    pub fn is_bin_op(&self) -> bool {
        match self {
            Self::Plus | Self::Minus | Self::Times | Self::Slash | Self::Pow => true,
            _ => false,
        }
    }

    /// Assumes [Token#is_op] returned true.
    pub fn is_left_assoc(&self) -> bool {
        match self {
            Self::Plus | Self::Minus | Self::Times | Self::Pow => true,
            _ => false,
        }
    }

    /// Assumes [Token#is_op] returned true.
    pub fn op_prec(&self) -> u32 {
        match self {
            Self::Plus | Self::Minus => 1,
            Self::Times | Self::Slash => 2,
            Self::Pow => 3,
            _ => 0,
        }
    }

    pub fn is_before_unary(&self) -> bool {
        match self {
            t if t.is_op() => true,
            Self::ParenStart => true,
            _ => false,
        }
    }
}

/// [Token]ize the given input string.
pub fn tokenize(source: String) -> Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();

    let mut iterator = source.chars().into_iter().peekable();
    while let Some(c) = iterator.next() {
        let token = match c {
            '+' => {
                if let Some(prev) = tokens.last() {
                    if prev.is_before_unary() {
                        Token::UnaryPlus
                    } else {
                        Token::Plus
                    }
                } else {
                    Token::UnaryPlus
                }
            }
            '-' => {
                if let Some(prev) = tokens.last() {
                    if prev.is_before_unary() {
                        Token::UnaryMinus
                    } else {
                        Token::Minus
                    }
                } else {
                    Token::UnaryMinus
                }
            }
            '*' => {
                if let Some('*') = iterator.peek() {
                    // Can safely unwrap
                    iterator.next().unwrap();
                    Token::Pow
                } else {
                    Token::Times
                }
            }
            '/' => Token::Slash,
            '(' => Token::ParenStart,
            ')' => Token::ParenEnd,
            'e' => Token::E,
            // Parse PI
            'p' => {
                if let Some('i') = iterator.peek() {
                    // Can safely unwrap
                    iterator.next();
                    Token::Pi
                } else {
                    return Err(ParserError::Tokenize("Expected token 'pi'".into()))?;
                }
            }
            digit @ '0'..='9' => tokenize_number(&mut iterator, digit)?,
            '.' => tokenize_number(&mut iterator, '.')?,
            c if c.is_whitespace() => Token::Ignore,
            _ => {
                return Err(ParserError::Tokenize(format!("Unexpected token '{}'", c)))?;
            }
        };

        if token != Token::Ignore {
            tokens.push(token)
        }
    }

    Ok(tokens)
}

/// Tokenize a single number according to the following grammar:
/// ```bnf
/// <number>  ::= <digits> [ "." [ <digits> ] ] | "." <digits>
/// <digits>  ::= "0" .. "9"
/// ```
fn tokenize_number(iterator: &mut Peekable<Chars>, first_digit: char) -> Result<Token> {
    let mut acc = String::new();
    acc.push(first_digit);

    // Read the integer part of the number
    // (or the decimal part if the first_digit was a dot)
    while let Some(digit @ '0'..='9') = iterator.peek() {
        acc.push(*digit);
        iterator.next();
    }

    // Sanity check, a single dot is not a valid number
    if first_digit == '.' && acc.len() == 1 {
        return Err(ParserError::Tokenize(
            "A single dot isn't a valid number !".into(),
        ))?;
    }

    // If the first char was a dot, we were reading the decimal part already, so skip this step.
    if first_digit != '.' {
        if let Some('.') = iterator.peek() {
            acc.push('.');
            iterator.next();

            while let Some(digit @ '0'..='9') = iterator.peek() {
                acc.push(*digit);
                iterator.next();
            }
        }
    }

    // We can safely unwrap because we are good at parsing
    let number = acc.parse::<f32>().unwrap();
    Ok(Token::Number(number))
}

#[cfg(test)]
mod tests {
    use crate::token::{tokenize, Token};

    #[test]
    fn tokenize_numbers() {
        let tokens = tokenize("012.345".into()).unwrap();
        assert_eq!(tokens, vec![Token::Number(12.345)]);

        let tokens = tokenize("pie".into()).unwrap();
        assert_eq!(tokens, vec![Token::Pi, Token::E,]);

        let tokens = tokenize("12".into()).unwrap();
        assert_eq!(tokens, vec![Token::Number(12.0)]);

        let tokens = tokenize("12.".into()).unwrap();
        assert_eq!(tokens, vec![Token::Number(12.0)]);

        let tokens = tokenize(".4".into()).unwrap();
        assert_eq!(tokens, vec![Token::Number(0.4)]);

        assert!(tokenize(".".into()).is_err());
    }

    #[test]
    fn tokenize_operators() {
        let tokens = tokenize("+2+-1-*/***".into()).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::UnaryPlus,
                Token::Number(2.),
                Token::Plus,
                Token::UnaryMinus,
                Token::Number(1.),
                Token::Minus,
                Token::Times,
                Token::Slash,
                Token::Pow,
                Token::Times,
            ]
        )
    }

    #[test]
    fn tokenize_other() {
        let tokens = tokenize(" \n\t".into()).unwrap();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn tokenize_fail() {
        assert!(tokenize("abc".into()).is_err());
        assert!(tokenize("%".into()).is_err());
    }
}
