mod vm;
mod ast;
mod interpreter;
mod solve;
mod repl;

fn main() {
    // repl::start();

    let mut interpreter = interpreter::Interpreter::new();
    let code = r#"
    var x = 1
    math m {
        2 * x - 7 = 5 - 4 * x
    }
    "#;

    let ast = ast::parse(code);
    match ast {
        Ok(ast) => {
            println!("{:?}", ast);
            match interpreter.interpret(ast) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}