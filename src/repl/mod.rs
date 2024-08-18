use crate::interpreter::Interpreter;

pub fn start() {
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let ast = crate::ast::parse(input);
        match ast {
            Ok(ast) => {
                match interpreter.interpret(ast) {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Runtime error: {}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("Parse error: {}", e);
            }
        }
    }
}