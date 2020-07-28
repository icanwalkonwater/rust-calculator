//! # Parser module
//! Contains the functions used to parse the grammar.

use crate::ast::{Add, AddOp, Atom, Digit, Expr, Literal, Mul, MulOp};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Token {
    Plus,
    Minus,
    Times,
    Div,
    ParenStart,
    ParenEnd,
    E,
    Pi,
    Digit(char),
}

impl Token {
    pub fn is_add_op(self) -> bool {
        self == Token::Plus || self == Token::Minus
    }

    pub fn is_mul_op(self) -> bool {
        self == Token::Times || self == Token::Div
    }

    pub fn is_paren(self) -> bool {
        self == Token::ParenStart || self == Token::ParenEnd
    }

    pub fn is_literal(self) -> bool {
        match self {
            Token::E | Token::Pi | Token::Digit(_) => true,
            _ => false,
        }
    }
}

/// [Token]ize the given input string.
/// # Panics
/// If a part of the string failes to be recognized as a token.
pub fn tokenize(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut iterator = source.chars();
    while let Some(c) = iterator.next() {
        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Times),
            '/' => tokens.push(Token::Div),
            '(' => tokens.push(Token::ParenStart),
            ')' => tokens.push(Token::ParenEnd),
            'E' => tokens.push(Token::E),
            'p' => {
                if iterator.next().expect("Unexpected EOF") == 'i' {
                    tokens.push(Token::Pi);
                } else {
                    panic!("Unexpected token, expected 'i' to form 'pi'");
                }
            }
            d @ '0'..='9' => tokens.push(Token::Digit(d)),
            _ if c.is_whitespace() => {}
            _ => panic!("Unexpected token"),
        }
    }

    tokens
}

/// Parses this part of the grammar:
/// ```bnf
/// <expr>  ::= <add>```
pub fn parse_expr(tokens: &[Token]) -> Expr {
    let (add, tokens) = parse_add(tokens);

    if !tokens.is_empty() {
        panic!("There are tokens left");
    }

    Expr { inner: add }
}

/// Parses this part of the grammar:
/// ```bnf
/// <add>   ::= <mul> [ <addOp> <add> ]
/// <addOp> ::= '+' | '-'```
fn parse_add(tokens: &[Token]) -> (Add, &[Token]) {
    let (mul, tokens) = parse_mul(tokens);

    if !tokens.is_empty() && tokens[0].is_add_op() {
        let op = if tokens[0] == Token::Plus {
            AddOp::Add
        } else {
            AddOp::Sub
        };

        let (add, tokens) = parse_add(&tokens[1..]);

        (
            Add {
                lhs: mul,
                op: Some(op),
                rhs: Some(Box::new(add)),
            },
            tokens,
        )
    } else {
        (Add::small(mul), tokens)
    }
}

/// Parses this part of the grammar:
/// ```bnf
/// <mul>   ::= <atom> [ <mulOp> <mul> ]
/// <mulOp> ::= '*' / '/'```
fn parse_mul(tokens: &[Token]) -> (Mul, &[Token]) {
    let (atom, tokens) = parse_atom(tokens);

    if !tokens.is_empty() && tokens[0].is_mul_op() {
        let op = if tokens[0] == Token::Times {
            MulOp::Mul
        } else {
            MulOp::Div
        };

        let (mul, tokens) = parse_mul(&tokens[1..]);

        (
            Mul {
                lhs: atom,
                op: Some(op),
                rhs: Some(Box::new(mul)),
            },
            tokens,
        )
    } else {
        (Mul::small(atom), tokens)
    }
}

/// Parses this part of the grammar:
/// ```bnf
/// <atom> ::= <literal> | '(' <add> ')'```
fn parse_atom(tokens: &[Token]) -> (Atom, &[Token]) {
    if tokens[0] == Token::ParenStart {
        let (add, tokens) = parse_add(&tokens[1..]);
        assert_eq!(tokens[0], Token::ParenEnd);

        (Atom::Add(Box::new(add)), &tokens[1..])
    } else {
        let (lit, tokens) = parse_literal(tokens);

        (Atom::Literal(lit), tokens)
    }
}

