use crate::{ast::parser::{ASTNode, Operator}, vm::VM};
use crate::vm::symbol::DataType;
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

pub fn get_known_values(vars: &Vec<String>, vm: &VM) -> HashMap<String, f64> {
    let mut known_values = HashMap::new();
    for var in vars {
        if let Ok(Some(value)) = vm.get_variable(var) {
            // known_values.insert(var.clone(), value);
            // only numbers
            match value {
                DataType::Number(num) => {
                    known_values.insert(var.clone(), num as f64);
                },
                _ => {}
            }
        }
    }
    known_values

}

fn extract_coefficients_ordered(
    node: &ASTNode,
    sign: f64,
    coefficients: &mut Vec<f64>,
    variables: &Vec<String>,
    constant: &mut f64,
    known_values: &HashMap<String, f64> // Add known values map as a parameter
) {
    match node {
        ASTNode::BinaryOp { left, right, op } => {
            match *op {
                Operator::Add => {
                    extract_coefficients_ordered(left, sign, coefficients, variables, constant, known_values);
                    extract_coefficients_ordered(right, sign, coefficients, variables, constant, known_values);
                }
                Operator::Sub => {
                    extract_coefficients_ordered(left, sign, coefficients, variables, constant, known_values);
                    extract_coefficients_ordered(right, -sign, coefficients, variables, constant, known_values);
                }
                Operator::Mul => {
                    if let ASTNode::Number(val) = **right {
                        extract_coefficients_ordered(left, sign * (val as f64), coefficients, variables, constant, known_values);
                    } else if let ASTNode::Number(val) = **left {
                        extract_coefficients_ordered(right, sign * (val as f64), coefficients, variables, constant, known_values);
                    } else {
                        panic!("Unsupported multiplication operation");
                    }
                }
                _ => panic!("Unsupported operator"),
            }
        }
        ASTNode::UnaryOp { op, expr } => {
            match *op {
                Operator::Neg => {
                    extract_coefficients_ordered(expr, -sign, coefficients, variables, constant, known_values);
                }
                _ => panic!("Unsupported unary operator"),
            }
        }
        ASTNode::Identifier(name) => {
            if let Some(value) = known_values.get(name) {
                // Identifier has a known value, treat it as a constant
                *constant += sign * value;
                println!("Found known value: {} = {}", name, value);
            } else if let Some(index) = variables.iter().position(|v| v == name) {
                // Update coefficients with the current sign
                coefficients[index] += sign;
            } else {
                panic!("Variable not found in variable list");
            }
        }
        ASTNode::Number(val) => {
            *constant += sign * (*val as f64);
        }
        _ => panic!("Unsupported ASTNode"),
    }
}

pub fn formulate_system(equations: Vec<ASTNode>, known_values: &HashMap<String, f64>) -> (DMatrix<f64>, DVector<f64>) {
    let mut variables = Vec::new();
    let mut coefficients_matrix = Vec::new();
    let mut constants_vector = Vec::new();

    for equation in &equations {
        let vars_in_eq = find_vars(equation);
        for var in vars_in_eq {
            if !variables.contains(&var) {
                variables.push(var);
            }
        }
    }

    for equation in equations {
        let mut coefficients = vec![0.0; variables.len()];
        let mut constant = 0.0;
        let (left, right) = match equation {
            ASTNode::MathExpression { left, right } => (left, right),
            _ => panic!("Expected MathExpression, found {:?}", equation),
        };

        extract_coefficients_ordered(&left, 1.0, &mut coefficients, &variables, &mut constant, known_values);
        extract_coefficients_ordered(&right, -1.0, &mut coefficients, &variables, &mut constant, known_values);

        coefficients_matrix.push(coefficients);
        constants_vector.push(constant);
    }

    let matrix = DMatrix::from_vec(coefficients_matrix.len(), variables.len(), coefficients_matrix.concat());
    let constants = DVector::from_vec(constants_vector);

    (matrix, constants)
}

pub fn solve_system(matrix: DMatrix<f64>, constants: DVector<f64>) -> Result<HashMap<String, f64>, String> {
    // Solve the linear system
    match matrix.lu().solve(&constants) {
        Some(solution) => {
            println!("Raw solution: {:?}", solution);
            let mut result = HashMap::new();
            for (i, value) in solution.iter().enumerate() {
                result.insert(format!("var{}", i + 1), *value);
            }
            Ok(result)
        }
        None => Err("No solution found".to_string()),
    }
}
