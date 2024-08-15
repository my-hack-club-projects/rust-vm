use std::{cell::RefCell, rc::Rc};

use crate::{symbol::DataType, vm::VM};

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Instruction {
    Halt,
    Out(usize),
    Debug(usize),
    MemDump,
    RegDump,

    // Arithmetic and logical operations. First argument is the register to be modified.
    Add(usize, Vec<usize>), // ADD R1, [R2, R3, ...]
    Sub(usize, Vec<usize>), // SUB R1, [R2, R3, ...]
    Mul(usize, Vec<usize>), // MUL R1, [R2, R3, ...]
    Div(usize, Vec<usize>), // DIV R1, [R2, R3, ...]
    Mod(usize, Vec<usize>), // MOD R1, [R2, R3, ...]
    Exp(usize, Vec<usize>), // EXP R1, [R2, R3, ...]

    Gt(usize, Vec<usize>),   // GT R1, [R2, R3, ...]
    Lt(usize, Vec<usize>),   // LT R1, [R2, R3, ...]
    Gte(usize, Vec<usize>),  // GTE R1, [R2, R3, ...]
    Lte(usize, Vec<usize>),  // LTE R1, [R2, R3, ...]
    Eq(usize, Vec<usize>),   // EQ R1, [R2, R3, ...]
    Ne(usize, Vec<usize>),   // NE R1, [R2, R3, ... 

    And(usize, Vec<usize>),  // AND R1, [R2, R3, ...]
    Or(usize, Vec<usize>),   // OR R1, [R2, R3, ...]
    Xor(usize, Vec<usize>),  // XOR R1, [R2, R3, ...]
    Not(usize, Vec<usize>),  // NOT R1, [R2]

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
    ContinueWhile,
}

impl Instruction {
    fn register_operation(&self, vm: &mut crate::vm::VM, output_index: usize, compare_indices: Vec<usize>, fnc: Box<dyn Fn(i32, i32) -> i32>) {
        // let registers = vm.registers.unwrap();
        let registers = vm.registers.as_ref().unwrap();
        let r1 = &registers[compare_indices[0]];
        let r2 = &registers[compare_indices[1]];
        let v1 = r1.get_value(&vm.memory);
        let v2 = r2.get_value(&vm.memory);
        
        if let (Some(DataType::Number(v1)), Some(DataType::Number(v2))) = (v1, v2) {
            let result = DataType::Number(fnc(v1, v2));
            let address = vm.get_or_add_to_memory(result);
            vm.registers.as_mut().unwrap()[output_index].address = address;
        } else {
            panic!("Error: Cannot perform arithmetic on non-numeric values.");
        }
    }
    
    fn truthy_check(&self, value: DataType) -> bool {
        let truthy = match value {
            DataType::Number(n) => n != 0,
            DataType::Null() => false,
            _ => true, // This WILL break if we add null
        };
        truthy
    }

    fn truthy_check_reg(&self, vm: &mut crate::vm::VM, reg_index: usize) -> bool {
        let value = vm.get_register_value(reg_index);
        self.truthy_check(value)
    }

    pub fn execute(&self, vm: &mut crate::vm::VM, program: Vec<Instruction>) -> Option<Vec<Rc<RefCell<DataType>>>> {
        match self {
            Instruction::Halt => { vm.running = false; None },
            Instruction::Out(r) => {
                let value = vm.get_register_value(*r);
                println!("{}", value);
                None
            },
            Instruction::Debug(r) => {
                let value = vm.get_register_value(*r);
                let address = vm.get_register_address_index(*r);
                println!("Register {}: {:?}, mem[{}]", *r, value, address);
                None
            },
            Instruction::MemDump => {
                // println!("{:?}", vm.memory); // Need to print reference counts
                for (i, v) in vm.memory.iter().enumerate() {
                    println!("mem[{}]: {:?}", i, Rc::strong_count(v));
                }
                None
            },
            Instruction::RegDump => {
                println!("{:?}", vm.registers);
                None
            },

            Instruction::Add(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| a + b)); None },
            Instruction::Sub(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| a - b)); None },
            Instruction::Mul(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| a * b)); None },
            Instruction::Div(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| a / b)); None },
            Instruction::Mod(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| a % b)); None },
            Instruction::Exp(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| a.pow(b as u32))); None },

            Instruction::Gt(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| if a > b { 1 } else { 0 })); None },
            Instruction::Lt(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| if a < b { 1 } else { 0 })); None },
            Instruction::Gte(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| if a >= b { 1 } else { 0 })); None },
            Instruction::Lte(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| if a <= b { 1 } else { 0 })); None },
            Instruction::Eq(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| if a == b { 1 } else { 0 })); None },
            Instruction::Ne(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| if a != b { 1 } else { 0 })); None },

            Instruction::And(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| if a != 0 && b != 0 { 1 } else { 0 })); None },
            Instruction::Or(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| if a != 0 || b != 0 { 1 } else { 0 })); None },
            Instruction::Xor(out, comp) => { self.register_operation(vm, *out, comp.clone(), Box::new(|a, b| if a != b { 1 } else { 0 })); None },
            Instruction::Not(out, comp) => { self.register_operation(vm, *out, vec![comp[0], 0], Box::new(|a, _| if a == 0 { 1 } else { 0 })); None },

            Instruction::LoadLiteral(r, value) => {
                let address = vm.get_or_add_to_memory(DataType::Number(*value));
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
                let address = vm.registers.as_ref().unwrap()[*source_register].address.clone();
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
                let return_addresses = register_indices.iter().map(|i| vm.registers.as_ref().unwrap()[*i].address.clone()).collect::<Vec<Rc<RefCell<DataType>>>>();
                Some(return_addresses)
            },

            Instruction::If(condition_reg, instructions) => {
                if self.truthy_check_reg(vm, *condition_reg) {
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
                fn get_condition_result(vm: &mut VM, instr: Vec<Instruction>) -> DataType {
                    let mut final_result: Option<DataType> = None;
                    for i in instr {
                        if let Some(result) = i.execute(vm, vec![]) {
                            let value = result[0].clone();
                            final_result = Some(Rc::clone(&value).borrow().clone());
                            break;
                        }
                    }
                    
                    final_result.unwrap()
                }

                let old_pc = vm.pc;
                while self.truthy_check(get_condition_result(vm, condition_instructions.to_vec())) {

                    vm.pc = 0;
                    vm.push_scope();
                    vm.execute(instructions.clone());
                    vm.pop_scope();

                    if vm.state.loop_break {
                        vm.state.loop_break = false;
                        break;
                    }

                }
                vm.pc = old_pc;

                None
            },
            Instruction::BreakWhile => {
                println!("BreakWhile");
                // Set the program counter to the end of the while loop. Then, set a 'break' flag.
                
                vm.pc = program.len();
                vm.state.loop_break = true;

                None
            },

            Instruction::ContinueWhile => {
                println!("ContinueWhile");
                // Set the program counter to the end of the loop, skipping the rest of the instructions.
                // The program will be executed again if the condition is still true.
                vm.pc = program.len();
                None
            },
        }
    }
}
