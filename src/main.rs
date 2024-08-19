mod vm;
mod ast;
mod interpreter;
mod repl;

fn main() {
    // repl::start();

    let mut interpreter = interpreter::Interpreter::new();
    let code = r#"
    mut x = 0
    while 1 {
    x += 1
        if x > 10 {
            break
        }
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