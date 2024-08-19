use crate::vm::{VM, symbol::DataType};
use crate::ast::parser::{ASTNode, Operator, AssignmentKind};

pub struct Interpreter {
    vm: VM,
    flags: InterpreterFlags,
}

struct InterpreterFlags {
    pub break_flag: bool,
    pub continue_flag: bool,
}
impl InterpreterFlags {
    pub fn new() -> InterpreterFlags {
        InterpreterFlags {
            break_flag: false,
            continue_flag: false,
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            vm: VM::new(),
            flags: InterpreterFlags::new(),
        }
    }

    fn compute_expr(&mut self, expr: ASTNode) -> Result<DataType, String> {
        match expr {
            ASTNode::Number(value) => Ok(DataType::Number(value)),
            ASTNode::Identifier(name) => {
                let result = self.vm.get_variable(&name);
                match result {
                    Ok(value) => match value {
                        Some(value) => Ok(value),
                        None => Err(format!("Variable {:?} not found", name)),
                    },
                    Err(e) => Err(e),       
                }
            },
            ASTNode::BinaryOp { left, op, right } => {
                let left_result = self.compute_expr(*left);
                let right_result = self.compute_expr(*right);

                let left = match left_result {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };
                let right = match right_result {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };
                
                // Only add numbers
                if !left.is_number() || !right.is_number() {
                    return Err(format!("Expected numbers, got {:?} and {:?}", left, right));
                }

                let result = match op {
                    Operator::Add => left + right,
                    Operator::Sub => left - right,
                    Operator::Mul => left * right,
                    Operator::Div => left / right,
                    Operator::Mod => left % right,

                    Operator::And => left & right,
                    Operator::Or => left | right,

                    Operator::Eq => if left == right { DataType::Number(1) } else { DataType::Number(0) },
                    Operator::Ne => if left != right { DataType::Number(1) } else { DataType::Number(0) },
                    Operator::Lt => if left < right { DataType::Number(1) } else { DataType::Number(0) },
                    Operator::Gt => if left > right { DataType::Number(1) } else { DataType::Number(0) },
                    Operator::Le => if left <= right { DataType::Number(1) } else { DataType::Number(0) },
                    Operator::Ge => if left >= right { DataType::Number(1) } else { DataType::Number(0) },

                    _ => panic!("Unexpected operator"),
                };

                Ok(result)
            },
            ASTNode::UnaryOp { op, expr } => {
                let expr_result = self.compute_expr(*expr);
                let expr = match expr_result {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };  

                match op {
                    Operator::Not => if self.vm.truthy_check(expr) { Ok(DataType::Number(0)) } else { Ok(DataType::Number(1)) },
                    Operator::Neg => Ok(-expr),
                    _ => panic!("Unexpected operator"),
                }
            },
            ASTNode::FunctionCall { name, args } => {
                let mut arg_indices = vec![];
                for (i, arg) in args.iter().enumerate() {
                    let arg_value = self.compute_expr(arg.clone());
                    match arg_value {
                        Ok(value) => {
                            match self.vm.get_or_add_to_memory(value) {
                                Ok(_) => arg_indices.push(i),
                                Err(e) => return Err(e),
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                
                let function = self.vm.get_function(&name);
                let (params, body, scope) = match function {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };

                let old_scopes = self.vm.scopes.clone();
                let old_pc = self.vm.pc;
                self.vm.scopes = vec![scope];
                self.vm.pc = 0;

                for (i, param) in params.iter().enumerate() {
                    let value = self.vm.get_register_value(i);
                    match value {
                        Ok(value) => {
                            match self.vm.declare_variable(param.clone(), value, false) {
                                Ok(_) => {},
                                Err(e) => return Err(e),
                            }
                        },
                        Err(e) => return Err(e),
                    }
                }

                // interpret the function body
                let result = self.interpret(body.clone());
                self.vm.scopes = old_scopes;
                self.vm.pc = old_pc;

                match result {
                    Ok(Some(value)) => Ok(value[0].clone()),
                    Ok(None) => Ok(DataType::Null()),
                    Err(e) => Err(e),
                }
            },
            _ => panic!("Expression {:?} not implemented yet", expr),
        }
    }

    fn match_node(&mut self, node: ASTNode) -> Result<Option<Vec<DataType>>, String> {
        match node {
            ASTNode::VariableDeclaration { mutable, name, value } => {
                let value = self.compute_expr(*value);
                match value {
                    Ok(value) => {
                        match self.vm.declare_variable(name, value, mutable) {
                            Ok(_) => return Ok(None),
                            Err(e) => return Err(e),
                        }
                    },
                    Err(e) => return Err(e),
                }
            },
            ASTNode::Assignment { name, kind, value } => {
                let value_result = self.compute_expr(*value);
                let value = match value_result {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };
                let current_value = self.vm.get_variable(&name);
                let current_value = match current_value {
                    Ok(value) => match value {
                        Some(value) => value,
                        None => return Err(format!("Variable {:?} not found", name)),
                    },
                    Err(e) => return Err(e),
                };
                let modified_value = match kind {
                    AssignmentKind::Assign => value,
                    AssignmentKind::Add => current_value + value,
                    AssignmentKind::Sub => current_value - value,
                    AssignmentKind::Mul => current_value * value,
                    AssignmentKind::Div => current_value / value,
                    AssignmentKind::Mod => current_value % value,
                };
                let address = self.vm.get_or_add_to_memory(modified_value);
                match address {
                    Ok(address) => {
                        match self.vm.set_variable_address(&name, address) {
                            Ok(_) => Ok(None),
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => Err(e),
                }
            },
            ASTNode::FunctionDeclaration { name, params, body } => {
                match self.vm.declare_function(name, params, body) {
                    Ok(_) => Ok(None),
                    Err(e) => Err(e),
                }
            },
            ASTNode::Return { expr } => {
                let value = self.compute_expr(*expr);
                match value {
                    Ok(value) => Ok(Some(vec![value])),
                    Err(e) => Err(e),
                }
            },

            ASTNode::IfStatement { condition, body, else_body, else_ifs } => {
                let condition_value = self.compute_expr(*condition);
                let is_truthy = match condition_value {
                    Ok(value) => self.vm.truthy_check(value),
                    Err(e) => return Err(e),
                };

                if is_truthy {
                    return self.interpret(body);
                } else {
                    for (condition, body) in else_ifs {
                        let condition_value = self.compute_expr(*condition);
                        let is_truthy = match condition_value {
                            Ok(value) => self.vm.truthy_check(value),
                            Err(e) => return Err(e),
                        };

                        if is_truthy {
                            return self.interpret(body);
                        }
                    }

                    return self.interpret(else_body);
                }
            },
            ASTNode::WhileStatement { condition, body } => {
                let mut output = None;
                loop {
                    let condition_value_result = self.compute_expr(*condition.clone());
                    let condition_value = match condition_value_result {
                        Ok(value) => value,
                        Err(e) => return Err(e),
                    };
                    if !self.vm.truthy_check(condition_value) {
                        break;
                    }
                    let result = self.interpret(body.clone());
                    if self.flags.break_flag {
                        self.flags.break_flag = false;
                        break;
                    }
                    match result {
                        Ok(value) => {
                            if let Some(value) = value {
                                output = Some(value);
                            }
                        },
                        Err(e) => return Err(e),
                    }
                };

                if let Some(value) = output {
                    return Ok(Some(value));
                } else {
                    return Ok(None);
                }
            },
            ASTNode::Break {  } => {
                self.flags.break_flag = true;
                Ok(None)
            },
            ASTNode::Continue {  } => {
                self.flags.continue_flag = true;
                Ok(None)
            },

            ASTNode::Output { expr } => {
                let expr_value = self.compute_expr(*expr);

                match expr_value {
                    Ok(value) => {
                        println!("{}", value);
                        Ok(None)
                    },
                    Err(e) => Err(e),
                }
            },

            ASTNode::MathBody { name, body } => {
                println!("Solving math expression: {:?}", name);
                return Ok(None);
            }
            
            _ => {
                panic!("Invalid node: {:?}", node);
            }
        }
    }

    pub fn interpret(&mut self, ast: Vec<ASTNode>) -> Result<Option<Vec<DataType>>, String> {
        for node in ast {
            let result = self.match_node(node);
            if self.flags.continue_flag {
                self.flags.continue_flag = false;
                return Ok(None);
            }

            match result {
                Ok(option_value) => {
                    if let Some(value) = option_value {
                        return Ok(Some(value));
                    }
                },
                Err(e) => return Err(e),
            }

        }
        
        Ok(None)
    }
}