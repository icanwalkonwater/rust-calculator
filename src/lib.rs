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
pub mod parser;
pub mod eval;