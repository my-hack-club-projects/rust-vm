mod vm;
mod ast;
mod interpreter;

fn main() {
    // multiline string
    let code = r#"
    fun rec_counter(n) {
        out n
        if n < 10 {
            return rec_counter(n + 1)
        } else {
            return n
        }

    }

    var x = rec_counter(1);
    out x
    "#;

    let ast = ast::parse(code);
    println!("{:?}", ast);
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.interpret(ast);
}