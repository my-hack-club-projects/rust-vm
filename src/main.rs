mod vm;
mod ast;
mod interpreter;
mod repl;

// fn main() {
//     // multiline string
//     let code = r#"
//     var nterms = 30
//     fun recursive_fibonacci(n) {
//         if n <= 1 {
//             return n
//         }
//         return recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
//     }

//     mut i = 0
//     while i < nterms {
//         out recursive_fibonacci(i)
//         i += 1
//     }

//     "#;

//     let ast = ast::parse(code);
//     println!("{:?}", ast);
//     let mut interpreter = interpreter::Interpreter::new();
//     interpreter.interpret(ast);
// }

fn main() {
    repl::start();
}