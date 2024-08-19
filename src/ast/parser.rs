use core::panic;

use crate::ast::lexer::Token;

#[derive(Clone, Debug)]
pub enum AssignmentKind {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Neg,
}

fn precedence(op: &Operator) -> i32 {
    match op {
        Operator::Or => 1,
        Operator::And => 2,
        Operator::Eq | Operator::Ne => 3,
        Operator::Lt | Operator::Le | Operator::Gt | Operator::Ge => 4,
        Operator::Add | Operator::Sub => 5,
        Operator::Mul | Operator::Div | Operator::Mod => 6,
        Operator::Neg | Operator::Not => 7, // Unary operators
    }
}

#[derive(Clone, Debug)]
pub enum ASTNode {
    Identifier(String), // Variable, function names
    Number(i32), // Integer literals TODO: Add support for floating point numbers

    BinaryOp { // An operation that takes two operands
        left: Box<ASTNode>,
        op: Operator,
        right: Box<ASTNode>,
    },
    UnaryOp { // An operation that takes one operand (e.g. negation)
        op: Operator,
        expr: Box<ASTNode>,
    },

    VariableDeclaration { // Declaration of a variable
        mutable: bool,
        name: String,
        value: Box<ASTNode>,
    },
    Assignment { // Assignment of a value to a variable
        name: String,
        kind: AssignmentKind,
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

    IfStatement { // If statement
        condition: Box<ASTNode>,
        body: Vec<ASTNode>,
        else_body: Vec<ASTNode>,
        else_ifs: Vec<(Box<ASTNode>, Vec<ASTNode>)>,
    },
    WhileStatement { // While statement
        condition: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
    Break {},
    Continue {},
    Return { expr: Box<ASTNode> },
    Output { expr: Box<ASTNode> },


    // TODO: Add more AST nodes
}

fn parse_fn_call(name: String, tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<ASTNode, String> {
    tokens.next(); // Consume the '(' symbol
    let mut args = Vec::new();
    let mut level = 1;
    let mut current_expr = Vec::new();
    while let Some(token) = tokens.next() {
        fn push_expr(expr: &mut Vec<Token>, nodes: &mut Vec<ASTNode>) -> Result<(), String> {
            if !expr.is_empty() {
                match parse_expr(&mut expr.iter().peekable(), 0) {
                    Ok(node) => {
                        nodes.push(node);
                        return Ok(())
                    }
                    Err(err) => return Err(err),
                }
            } else {
                return Err("Expected expression".to_string());
            }
        }
        match token {
            Token::Symbol('(') => level += 1,
            Token::Symbol(')') => {
                if level == 1 {
                    match push_expr(&mut current_expr, &mut args) {
                        Ok(_) => {},
                        Err(err) => return Err(err),
                    }
                }
                level -= 1;
            }
            Token::Symbol(',') => {
                match push_expr(&mut current_expr, &mut args) {
                    Ok(_) => {},
                    Err(err) => return Err(err),
                }
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

    Ok(ASTNode::FunctionCall { name, args })
}

fn parse_parantheses(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<ASTNode, String> {
    let mut expr_tokens = Vec::new();
    let mut level = 1;
    while let Some(token) = tokens.next() {
        match token {
            Token::Symbol('(') => level += 1,
            Token::Symbol(')') => {
                level -= 1;
                if level == 0 {
                    break;
                }
            },
            _ => {},
        }
        expr_tokens.push(token.clone());
    }
    parse_expr(&mut expr_tokens.iter().peekable(), 0)
}

fn parse_expr(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>, min_prec: i32) -> Result<ASTNode, String> {
    // Parse the left-hand side expression (either a number, identifier, or a parenthesized expression)
    let mut left: ASTNode = match tokens.next() {
        Some(Token::Number(value)) => ASTNode::Number(*value),
        Some(Token::Identifier(name)) => {
            match tokens.peek() {
                Some(&Token::Symbol('(')) => {
                    match parse_fn_call(name.clone(), tokens) {
                        Ok(node) => node,
                        Err(err) => return Err(err),
                    }
                },
                _ => ASTNode::Identifier(name.clone()),
            }
        },
        Some(Token::Symbol('(')) => {
            match parse_parantheses(tokens) {
                Ok(node) => node,
                Err(err) => return Err(err),
            }
        }
        Some(Token::Operator(op)) => {
            let op_enum = match op.as_str() {
                "-" => Operator::Neg,
                "~" => Operator::Not,
                _ => return Err("Unexpected operator".to_string()),
            };
            let next_expr = parse_expr(tokens, precedence(&op_enum));
            match next_expr {
                Ok(node) => ASTNode::UnaryOp {
                    op: op_enum,
                    expr: Box::new(node),
                },
                Err(err) => return Err(err),
            }
        },
        _ => return Err("Unexpected token".to_string()),
    };

    // Process all operators following the left-hand side, respecting precedence
    while let Some(&Token::Operator(ref op_str)) = tokens.peek() {
        let op_enum = match op_str.as_str() {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            "%" => Operator::Mod,
            "&" => Operator::And,
            "|" => Operator::Or,
            "<" => Operator::Lt,
            "<=" => Operator::Le,
            ">" => Operator::Gt,
            ">=" => Operator::Ge,
            "==" => Operator::Eq,
            "~=" => Operator::Ne,
            _ => return Err("Unexpected operator".to_string()),
        };

        let prec = precedence(&op_enum);
        if prec < min_prec {
            break;
        }

        tokens.next(); // Consume the operator

        // Recursively parse the right-hand side of the expression, considering the next operator's precedence
        let right = parse_expr(tokens, prec + 1);
        match right {
            Ok(node) => {
                left = ASTNode::BinaryOp {
                    left: Box::new(left),
                    op: op_enum,
                    right: Box::new(node),
                };
            },
            Err(err) => return Err(err),
        }
    }

    Ok(left)
}


fn parse_body(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Vec<ASTNode>, String> {
    let mut nodes = Vec::new();
    let mut level = 1;
    while let Some(token) = tokens.next() {
        match token {
            Token::Symbol('{') => level += 1,
            Token::Symbol('}') => {
                level -= 1;
                if level == 0 {
                    break;
                }
            },
            _ => nodes.push(token.clone()),
        }
    }

    parse(nodes)
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<ASTNode>, String> {
    let mut nodes = Vec::new();
    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        match token {
            Token::Identifier(name) => {
                match tokens.peek() {
                    Some(&Token::Assigner(op)) => {
                        tokens.next(); // Consume the '=' symbol
                        let value = parse_expr(&mut tokens, 0);
                        let kind = match op.as_str() {
                            "=" => AssignmentKind::Assign,
                            "+=" => AssignmentKind::Add,
                            "-=" => AssignmentKind::Sub,
                            "*=" => AssignmentKind::Mul,
                            "/=" => AssignmentKind::Div,
                            "%=" => AssignmentKind::Mod,
                            _ => return Err("Unexpected assignment operator".to_string()),
                        };
                        match value {
                            Ok(node) => nodes.push(ASTNode::Assignment { name: name.clone(), kind, value: Box::new(node) }),
                            Err(err) => return Err(err),
                        }
                    },
                    Some(&Token::Symbol('(')) => {
                        match parse_fn_call(name.clone(), &mut tokens) {
                            Ok(node) => nodes.push(node),
                            Err(err) => return Err(err),
                        }
                    },
                    _ => {
                        return Err(format!("Unexpected token \"{}\"", name));
                    },
                }
            },
            Token::Keyword(name) => {
                match name.as_str() {
                    "var" => {
                        let name = match tokens.next() {
                            Some(Token::Identifier(name)) => name.clone(),
                            _ => return Err("Expected an identifier".to_string()),
                        };

                        match tokens.peek() {
                            Some(&Token::Assigner(op)) => {
                                if op.as_str() == "=" {
                                    tokens.next(); // Consume the '=' symbol
                                    let value = parse_expr(&mut tokens, 0);
                                    match value {
                                        Ok(node) => nodes.push(ASTNode::VariableDeclaration { mutable: false, name, value: Box::new(node) }),
                                        Err(err) => return Err(err),
                                    }
                                } else {
                                    return Err("Unexpected assignment operator during variable declaration".to_string());
                                }
                            },
                            _ => {
                                nodes.push(ASTNode::VariableDeclaration { mutable: false, name, value: Box::new(ASTNode::Number(0)) });
                            },
                        }
                    },
                    "mut" => {
                        let name = match tokens.next() {
                            Some(Token::Identifier(name)) => name.clone(),
                            _ => return Err("Expected an identifier".to_string()),
                        };
                        
                        match tokens.peek() {
                            Some(&Token::Assigner(op)) => {
                                if op.as_str() == "=" {
                                    tokens.next(); // Consume the '=' symbol
                                    let value = parse_expr(&mut tokens, 0);
                                    match value {
                                        Ok(node) => nodes.push(ASTNode::VariableDeclaration { mutable: true, name, value: Box::new(node) }),
                                        Err(err) => return Err(err),
                                    }
                                } else {
                                    return Err("Unexpected assignment operator during variable declaration".to_string());
                                }
                            },
                            _ => {
                                return Err("Expected an assignment operator during variable declaration".to_string());
                            },
                        }
                    },
                    "fun" => {
                        let name = match tokens.next() {
                            Some(Token::Identifier(name)) => name.clone(),
                            _ => return Err("Expected an identifier".to_string()),
                        };

                        let mut params = Vec::new();
                        if let Some(&Token::Symbol('(')) = tokens.peek() {
                            tokens.next(); // Consume the '(' symbol
                            while let Some(&Token::Identifier(ref param)) = tokens.next() {
                                params.push(param.clone());
                                match tokens.next() {
                                    Some(Token::Symbol(',')) => {},
                                    Some(Token::Symbol(')')) => break,
                                    _ => return Err("Expected ',' or ')'".to_string()),
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
                            return Err("Expected a code block".to_string());
                        }
                        
                        match parse(body) {
                            Ok(mut nodes) => nodes.push(ASTNode::FunctionDeclaration { name, params, body: nodes.clone() }),
                            Err(err) => return Err(err),
                        }
                    },
                    "if" => {
                        let mut condition_tokens = Vec::new();
                        while let Some(&token) = tokens.peek() {
                            match token {
                                Token::Symbol('{') => break,
                                _ => condition_tokens.push(token.clone()),
                            }
                            tokens.next();
                        }
                        let condition = parse_expr(&mut condition_tokens.iter().peekable(), 0);
                        match condition {
                            Err(err) => return Err(err),
                            _ => {},
                        }
                        if tokens.next() != Some(&Token::Symbol('{')) {
                            return Err("Expected a code block".to_string());
                        }
                        let body = parse_body(&mut tokens);
                        match body {
                            Err(err) => return Err(err),
                            _ => {},
                        }
                        let mut else_body = Vec::new();
                        let mut else_ifs = Vec::new();

                        while let Some(&token) = tokens.peek() {
                            match token {
                                Token::Keyword(ref name) => {
                                    match name.as_str() {
                                        "elseif" => {
                                            if !else_body.is_empty() {
                                                return Err("'elseif' must come before 'else'".to_string());
                                            }
                                            tokens.next(); // Consume the 'elseif' keyword

                                            let mut condition_tokens = Vec::new();
                                            while let Some(&token) = tokens.peek() {
                                                match token {
                                                    Token::Symbol('{') => break,
                                                    _ => condition_tokens.push(token.clone()),
                                                }
                                                tokens.next();
                                            }
                                            let condition = parse_expr(&mut condition_tokens.iter().peekable(), 0);
                                            if tokens.next() != Some(&Token::Symbol('{')) {
                                                return Err("Expected a code block".to_string());
                                            }
                                            match parse_body(&mut tokens) {
                                                Ok(nodes) => else_ifs.push((Box::new(condition.unwrap()), nodes)),
                                                Err(err) => return Err(err),
                                            }
                                        },
                                        "else" => {
                                            tokens.next(); // Consume the 'else' keyword
                                            if tokens.next() != Some(&Token::Symbol('{')) {
                                                return Err("Expected a code block".to_string());
                                            }
                                            match parse_body(&mut tokens) {
                                                Ok(nodes) => else_body = nodes,
                                                Err(err) => return Err(err),
                                            }
                                            break;
                                        },
                                        _ => break,
                                    }
                                },
                                _ => break,
                            }
                        }

                        match condition {
                            Ok(node) => nodes.push(ASTNode::IfStatement { condition: Box::new(node), body: body.unwrap(), else_body, else_ifs }),
                            Err(err) => return Err(err),
                        }

                    },
                    "while" => {
                        let mut condition_tokens = Vec::new();
                        while let Some(&token) = tokens.peek() {
                            match token {
                                Token::Symbol('{') => break,
                                _ => condition_tokens.push(token.clone()),
                            }
                            tokens.next();
                        }
                        let condition = parse_expr(&mut condition_tokens.iter().peekable(), 0);
                        if tokens.next() != Some(&Token::Symbol('{')) {
                            return Err("Expected a code block".to_string());
                        }
                        let body = parse_body(&mut tokens);
                        match body {
                            Err(err) => return Err(err),
                            _ => {},
                        }

                        match condition {
                            Ok(node) => nodes.push(ASTNode::WhileStatement { condition: Box::new(node), body: body.unwrap() }),
                            Err(err) => return Err(err),
                        }
                    },
                    "break" => {
                        nodes.push(ASTNode::Break {});
                    },
                    "continue" => {
                        nodes.push(ASTNode::Continue {});
                    },
                    "return" => {
                        let expr = parse_expr(&mut tokens, 0);
                        match expr {
                            Ok(node) => nodes.push(ASTNode::Return { expr: Box::new(node) }),
                            Err(err) => return Err(err),
                        }
                    },
                    "out" => {
                        let expr = parse_expr(&mut tokens, 0);
                        match expr {
                            Ok(node) => nodes.push(ASTNode::Output { expr: Box::new(node) }),
                            Err(err) => return Err(err),
                        }
                    }
                    _ => {},
                }
            }
            _ => {
                return Err(format!("Unexpected token {:?}", token));
            }
        }
    }

    Ok(nodes)
}