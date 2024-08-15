mod ast;

fn main() {
    let code = "y + x = 1 + someFunction(1, 32 *2, 3)";
    let ast = ast::parse(code);
    println!("{:?}", ast);
}