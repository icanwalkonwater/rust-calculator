//! # Simple expression parser
//! Grammar used:
//! ```bnf
//! <expr>  ::= <add>
//! <add>   ::= <mul> [ <addOp> <add> ]
//! <addOp> ::= '+' | '-'
//! <mul>   ::= <atom> [ <mulOp> <mul> ]
//! <mulOp> ::= '*' / '/'
//! <atom> ::= <literal> | '(' <add> ')'
//! <literal> ::= <digit> | 'e' | 'pi'
//! <digit> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | <digit>
//! ```

pub mod ast;
pub mod eval;
pub mod parser;
pub mod token;

pub mod errors {
    use crate::token::Token;
    use thiserror::Error;

    pub type Result<T> = std::result::Result<T, ParserError>;

    #[derive(Error, Debug)]
    pub enum ParserError {
        #[error("Tokenization error: {0}")]
        Tokenize(String),
        #[error("Mismatched parenthesis !")]
        MismatchedParenthesis,
        #[error("Too much operands in the expression !")]
        TooMuchOperands,
        #[error("Not enough operands in the expression !")]
        NotEnoughOperands,
        #[error("Unexpected operator: {0}")]
        UnexpectedOperator(Token),
        #[error("Unsupported operator: {0}")]
        UnsupportedOperator(Token),
    }
}
