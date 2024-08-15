// Lexer for our calculator programming language
// Define the symbol table
// Define types of tokens

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String), // Keywords such as 'let', 'if', 'else', ...
    Identifier(String), // Variable, function names
    Number(i32), // Integer literals TODO: Add support for floating point numbers
    Operator(String), // Operators such as '+', '-', '*', ...
    Symbol(char), // Symbols such as '(', ')', '{', '}', ...
    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Match whitespace and skip it
            ' ' | '\t' | '\n' => { chars.next(); },
            // Match numbers (TODO: Add support for floating point numbers)
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
            },
            // Match keywords and identifiers
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "var" | "mut" | "if" | "while" | "break" | "continue" | "fun" => tokens.push(Token::Keyword(ident)),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            },
            // Match operators
            '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' => {
                let mut op = String::new();
                while let Some(&c) = chars.peek() {
                    if "+-*/=<>".contains(c) {
                        op.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Operator(op));
            },
            // Match symbols
            ',' | '(' | ')' | '{' | '}' | '?' | '!' => {
                tokens.push(Token::Symbol(ch));
                chars.next();
            },
            // Unrecognized characters
            _ => panic!("Unexpected character: {}", ch),
        }
    }

    tokens.push(Token::EOF);
    tokens
}