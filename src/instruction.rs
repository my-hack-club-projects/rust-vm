use crate::{symbol::DataType, vm::VM};

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Instruction {
    Halt,
    Out(usize),
    Debug(usize),

    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Mod(usize, usize),
    Exp(usize, usize),

    Gt(usize, usize),   // GT R1, R2
    Lt(usize, usize),   // LT R1, R2
    Gte(usize, usize),  // GTE R1, R2
    Lte(usize, usize),  // LTE R1, R2
    Eq(usize, usize),   // EQ R1, R2
    Ne(usize, usize),   // NE R1, R2
    
    And(usize, usize),  // AND R1, R2
    Or(usize, usize),   // OR R1, R2
    Xor(usize, usize),  // XOR R1, R2
    Not(usize),         // NOT R1

    LoadLiteral(usize, i32), // LOADLITERAL R1, value

    DeclareVar(usize, String), // DECLAREVAR "var_name", value
    DeclareMutVar(usize, String), // DECLAREMUTVAR "var_name", value
    LoadVar(usize, String),    // LOADVAR "var_name"
    StoreVar(usize, String),   // STOREVAR "var_name"

    DeclareFunc(String, Vec<String>, Vec<Instruction>), // DECLAREFUNC "func_name", num_args, [instructions]
    CallFunc(String, Vec<usize>), // CALLFUNC "func_name", [args as register indices]
    RetFunc(Vec<usize>), // RETFUNC

    If(usize, Vec<Instruction>), // IF R1, [instructions]
    ElseIf(usize, Vec<Instruction>), // ELSEIF R1, [instructions]
    Else(Vec<Instruction>), // ELSE [instructions]

    While(Vec<Instruction>, Vec<Instruction>), // WHILE [condition_instructions], [instructions]
    BreakWhile,
}

impl Instruction {
    fn register_operation(&self, vm: &mut crate::vm::VM, r1_index: usize, r2_index: usize, fnc: Box<dyn Fn(i32, i32) -> i32>) {
        let registers = vm.registers.as_mut().unwrap();
        let r1 = &registers[r1_index];
        let r2 = &registers[r2_index];
        let v1 = r1.get_value(&vm.memory);
        let v2 = r2.get_value(&vm.memory);
        
        if let (Some(DataType::Number(v1)), Some(DataType::Number(v2))) = (v1, v2) {
            r1.set_value(&mut vm.memory, DataType::Number(fnc(v1, v2)));
        } else {
            panic!("Error: Cannot perform arithmetic on non-numeric values.");
        }
    }

    fn truthy_check(&self, value: DataType) -> bool {
        let truthy = match value {
            DataType::Number(n) => n != 0,
            // TODO: Explicitly return false for future null datatype
            _ => true, // This WILL break if we add null
        };
        truthy
    }

    fn truthy_check_reg(&self, vm: &mut crate::vm::VM, reg_index: usize) -> bool {
        let value = vm.get_register_value(reg_index);
        self.truthy_check(value)
    }

