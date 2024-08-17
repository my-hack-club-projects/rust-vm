mod vm;
mod ast;
mod interpreter;

fn main() {
    // multiline string
    let code = r#"
    mut x = 0
    while x < 10 {
        x = x + 1
        x
    }
    "#;

    let ast = ast::parse(code);
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(ast);
}