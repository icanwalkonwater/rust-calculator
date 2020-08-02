use rust_calculator::parser::Parser;
use rust_calculator::token::tokenize;

use rust_calculator::errors::Result;
use rust_calculator::eval::Eval;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<()> {
    let raw_expr = std::env::args().skip(1).collect::<String>();
    let tokens = tokenize(raw_expr)?;
    let parser = Parser::new(tokens);
    let expr = parser.parse()?;

    println!("{}", expr.eval());
    Ok(())
}