    pub fn execute(&self, vm: &mut crate::vm::VM, program: Vec<Instruction>) -> Option<Vec<i32>> {
        match self {
            Instruction::Halt => { vm.running = false; None },
            Instruction::Out(r) => {
                let value = vm.get_register_value(*r);
                println!("{}", value);
                None
            },
            Instruction::Debug(r) => {
                let value = vm.get_register_value(*r);
                let address = vm.get_register_address(*r);
                println!("{:?} at mem[{}]", value, address);
                None
            },

            Instruction::Add(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| a + b)); None },
            Instruction::Sub(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| a - b)); None },
            Instruction::Mul(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| a * b)); None },
            Instruction::Div(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| a / b)); None },
            Instruction::Mod(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| a % b)); None },
            Instruction::Exp(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| a.pow(b as u32))); None },

            Instruction::Gt(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| if a > b { 1 } else { 0 })); None },
            Instruction::Lt(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| if a < b { 1 } else { 0 })); None },
            Instruction::Gte(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| if a >= b { 1 } else { 0 })); None },
            Instruction::Lte(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| if a <= b { 1 } else { 0 })); None },
            Instruction::Eq(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| if a == b { 1 } else { 0 })); None },
            Instruction::Ne(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| if a != b { 1 } else { 0 })); None },

            Instruction::And(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| if a != 0 && b != 0 { 1 } else { 0 })); None },
            Instruction::Or(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| if a != 0 || b != 0 { 1 } else { 0 })); None },
            Instruction::Xor(r1, r2) => { self.register_operation(vm, *r1, *r2, Box::new(|a, b| if a != b { 1 } else { 0 })); None },
            Instruction::Not(r) => { self.register_operation(vm, *r, 0, Box::new(|a, _| if a == 0 { 1 } else { 0 })); None },

            Instruction::LoadLiteral(r, value) => {
                let address = vm.add_to_memory(DataType::Number(*value));
                vm.registers.as_mut().unwrap()[*r].address = address;
                None
            },

            Instruction::DeclareVar(source_register, var_name) => {
                let value = vm.registers.as_ref().unwrap()[*source_register].get_value(&vm.memory).unwrap();
                vm.declare_variable(var_name.clone(), value, false);
                None
            },
            Instruction::DeclareMutVar(source_register, var_name) => {
                let value = vm.registers.as_ref().unwrap()[*source_register].get_value(&vm.memory).unwrap();
                vm.declare_variable(var_name.clone(), value, true);
                None
            },
            Instruction::LoadVar(target_register, var_name) => {
                let address = vm.get_variable_address(var_name).unwrap();
                vm.registers.as_mut().unwrap()[*target_register].address = address;
                None
            },
            Instruction::StoreVar(source_register, var_name) => {
                // Get the address that the register points to and set the variable to point to that address.
                let address = vm.registers.as_ref().unwrap()[*source_register].address;
                vm.set_variable_address(var_name, address);
                None
            },

            Instruction::DeclareFunc(func_name, params, instructions) => {
                vm.declare_function(func_name.clone(), params.clone(), instructions.clone());
                None
            },

            Instruction::CallFunc(func_name, args) => {
                vm.call_function(func_name, args.to_vec());
                None
            },

            Instruction::RetFunc(register_indices) => {
                // TODO: Set vm.running to false right before returning
                let return_addresses = register_indices.iter().map(|i| vm.registers.as_ref().unwrap()[*i].address as i32).collect::<Vec<i32>>();
                Some(return_addresses)
            },

            Instruction::If(condition_reg, instructions) => {
                if self.truthy_check_reg(vm, *condition_reg) {
                    println!("If statement met");
                    let old_pc = vm.pc;
                    
                    vm.pc = 0;
                    vm.push_scope();
                    vm.execute(instructions.clone());
                    vm.pop_scope();
                    
                    vm.pc = old_pc;
                    vm.state.if_statement_met = true;
                } else {
                    vm.state.if_statement_met = false;
                }
                None
            },
            Instruction::ElseIf(condition_reg, instructions) => {
                if !vm.state.if_statement_met && self.truthy_check_reg(vm, *condition_reg) {
                    let old_pc = vm.pc;
                    
                    vm.pc = 0;
                    vm.push_scope();
                    vm.execute(instructions.clone());
                    vm.pop_scope();
                    
                    vm.pc = old_pc;
                    vm.state.if_statement_met = true;
                }
                None
            },
            Instruction::Else(instructions) => {
                if !vm.state.if_statement_met {
                    let old_pc = vm.pc;

                    vm.pc = 0;
                    vm.push_scope();
                    vm.execute(instructions.clone());
                    vm.pop_scope();

                    vm.pc = old_pc;
                }
                None
            },

            Instruction::While(condition_instructions, instructions) => {
                // TODO: Do not use a register, as it could be modified in the loop.
                // Use a Vec of instructions, the last of which is a return statement.
                // Then check if the result of that program is truthy.
                fn get_condition_result(vm: &mut VM, instr: Vec<Instruction>) -> DataType {
                    let mut final_result = None;
                    for i in instr {
                        if let Some(result) = i.execute(vm, vec![]) {
                            let reg = result[0];
                            let value = vm.get_register_value(reg as usize);
                            final_result = Some(value);
                            break;
                        }
                    }
                    final_result.unwrap()
                }

                while self.truthy_check(get_condition_result(vm, condition_instructions.to_vec())) {
                    let old_pc = vm.pc;

                    vm.pc = 0;
                    vm.push_scope();
                    vm.execute(instructions.clone());
                    vm.pop_scope();

                    vm.pc = old_pc;
                }
                None
            },
            Instruction::BreakWhile => {
                // Set the program counter to the end of the while loop.
                // Since the loop executes the instructions as a separate program, the program counter will be set to the end of the loop.
                vm.pc = program.len();
                None
            },
        }
    }
}
