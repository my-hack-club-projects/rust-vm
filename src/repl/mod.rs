use crate::interpreter::Interpreter;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use ctrlc;

pub fn start() {
    let ctrlc_flag = Arc::new(AtomicBool::new(false));
    let ctrlc_flag_clone = Arc::clone(&ctrlc_flag);
    
    ctrlc::set_handler(move || {
        ctrlc_flag_clone.store(true, Ordering::SeqCst);
        std::io::Write::write(&mut std::io::stdout(), b"\n> ").unwrap();
    }).expect("Error setting Ctrl-C handler");
    
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        // continue if ctrlc_flag
        if ctrlc_flag.load(Ordering::SeqCst) {
            ctrlc_flag.store(false, Ordering::SeqCst);
            continue;
        }

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
