use rust_calculator::parser::{tokenize, parse_expr};
use rust_calculator::eval::Eval;

fn main() {
    let expr = std::env::args().skip(1).collect::<String>();
    let tokens = tokenize(expr);
    let ast = parse_expr(&tokens);

    println!("Result: {}", ast.eval());
}
