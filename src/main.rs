mod vm;
mod ast;
mod interpreter;

fn main() {
    // multiline string
    let code = r#"
    fun f(a, b) {
        return a + b
    }
    "#;

    let ast = ast::parse(code);
    println!("{:?}", ast);
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(ast);
}