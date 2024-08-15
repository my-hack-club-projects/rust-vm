mod ast;

fn main() {
    // multiline string
    let code = r#"
    while x ~= 10 {
        x = x
    }
    "#;

    let ast = ast::parse(code);
    println!("{:?}", ast);
}