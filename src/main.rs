mod ast;

fn main() {
    // multiline string
    let code = r#"
    var x = 0
    mut y = 1
    while x ~= 10 {
        x = (x + 1) * 2
    }
    "#;

    let ast = ast::parse(code);
    println!("{:?}", ast);
}