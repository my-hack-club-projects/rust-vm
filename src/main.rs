mod vm;
mod ast;
mod interpreter;
mod solve;
mod repl;

fn main() {
    // repl::start();

    let mut interpreter = interpreter::Interpreter::new();
    let code = r#"
    math m {
        x + y = 5
        x - y = 1
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