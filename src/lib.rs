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


pub mod token;
pub mod parser;
pub mod ast;
pub mod eval;

pub mod errors {
    use thiserror::Error;

    pub type Result<T> = std::result::Result<T, ParserError>;

    #[derive(Error, Debug)]
    pub enum ParserError {
        #[error("Tokenization error: {}")]
        Tokenize(String),
        #[error("Mismatched parenthesis !")]
        MismatchedParenthesis,
        #[error("Too much operands in the expression !")]
        TooMuchOperands,
    }
}
