// src/instruction.rs
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Instruction {
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
    Shl(usize, usize),  // SHL R1, R2 (R1 << R2)
    Shr(usize, usize),  // SHR R1, R2 (R1 >> R2)

    Mov(usize, usize),
    Jmp(usize),
    Jz(usize, usize),
    Jnz(usize, usize),
    Cmp(usize, usize),
    Halt,
    Nop,
    Out(usize),

    Jg(usize, usize),   // JG R1, LABEL
    Jl(usize, usize),   // JL R1, LABEL
    Je(usize, usize),   // JE R1, LABEL
    Jne(usize, usize),  // JNE R1, LABEL

    DeclareVar(String, i32), // DECLAREVAR "var_name", value
    LoadVar(usize, String),    // LOADVAR "var_name"
    StoreVar(usize, String),   // STOREVAR "var_name"

    DeclareFunc(String, Vec<String>, Vec<Instruction>), // DECLAREFUNC "func_name", num_args, [instructions]
    CallFunc(String, Vec<String>), // CALLFUNC "func_name", [args]
    RetFunc(Vec<usize>), // RETFUNC
}

impl Instruction {
    // execute() is a method that takes a mutable reference to a VM instance and returns nothing or an i32
    pub fn execute(&self, vm: &mut crate::vm::VM) -> Option<Vec<i32>> {
        match self {
            Instruction::Add(r1, r2) => { vm.registers[*r1] += vm.registers[*r2]; None } ,
            Instruction::Sub(r1, r2) => { vm.registers[*r1] -= vm.registers[*r2]; None },
            Instruction::Mul(r1, r2) => { vm.registers[*r1] *= vm.registers[*r2]; None },
            Instruction::Div(r1, r2) => { vm.registers[*r1] /= vm.registers[*r2]; None },
            Instruction::Mod(r1, r2) => { vm.registers[*r1] %= vm.registers[*r2]; None },
            Instruction::Exp(r1, r2) => { vm.registers[*r1] = vm.registers[*r1].pow(vm.registers[*r2] as u32); None },

            Instruction::Gt(r1, r2) => { vm.registers[*r1] = if vm.registers[*r1] > vm.registers[*r2] { 1 } else { 0 }; None },
            Instruction::Lt(r1, r2) => { vm.registers[*r1] = if vm.registers[*r1] < vm.registers[*r2] { 1 } else { 0 }; None },
            Instruction::Gte(r1, r2) => { vm.registers[*r1] = if vm.registers[*r1] >= vm.registers[*r2] { 1 } else { 0 }; None },
            Instruction::Lte(r1, r2) => { vm.registers[*r1] = if vm.registers[*r1] <= vm.registers[*r2] { 1 } else { 0 }; None },
            Instruction::Eq(r1, r2) => { vm.registers[*r1] = if vm.registers[*r1] == vm.registers[*r2] { 1 } else { 0 }; None },
            Instruction::Ne(r1, r2) => { vm.registers[*r1] = if vm.registers[*r1] != vm.registers[*r2] { 1 } else { 0 }; None },

            Instruction::And(r1, r2) => { vm.registers[*r1] = if vm.registers[*r1] != 0 && vm.registers[*r2] != 0 { 1 } else { 0 }; None },
            Instruction::Or(r1, r2) => { vm.registers[*r1] = if vm.registers[*r1] != 0 || vm.registers[*r2] != 0 { 1 } else { 0 }; None },
            Instruction::Xor(r1, r2) => { vm.registers[*r1] = if vm.registers[*r1] != vm.registers[*r2] { 1 } else { 0 }; None },
            Instruction::Not(r) => { vm.registers[*r] = if vm.registers[*r] == 0 { 1 } else { 0 }; None },
            Instruction::Shl(r1, r2) => { vm.registers[*r1] <<= vm.registers[*r2]; None },

            Instruction::Shr(r1, r2) => { vm.registers[*r1] >>= vm.registers[*r2]; None },
            Instruction::Mov(r1, r2) => { vm.registers[*r1] = vm.registers[*r2]; None },
            Instruction::Jmp(addr) => { vm.pc = *addr; None },
            Instruction::Jz(r, addr) => { if vm.registers[*r] == 0 { vm.pc = *addr }; None },
            Instruction::Jnz(r, addr) => { if vm.registers[*r] != 0 { vm.pc = *addr }; None },
            Instruction::Cmp(_r1, _r2) => { /* Placeholder for future flags */ None },
            Instruction::Out(r) => { println!("OUT: {}", vm.registers[*r]); None },
            Instruction::Nop => { /* No operation */ None },
            Instruction::Halt => { vm.running = false; None },

            Instruction::Jg(r, addr) => { if vm.registers[*r] > 0 { vm.pc = *addr }; None },
            Instruction::Jl(r, addr) => { if vm.registers[*r] < 0 { vm.pc = *addr }; None },
            Instruction::Je(r, addr) => { if vm.registers[*r] == 0 { vm.pc = *addr }; None },
            Instruction::Jne(r, addr) => { if vm.registers[*r] != 0 { vm.pc = *addr }; None },


            Instruction::DeclareVar(var_name, value) => {
                vm.declare_variable(var_name.clone(), *value);
                None
            },
            Instruction::LoadVar(target_register, var_name) => {
                vm.registers[*target_register] = vm.get_variable(var_name).unwrap_or(0);
                None
            },
            Instruction::StoreVar(source_register, var_name) => {
                vm.set_variable(var_name, vm.registers[*source_register]);
                None
            },

            Instruction::DeclareFunc(func_name, params, instructions) => {
                vm.declare_function(func_name.clone(), params.clone(), instructions.clone());
                None
            },

            Instruction::CallFunc(func_name, args) => {
                vm.call_function(func_name, args.clone());
                None
            },

            Instruction::RetFunc(register_indices) => {
                let return_values: Vec<i32> = register_indices.iter().map(|&i| vm.registers[i]).collect();
                Some(return_values)
            },
        }
    }
}
