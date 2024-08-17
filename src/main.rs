mod vm;
mod ast;
mod interpreter;

fn main() {
    // multiline string
    let code = r#"
    1 # This is a comment
    2 + 2 #[[
    This is a multiline comment
    ]]

    var a = 5
    a + 5
    "#;

    let ast = ast::parse(code);
    println!("{:?}", ast);
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(ast);
}