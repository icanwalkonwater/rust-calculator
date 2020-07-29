use rust_calculator::token::tokenize;

fn main() {
    let expr = std::env::args().skip(1).collect::<String>();
    let tokens = tokenize(expr);
    // let ast = parse_expr(&tokens);
    // println!("Result: {}", ast.eval());
}
