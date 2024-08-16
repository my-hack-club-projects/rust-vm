use crate::ast::lexer::Token;

#[derive(Debug)]
enum AssignmentKind {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
enum Operator {
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

#[derive(Debug)]
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


    // TODO: Add more AST nodes
}

fn parse_fn_call(name: String, tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> ASTNode {
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

    ASTNode::FunctionCall { name: name.clone(), args }
}

fn parse_parantheses(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> ASTNode {
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
    parse_expr(&mut expr_tokens.iter().peekable())
}

fn parse_expr(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> ASTNode {
    let mut left = match tokens.next() {
        Some(Token::Number(value)) => ASTNode::Number(*value),
        Some(Token::Identifier(name)) => ASTNode::Identifier(name.clone()),
        Some(Token::Symbol('(')) => {
            parse_parantheses(tokens)
        },
        Some(Token::Operator(op)) => {
            let op_enum = match op.as_str() {
                "-" => Operator::Neg,
                "~" => Operator::Not,
                _ => panic!("Unexpected operator"),
            };
            let next_token = tokens.next();
            let expr = match next_token {
                Some(Token::Number(value)) => ASTNode::Number(*value),
                Some(Token::Identifier(name)) => match tokens.peek() {
                    Some(&Token::Symbol('(')) => parse_fn_call(name.clone(), tokens),
                    _ => ASTNode::Identifier(name.clone()),
                },
                Some(Token::Symbol('(')) => {
                    parse_parantheses(tokens)
                }
                _ => panic!("Expected a number or an identifier"),
            };
            ASTNode::UnaryOp { op: op_enum, expr: Box::new(expr) }
        },
        
        _ => panic!("Unexpected token"),
    };

    while let Some(&token) = tokens.peek() {
        match token {
            Token::Operator(op) => {
                tokens.next(); // Consume the operator
                let right = parse_expr(tokens);
                let op_enum = match op.as_str() {
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
                    _ => panic!("Unexpected operator"),
                };
                left = ASTNode::BinaryOp { op: op_enum, left: Box::new(left), right: Box::new(right) };
            },
            Token::Symbol('(') => {
                if let ASTNode::Identifier(name) = &left {
                    left = parse_fn_call(name.clone(), tokens);
                } else {
                    tokens.next(); // Consume the '(' symbol
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
                    left = parse_expr(&mut expr_tokens.iter().peekable());
                }
                // left = parse_fn_call(match left {
                //     ASTNode::Identifier(name) => name,
                //     _ => panic!("Expected an identifier"),
                // }, tokens);
            },
            _ => break,
        }
    }

    left
}

fn parse_body(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Vec<ASTNode> {
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

pub fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut nodes = Vec::new();
    let mut tokens = tokens.iter().peekable();

    while let Some(token) = tokens.next() {
        match token {
            Token::Identifier(name) => {
                match tokens.peek() {
                    Some(&Token::Assigner(op)) => {
                        tokens.next(); // Consume the '=' symbol
                        let value = parse_expr(&mut tokens);
                        let kind = match op.as_str() {
                            "=" => AssignmentKind::Assign,
                            "+=" => AssignmentKind::Add,
                            "-=" => AssignmentKind::Sub,
                            "*=" => AssignmentKind::Mul,
                            "/=" => AssignmentKind::Div,
                            "%=" => AssignmentKind::Mod,
                            _ => panic!("Unexpected assignment operator"),
                        };
                        nodes.push(ASTNode::Assignment { name: name.clone(), kind, value: Box::new(value) });
                    },
                    Some(&Token::Symbol('(')) => {
                        nodes.push(parse_fn_call(name.clone(), &mut tokens));
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
                    "if" => {
                        let mut condition_tokens = Vec::new();
                        while let Some(&token) = tokens.peek() {
                            match token {
                                Token::Symbol('{') => break,
                                _ => condition_tokens.push(token.clone()),
                            }
                            tokens.next();
                        }
                        let condition = parse_expr(&mut condition_tokens.iter().peekable());
                        if tokens.next() != Some(&Token::Symbol('{')) {
                            panic!("Expected a code block");
                        }
                        let body = parse_body(&mut tokens);
                        let mut else_body = Vec::new();
                        let mut else_ifs = Vec::new();

                        while let Some(&token) = tokens.peek() {
                            match token {
                                Token::Keyword(ref name) => {
                                    match name.as_str() {
                                        "elseif" => {
                                            if !else_body.is_empty() {
                                                panic!("'elseif' must come before 'else'");
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
                                            let condition = parse_expr(&mut condition_tokens.iter().peekable());
                                            if tokens.next() != Some(&Token::Symbol('{')) {
                                                panic!("Expected a code block");
                                            }
                                            let body = parse_body(&mut tokens);
                                            else_ifs.push((Box::new(condition), body));
                                        },
                                        "else" => {
                                            tokens.next(); // Consume the 'else' keyword
                                            if tokens.next() != Some(&Token::Symbol('{')) {
                                                panic!("Expected a code block");
                                            }
                                            else_body = parse_body(&mut tokens);
                                            break;
                                        },
                                        _ => break,
                                    }
                                },
                                _ => break,
                            }
                        }

                        nodes.push(ASTNode::IfStatement { condition: Box::new(condition), body, else_body, else_ifs });
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
                        let condition = parse_expr(&mut condition_tokens.iter().peekable());
                        if tokens.next() != Some(&Token::Symbol('{')) {
                            panic!("Expected a code block");
                        }
                        let body = parse_body(&mut tokens);

                        nodes.push(ASTNode::WhileStatement { condition: Box::new(condition), body });
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