mod vm;
mod ast;
mod interpreter;

fn main() {
    // multiline string
    let code = r#"
    var x = 1 + 2 * 3 / 4 % 5
    var y = -x
    x y
    "#;

    let ast = ast::parse(code);
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(ast);
}