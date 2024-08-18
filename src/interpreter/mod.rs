use crate::vm::{VM, symbol::DataType};
use crate::ast::parser::{ASTNode, Operator, AssignmentKind};
use crate::vm::instruction::Instruction;

pub struct Interpreter {
    vm: VM,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            vm: VM::new(),
        }
    }

    fn compute_expr(&mut self, expr: ASTNode) -> Result<DataType, String> {
        match expr {
            ASTNode::Number(value) => Ok(DataType::Number(value)),
            ASTNode::Identifier(name) => {
                let instructions = vec![
                    Instruction::LoadVar(0, name),
                ];
                self.vm.execute(instructions);
                Ok(self.vm.get_register_value(0))
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
                    panic!("Expected numbers, got {:?} and {:?}", left, right);
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
                            self.vm.load_value_into_register(i, value);
                            arg_indices.push(i);
                        }
                        Err(e) => return Err(e),
                    }
                }
                
                let (params, body, scope) = self.vm.get_function(&name);

                let old_scopes = self.vm.scopes.clone();
                let old_pc = self.vm.pc;
                // println!("Scope: {:?}", scope);
                self.vm.scopes = vec![scope];
                self.vm.pc = 0;

                for (i, param) in params.iter().enumerate() {
                    self.vm.declare_variable(param.clone(), self.vm.get_register_value(i), false);
                }

                // interpret the function body
                let result = self.interpret(body.clone());
                // println!("Result: {:?}", result); // 
                self.vm.scopes = old_scopes;
                self.vm.pc = old_pc;

                // if let Some(value) = result {
                //     value[0].clone()
                // } else {
                //     // println!("Function {:?} returned None", name);
                //     DataType::Null()
                // }
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
                // self.vm.load_value_into_register(0, value);
                // if mutable {
                //     vec![Instruction::DeclareMutVar(0, name)]
                // } else {
                //     vec![Instruction::DeclareVar(0, name)]
                // }
                match value {
                    Ok(value) => {
                        self.vm.declare_variable(name, value, mutable);
                    },
                    Err(e) => return Err(e),
                }
                Ok(None)
            },
            ASTNode::Assignment { name, kind, value } => {
                let value_result = self.compute_expr(*value);
                let value = match value_result {
                    Ok(value) => value,
                    Err(e) => return Err(e),
                };
                let current_value = self.vm.get_variable(&name).expect("Variable not found");
                let modified_value = match kind {
                    AssignmentKind::Assign => value,
                    AssignmentKind::Add => current_value + value,
                    AssignmentKind::Sub => current_value - value,
                    AssignmentKind::Mul => current_value * value,
                    AssignmentKind::Div => current_value / value,
                    AssignmentKind::Mod => current_value % value,
                };
                // self.vm.load_value_into_register(0, modified_value);
                // vec![Instruction::StoreVar(0, name)]
                let address = self.vm.get_or_add_to_memory(modified_value);
                self.vm.set_variable_address(&name, address);
                Ok(None)
            },
            ASTNode::FunctionDeclaration { name, params, body } => {
                self.vm.declare_function(name, params, body);
                Ok(None)
            },
            ASTNode::Return { expr } => {
                let value = self.compute_expr(*expr);
                // self.vm.load_value_into_register(0, value);
                // vec![Instruction::RetFunc(vec![0])]
                // Ok(Some(vec![value]))
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
                    // if let Some(value) = result {
                    //     output = Some(value);
                    // }
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
                // vec![Instruction::BreakWhile]
                Ok(None) // TODO: Make it return some kind of LoopEnd enum for the interpreter to handle
            },
            ASTNode::Continue {  } => {
                // vec![Instruction::ContinueWhile]
                Ok(None) // TODO: Make it return some kind of LoopEnd enum for the interpreter to handle
            },

            ASTNode::Output { expr } => {
                let expr_value = self.compute_expr(*expr);
                // self.vm.load_value_into_register(0, expr_value);
                // vec![Instruction::Out(0)]
                // println!("{}", expr_value);
                // Ok(None)

                match expr_value {
                    Ok(value) => {
                        println!("{}", value);
                        Ok(None)
                    },
                    Err(e) => Err(e),
                }
            }
            
            _ => {
                panic!("Invalid node: {:?}", node);
            }
        }
    }

    // pub fn precompile(&mut self, ast: Vec<ASTNode>) -> Vec<Instruction> {
    //     let mut instructions = vec![];

    //     // Convert into instructions
    //     for node in ast {
    //         let node_instructions = self.match_node(node);
    //         instructions.extend(node_instructions);
    //     }
        
    //     instructions
    // }

    pub fn interpret(&mut self, ast: Vec<ASTNode>) -> Result<Option<Vec<DataType>>, String> {
        for node in ast {
            let result = self.match_node(node);

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