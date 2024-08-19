use crate::ast::parser::{ASTNode, Operator};
use core::panic;
use std::collections::HashMap;
use nalgebra::{DMatrix, DVector};

pub fn find_vars(ast: &ASTNode) -> Vec<String> {
    let mut vars = Vec::new();
    
    match ast {
        ASTNode::Identifier(name) => {
            vars.push(name.clone());
        },
        _ => {
            let children = ast.children();
            for child in children {
                let child_vars = find_vars(child);
                vars.extend(child_vars);
            }
        }
    }

    vars
}

fn extract_coefficients(node: &ASTNode, sign: f64, coefficients: &mut HashMap<String, f64>, constant: &mut f64) {
    match node {
        ASTNode::BinaryOp { left, right, op } => {
            match *op {
                Operator::Add => {
                    extract_coefficients(left, sign, coefficients, constant);
                    extract_coefficients(right, sign, coefficients, constant);
                }
                Operator::Sub => {
                    extract_coefficients(left, sign, coefficients, constant);
                    extract_coefficients(right, -sign, coefficients, constant);
                }
                Operator::Mul => {
                    if let ASTNode::Number(val) = **right {
                        extract_coefficients(left, sign * (val as f64), coefficients, constant);
                    } else if let ASTNode::Number(val) = **left {
                        extract_coefficients(right, sign * (val as f64), coefficients, constant);
                    } else {
                        panic!("Unsupported multiplication operation");
                    }
                }
                _ => panic!("Unsupported operator"),
            }
        }
        ASTNode::Identifier(name) => {
            *coefficients.entry(name.clone()).or_insert(0.0) += sign;
        }
        ASTNode::Number(val) => {
            *constant += sign * (*val as f64);
        }
        _ => panic!("Unsupported ASTNode"),
    }
}

pub fn formulate_equation(ast: &ASTNode) -> (HashMap<String, f64>, f64) {
    let mut coefficients = HashMap::new();
    let mut constant = 0.0;

    if let ASTNode::MathExpression { left, right } = ast {
        extract_coefficients(left, 1.0, &mut coefficients, &mut constant);
        extract_coefficients(right, -1.0, &mut coefficients, &mut constant);
    } else {
        panic!("Expected MathExpression, found {:?}", ast);
    }

    (coefficients, -constant)
}

pub fn solve_equation(coefficients: &mut HashMap<String, f64>, constant: &mut f64) -> Result<HashMap<String, f64>, String> {
    let vars: Vec<String> = coefficients.keys().cloned().collect();
    let num_vars = vars.len();

    // Create the coefficient matrix and the constant vector
    let mut matrix = DMatrix::zeros(num_vars, num_vars);
    let mut constants = DVector::zeros(num_vars);

    for (i, var) in vars.iter().enumerate() {
        if let Some(&coeff) = coefficients.get(var) {
            matrix[(i, i)] = coeff;
        }
        constants[i] = *constant;
    }

    // Solve the linear system
    match matrix.lu().solve(&constants) {
        Some(solution) => {
            let mut result = HashMap::new();
            for (i, var) in vars.iter().enumerate() {
                result.insert(var.clone(), solution[i]);
            }
            Ok(result)
        }
        None => Err("No solution found".to_string()),
    }
}
