mod vm;
mod ast;
mod interpreter;
mod solve;
mod repl;

fn main() {
    // repl::start();

    let mut interpreter = interpreter::Interpreter::new();
    let code = r#"
    #var x = 1
    math m {
        # 2 * x + 2 = x - y
        # 3 * x + 2 * y = 0

        x = 1
        y = x
        # expect x = 1, y = 1
        # actual x = -1, y = 0
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