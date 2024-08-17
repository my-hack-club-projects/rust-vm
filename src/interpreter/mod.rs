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

    fn compute_expr(&mut self, expr: ASTNode) -> DataType {
        match expr {
            ASTNode::Number(value) => DataType::Number(value),
            ASTNode::Identifier(name) => {
                let instructions = vec![
                    Instruction::LoadVar(0, name),
                ];
                self.vm.execute(instructions);
                self.vm.get_register_value(0)
            },
            ASTNode::BinaryOp { left, op, right } => {
                let left = self.compute_expr(*left);
                let right = self.compute_expr(*right);

                
                // Only add numbers
                if !left.is_number() || !right.is_number() {
                    panic!("Expected numbers, got {:?} and {:?}", left, right);
                }

                match op {
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
                }
            },
            ASTNode::UnaryOp { op, expr } => {
                let expr = self.compute_expr(*expr);

                match op {
                    Operator::Not => !expr, // TODO: Check truthiness instead of trying to negate a number
                    Operator::Neg => -expr,
                    _ => panic!("Unexpected operator"),
                }
            },
            _ => panic!("Expression {:?} not implemented yet", expr),
        }
    }

    pub fn interpret(&mut self, ast: Vec<ASTNode>) {
        println!("{:?}", ast);

        // Convert into instructions
        for node in ast {
            match node {
                ASTNode::VariableDeclaration { mutable, name, value } => {
                    let value = self.compute_expr(*value);
                    self.vm.load_value_into_register(0, value);
                    let mut instructions = vec![];
                    if mutable {
                        instructions.push(Instruction::DeclareMutVar(0, name))
                    } else {
                        instructions.push(Instruction::DeclareVar(0, name))
                    }
                    self.vm.execute(instructions);
                },
                ASTNode::Assignment { name, kind, value } => {
                    let value = self.compute_expr(*value);
                    let current_value = self.vm.get_variable(&name).expect("Variable not found");
                    let modified_value = match kind {
                        AssignmentKind::Assign => value,
                        AssignmentKind::Add => current_value + value,
                        AssignmentKind::Sub => current_value - value,
                        AssignmentKind::Mul => current_value * value,
                        AssignmentKind::Div => current_value / value,
                        AssignmentKind::Mod => current_value % value,
                    };
                    self.vm.load_value_into_register(0, modified_value);
                    let instructions = vec![
                        Instruction::StoreVar(0, name),
                    ];
                    self.vm.execute(instructions);
                },

                ASTNode::Identifier(name) => {
                    let instructions = vec![
                        Instruction::LoadVar(0, name),
                        Instruction::Out(0),
                    ];
                    self.vm.execute(instructions);
                },

                _ => {
                    println!("{:?}", node);
                    panic!("Not implemented yet");
                }
            }
        }
    }
}