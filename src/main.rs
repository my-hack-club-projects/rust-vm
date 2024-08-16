mod ast;

fn main() {
    // multiline string
    let code = r#"
    
    "#;

    let ast = ast::parse(code);
    println!("{:?}", ast);
}