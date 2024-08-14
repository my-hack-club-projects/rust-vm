use crate::symbol::DataType;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Instruction {
    Halt,
    Out(usize),

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

    pub fn execute(&self, vm: &mut crate::vm::VM) -> Option<Vec<i32>> {
        match self {
            Instruction::Halt => { vm.running = false; None },
            Instruction::Out(r) => { println!("{}", vm.get_register_value(*r)); None },

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
                let return_addresses = register_indices.iter().map(|i| vm.registers.as_ref().unwrap()[*i].address as i32).collect::<Vec<i32>>();
                Some(return_addresses)
            },
        }
    }
}
