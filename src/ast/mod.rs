mod lexer;
mod parser;

pub fn parse(input: &str) -> Vec<parser::ASTNode>{
    let tokens = lexer::tokenize(input);
    parser::parse(tokens)
}