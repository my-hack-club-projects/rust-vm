mod vm;
mod ast;
mod interpreter;

fn main() {
    // multiline string
    let code = r#"
    var a = 5

    fun f(x, y) {
        return x + y
    }

    f(5, 10)
    "#;

    let ast = ast::parse(code);
    println!("{:?}", ast);
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(ast);
}