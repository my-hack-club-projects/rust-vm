mod ast;

fn main() {
    let code = "fun main() { let x = 2 while x < 10 { x += 1 debug(x) if x >= 5 { break } } }";
    let ast = ast::parse(code);
    println!("{:?}", ast); // Note: not an AST yet, just the lexer tokens!
}