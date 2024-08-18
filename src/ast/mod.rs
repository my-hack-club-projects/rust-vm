mod lexer;
pub mod parser;

pub fn parse(input: &str) -> Result<Vec<parser::ASTNode>, String> {
    let tokens = lexer::tokenize(input);
    parser::parse(tokens)
}