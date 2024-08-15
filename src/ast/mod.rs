mod lexer;
mod parser;

pub fn parse(input: &str) -> Vec<lexer::Token>{
    let tokens = lexer::tokenize(input);
    // parser::parse(tokens) // TODO: Implement parser

    tokens
}