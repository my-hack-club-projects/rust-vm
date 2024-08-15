mod ast;

fn main() {
    let code = "someFunction(1, 32 *2, 3)";
    let ast = ast::parse(code);
    println!("{:?}", ast);
}