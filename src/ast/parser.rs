use crate::ast::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    Identifier(String), // Variable, function names
    Number(i32), // Integer literals TODO: Add support for floating point numbers

    VariableDeclaration { // Declaration of a variable
        mutable: bool,
        name: String,
        value: Box<ASTNode>,
    },
    Assignment { // Assignment of a value to a variable
        name: String,
        value: Box<ASTNode>,
    },
    
    FunctionDeclaration { // Declaration of a function
        name: String,
        params: Vec<String>,
        body: Vec<ASTNode>,
    },
    FunctionCall { // Call to a function
        name: String,
        args: Vec<ASTNode>,
    },

    BinaryOp { // An operation that takes two operands
        op: String,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    UnaryOp { // An operation that takes one operand (e.g. negation)
        op: String,
        expr: Box<ASTNode>,
    },

    // TODO: Add more AST nodes
}

fn parse_expr(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> ASTNode {
    let mut left = match tokens.next() {
        Some(Token::Number(value)) => ASTNode::Number(*value),
        Some(Token::Identifier(name)) => ASTNode::Identifier(name.clone()),
        
        _ => panic!("Unexpected token"),
    };

    while let Some(&token) = tokens.peek() {
        match token {
            Token::Operator(op) => {
                tokens.next(); // Consume the operator
                let right = parse_expr(tokens);
                left = ASTNode::BinaryOp { op: op.to_string(), left: Box::new(left), right: Box::new(right) };
            },
            _ => break,
        }
    }

    left
}

pub fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut nodes = Vec::new();
    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        match token {
            Token::Identifier(name) => {
                match tokens.peek() {
                    Some(&Token::Symbol('=')) => {
                        tokens.next(); // Consume the '=' symbol
                        let value = parse_expr(&mut tokens);
                        nodes.push(ASTNode::Assignment { name: name.clone(), value: Box::new(value) });
                    },
                    Some(&Token::Symbol('(')) => {
                        tokens.next(); // Consume the '(' symbol
                        let mut args = Vec::new();
                        let mut level = 1;
                        let mut current_expr = Vec::new();
                        while let Some(token) = tokens.next() {
                            fn push_expr(expr: &mut Vec<Token>, nodes: &mut Vec<ASTNode>) {
                                if !expr.is_empty() {
                                    nodes.push(parse_expr(&mut expr.iter().peekable()));
                                }
                            }
                            match token {
                                Token::Symbol('(') => level += 1,
                                Token::Symbol(')') => {
                                    if level == 1 {
                                        push_expr(&mut current_expr, &mut args);
                                    }
                                    level -= 1;
                                }
                                Token::Symbol(',') => {
                                    push_expr(&mut current_expr, &mut args);
                                    current_expr = Vec::new();
                                },
                                _ => {
                                    current_expr.push(token.clone());
                                },
                            }
                            if level == 0 {
                                break;
                            }
                        }
                        nodes.push(ASTNode::FunctionCall { name: name.clone(), args });
                    },
                    _ => nodes.push(ASTNode::Identifier(name.clone())),
                }
            },
            Token::Keyword(name) => {
                match name.as_str() {
                    "var" => {
                        let name = match tokens.next() {
                            Some(Token::Identifier(name)) => name.clone(),
                            _ => panic!("Expected an identifier"),
                        };

                        if let Some(&Token::Symbol('=')) = tokens.peek() {
                            tokens.next(); // Consume the '=' symbol
                            let value = parse_expr(&mut tokens);
                            nodes.push(ASTNode::VariableDeclaration { mutable: false, name, value: Box::new(value) });
                        } else {
                            nodes.push(ASTNode::VariableDeclaration { mutable: false, name, value: Box::new(ASTNode::Number(0)) });
                        }
                    },
                    "mut" => {
                        let name = match tokens.next() {
                            Some(Token::Identifier(name)) => name.clone(),
                            _ => panic!("Expected an identifier"),
                        };

                        if let Some(&Token::Symbol('=')) = tokens.peek() {
                            tokens.next(); // Consume the '=' symbol
                            let value = parse_expr(&mut tokens);
                            nodes.push(ASTNode::VariableDeclaration { mutable: true, name, value: Box::new(value) });
                        } else {
                            nodes.push(ASTNode::VariableDeclaration { mutable: true, name, value: Box::new(ASTNode::Number(0)) });
                        }
                    },
                    "fun" => {
                        let name = match tokens.next() {
                            Some(Token::Identifier(name)) => name.clone(),
                            _ => panic!("Expected an identifier"),
                        };

                        let mut params = Vec::new();
                        if let Some(&Token::Symbol('(')) = tokens.peek() {
                            tokens.next(); // Consume the '(' symbol
                            while let Some(&Token::Identifier(ref param)) = tokens.next() {
                                params.push(param.clone());
                                match tokens.next() {
                                    Some(Token::Symbol(',')) => {},
                                    Some(Token::Symbol(')')) => break,
                                    _ => panic!("Expected ',' or ')'"),
                                }
                            }
                        }

                        // The body is a code block starting with '{' and ending with '}'
                        let mut body = Vec::new();
                        if let Some(&Token::Symbol('{')) = tokens.peek() {
                            tokens.next(); // Consume the '{' symbol
                            // There can be {} nested, so we need to keep track of the nesting level
                            let mut level = 1;
                            while let Some(token) = tokens.next() {
                                match token {
                                    Token::Symbol('{') => level += 1,
                                    Token::Symbol('}') => level -= 1,
                                    _ => {},
                                }
                                if level == 0 {
                                    break;
                                }
                                body.push(token.clone());
                            }
                        } else {
                            panic!("Expected a code block");
                        }
                        
                        nodes.push(ASTNode::FunctionDeclaration { name, params, body: parse(body) });
                    },
                    _ => {},
                }
            }
            Token::Number(value) => nodes.push(ASTNode::Number(*value)),
            _ => {},
        }
    }

    nodes
}