/// Parses this part of the grammar:
/// ```bnf
/// <literal> ::= <digit> | 'e' | 'pi'```
fn parse_literal(tokens: &[Token]) -> (Literal, &[Token]) {
    if let Token::Digit(_) = tokens[0] {
        let (digit, tokens) = parse_digit(tokens);

        (Literal::Digit(digit), tokens)
    } else if let Token::E = tokens[0] {
        (Literal::E, &tokens[1..])
    } else if let Token::Pi = tokens[0] {
        (Literal::Pi, &tokens[1..])
    } else {
        unreachable!()
    }
}

/// Parses this part of the grammar:
/// ```bnf
/// <digit> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | <digit>```
fn parse_digit(tokens: &[Token]) -> (Digit, &[Token]) {
    let digits = tokens
        .into_iter()
        .take_while(|&t| match t {
            Token::Digit(_) => true,
            _ => false,
        })
        .map(|t| match t {
            Token::Digit(c) => c,
            _ => unreachable!(),
        })
        .collect::<String>();

    (
        digits.parse::<Digit>().expect("Fuck this"),
        &tokens[digits.len()..],
    )
}

#[cfg(test)]
mod tests {
    use crate::ast::{Add, AddOp, Atom, Expr, Literal, Mul, MulOp};
    use crate::parser::Token::{Digit, Div, Minus, ParenEnd, ParenStart, Pi, Plus, Times, E};
    use crate::parser::{
        parse_add, parse_atom, parse_digit, parse_expr, parse_literal, parse_mul, tokenize,
    };

    // <editor-fold>

    #[test]
    fn test_tokenize1() {
        let tokens = tokenize("1+ 1".into());
        assert_eq!(tokens, vec![Digit('1'), Plus, Digit('1')])
    }

    #[test]
    fn test_tokenize2() {
        let tokens = tokenize("1 +-*/ pi(E)".into());
        assert_eq!(
            tokens,
            vec![
                Digit('1'),
                Plus,
                Minus,
                Times,
                Div,
                Pi,
                ParenStart,
                E,
                ParenEnd
            ]
        )
    }

    #[test]
    #[should_panic]
    fn test_tokenize_fail1() {
        tokenize("a".into());
    }

    #[test]
    #[should_panic]
    fn test_tokenize_fail2() {
        tokenize("#".into());
    }

    #[test]
    #[should_panic]
    fn test_tokenize_fail3() {
        tokenize("PI".into());
    }

    // </editor-fold>

    // <editor-fold>

    #[test]
    fn test_parse_expr() {
        {
            let stream = tokenize("( (1-pi)*3+E ) / 4".into());
            let expr = parse_expr(&stream);
            assert_eq!(
                expr,
                Expr {
                    inner: Add::small(Mul {
                        lhs: Atom::Add(Box::new(Add {
                            lhs: Mul {
                                lhs: Atom::Add(Box::new(Add {
                                    lhs: Mul::small(Atom::Literal(Literal::Digit(1))),
                                    op: Some(AddOp::Sub),
                                    rhs: Some(Box::new(Add::small(Mul::small(Atom::Literal(
                                        Literal::Pi
                                    )))))
                                })),
                                op: Some(MulOp::Mul),
                                rhs: Some(Box::new(Mul::small(Atom::Literal(Literal::Digit(3))))),
                            },
                            op: Some(AddOp::Add),
                            rhs: Some(Box::new(Add::small(Mul::small(Atom::Literal(Literal::E)))))
                        })),
                        op: Some(MulOp::Div),
                        rhs: Some(Box::new(Mul::small(Atom::Literal(Literal::Digit(4)))))
                    })
                }
            );
        }
    }

