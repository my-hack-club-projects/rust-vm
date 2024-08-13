// src/instruction.rs
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

    Load(usize, usize),
    Store(usize, usize),
    Mov(usize, usize),
    Jmp(usize),
    Jz(usize, usize),
    Jnz(usize, usize),
    Cmp(usize, usize),
    Halt,
    Nop,
    Out(usize),

    Push(usize),        // PUSH R1
    Pop(usize),         // POP R1

    Jg(usize, usize),   // JG R1, LABEL
    Jl(usize, usize),   // JL R1, LABEL
    Je(usize, usize),   // JE R1, LABEL
    Jne(usize, usize),  // JNE R1, LABEL

    DeclareVar(String, i32), // DECLAREVAR "var_name", value
    LoadVar(usize, String),    // LOADVAR "var_name"
    StoreVar(usize, String),   // STOREVAR "var_name"
}

impl Instruction {
    pub fn execute(&self, vm: &mut crate::vm::VM) {
        match self {
            Instruction::Add(r1, r2) => vm.registers[*r1] += vm.registers[*r2],
            Instruction::Sub(r1, r2) => vm.registers[*r1] -= vm.registers[*r2],
            Instruction::Mul(r1, r2) => vm.registers[*r1] *= vm.registers[*r2],
            Instruction::Div(r1, r2) => vm.registers[*r1] /= vm.registers[*r2],
            Instruction::Mod(r1, r2) => vm.registers[*r1] %= vm.registers[*r2],
            Instruction::Exp(r1, r2) => vm.registers[*r1] = vm.registers[*r1].pow(vm.registers[*r2] as u32),
            
            Instruction::Gt(r1, r2) => vm.registers[*r1] = if vm.registers[*r1] > vm.registers[*r2] { 1 } else { 0 },
            Instruction::Lt(r1, r2) => vm.registers[*r1] = if vm.registers[*r1] < vm.registers[*r2] { 1 } else { 0 },
            Instruction::Gte(r1, r2) => vm.registers[*r1] = if vm.registers[*r1] >= vm.registers[*r2] { 1 } else { 0 },
            Instruction::Lte(r1, r2) => vm.registers[*r1] = if vm.registers[*r1] <= vm.registers[*r2] { 1 } else { 0 },
            Instruction::Eq(r1, r2) => vm.registers[*r1] = if vm.registers[*r1] == vm.registers[*r2] { 1 } else { 0 },
            Instruction::Ne(r1, r2) => vm.registers[*r1] = if vm.registers[*r1] != vm.registers[*r2] { 1 } else { 0 },

            Instruction::And(r1, r2) => vm.registers[*r1] = if vm.registers[*r1] != 0 && vm.registers[*r2] != 0 { 1 } else { 0 },
            Instruction::Or(r1, r2) => vm.registers[*r1] = if vm.registers[*r1] != 0 || vm.registers[*r2] != 0 { 1 } else { 0 },
            Instruction::Xor(r1, r2) => vm.registers[*r1] = if vm.registers[*r1] != vm.registers[*r2] { 1 } else { 0 },
            Instruction::Not(r) => vm.registers[*r] = if vm.registers[*r] == 0 { 1 } else { 0 },
            Instruction::Shl(r1, r2) => vm.registers[*r1] <<= vm.registers[*r2],
            Instruction::Shr(r1, r2) => vm.registers[*r1] >>= vm.registers[*r2],

            Instruction::Load(r, mem) => vm.registers[*r] = vm.memory[*mem],
            Instruction::Store(r, mem) => vm.memory[*mem] = vm.registers[*r],
            Instruction::Mov(r1, r2) => vm.registers[*r1] = vm.registers[*r2],
            Instruction::Jmp(addr) => vm.pc = *addr,
            Instruction::Jz(r, addr) => if vm.registers[*r] == 0 { vm.pc = *addr },
            Instruction::Jnz(r, addr) => if vm.registers[*r] != 0 { vm.pc = *addr },
            Instruction::Cmp(_r1, _r2) => { /* Placeholder for future flags */ },
            Instruction::Out(r) => println!("OUT: {}", vm.registers[*r]),
            Instruction::Nop => { /* No operation */ },
            Instruction::Halt => vm.running = false,

            Instruction::Push(r) => {
                vm.memory[vm.sp] = vm.registers[*r];
                vm.sp -= 1;
            }
            Instruction::Pop(r) => {
                vm.sp += 1;
                vm.registers[*r] = vm.memory[vm.sp];
            }

            Instruction::Jg(r, addr) => if vm.registers[*r] > 0 { vm.pc = *addr },
            Instruction::Jl(r, addr) => if vm.registers[*r] < 0 { vm.pc = *addr },
            Instruction::Je(r, addr) => if vm.registers[*r] == 0 { vm.pc = *addr },
            Instruction::Jne(r, addr) => if vm.registers[*r] != 0 { vm.pc = *addr },

            Instruction::DeclareVar(var_name, value) => {
                vm.declare_variable(var_name.clone(), *value);
            },
            Instruction::LoadVar(target_register, var_name) => {
                if let Some(value) = vm.get_variable(var_name) {
                    vm.registers[*target_register] = value; // Load variable into specified register
                } else {
                    eprintln!("Error: Variable '{}' not found.", var_name);
                }
            },
            Instruction::StoreVar(source_register, var_name) => {
                if let Some(_) = vm.variables.get(var_name) {
                    vm.set_variable(var_name, vm.registers[*source_register]); // Store specified register into variable
                } else {
                    eprintln!("Error: Variable '{}' not found.", var_name);
                }
            },
        }
    }
}
