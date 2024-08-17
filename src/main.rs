mod vm;
mod ast;
mod interpreter;

fn main() {
    // multiline string
    let code = r#"
    var x = 7
    if x == 6 {
        123
    } elseif x == 5 {
        10
    } else {
        0
    }
    "#;

    let ast = ast::parse(code);
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(ast);
}