    #[test]
    fn test_parse_add() {
        {
            let stream = tokenize("1+2".into());
            let (add, tokens) = parse_add(&stream);
            assert_eq!(
                add,
                Add {
                    lhs: Mul::small(Atom::Literal(Literal::Digit(1))),
                    op: Some(AddOp::Add),
                    rhs: Some(Box::new(Add::small(Mul::small(Atom::Literal(
                        Literal::Digit(2)
                    ))))),
                }
            );
            assert_eq!(tokens.len(), 0)
        }
        {
            let stream = tokenize("1+2*3".into());
            let (add, tokens) = parse_add(&stream);
            assert_eq!(
                add,
                Add {
                    lhs: Mul::small(Atom::Literal(Literal::Digit(1))),
                    op: Some(AddOp::Add),
                    rhs: Some(Box::new(Add::small(Mul {
                        lhs: Atom::Literal(Literal::Digit(2)),
                        op: Some(MulOp::Mul),
                        rhs: Some(Box::new(Mul::small(Atom::Literal(Literal::Digit(3))))),
                    }))),
                }
            );
            assert_eq!(tokens.len(), 0)
        }
    }

    #[test]
    fn test_parse_mul() {
        {
            let stream = tokenize("42".into());
            let (lit, tokens) = parse_mul(&stream);
            assert_eq!(lit, Mul::small(Atom::Literal(Literal::Digit(42))),);
            assert_eq!(tokens.len(), 0)
        }
        {
            let stream = tokenize("2*3".into());
            let (lit, tokens) = parse_mul(&stream);
            assert_eq!(
                lit,
                Mul {
                    lhs: Atom::Literal(Literal::Digit(2)),
                    op: Some(MulOp::Mul),
                    rhs: Some(Box::new(Mul::small(Atom::Literal(Literal::Digit(3))))),
                }
            );
            assert_eq!(tokens.len(), 0)
        }
        {
            let stream = tokenize("2/pi+1".into());
            let (lit, tokens) = parse_mul(&stream);
            assert_eq!(
                lit,
                Mul {
                    lhs: Atom::Literal(Literal::Digit(2)),
                    op: Some(MulOp::Div),
                    rhs: Some(Box::new(Mul::small(Atom::Literal(Literal::Pi)))),
                }
            );
            assert_eq!(tokens.len(), 2)
        }
    }

    #[test]
    fn test_parse_atom() {
        {
            let stream = tokenize("12".into());
            let (atom, tokens) = parse_atom(&stream);
            assert_eq!(atom, Atom::Literal(Literal::Digit(12)));
            assert_eq!(tokens.len(), 0)
        }
        {
            let stream = tokenize("(12)".into());
            let (atom, tokens) = parse_atom(&stream);
            assert_eq!(
                atom,
                Atom::Add(Box::new(Add::small(Mul::small(Atom::Literal(
                    Literal::Digit(12)
                )))))
            );
            assert_eq!(tokens.len(), 0)
        }
    }

    #[test]
    fn test_parse_literal() {
        {
            let stream = tokenize("145".into());
            let (lit, tokens) = parse_literal(&stream);
            assert_eq!(lit, Literal::Digit(145));
            assert_eq!(tokens.len(), 0)
        }
        {
            let stream = tokenize("E+1".into());
            let (lit, tokens) = parse_literal(&stream);
            assert_eq!(lit, Literal::E);
            assert_eq!(tokens.len(), 2)
        }
        {
            let stream = tokenize("pi*8".into());
            let (lit, tokens) = parse_literal(&stream);
            assert_eq!(lit, Literal::Pi);
            assert_eq!(tokens.len(), 2)
        }
    }

    #[test]
    fn test_parse_digit() {
        {
            let stream = vec![Digit('1'), Digit('2')];
            let (digit, tokens) = parse_digit(&stream);
            assert_eq!(digit, 12);
            assert_eq!(tokens.len(), 0);
        }
        {
            let stream = vec![Digit('4'), Digit('2'), Plus];
            let (digit, tokens) = parse_digit(&stream);
            assert_eq!(digit, 42);
            assert_eq!(tokens.len(), 1);
        }
    }

    // </editor-fold>
}
