// src/instruction.rs
#[allow(dead_code)]
pub enum Instruction {
    Add(usize, usize),       
    Sub(usize, usize),       
    Mul(usize, usize),       
    Div(usize, usize),       
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
}

impl Instruction {
    pub fn execute(&self, vm: &mut crate::vm::VM) {
        match self {
            Instruction::Add(r1, r2) => vm.registers[*r1] += vm.registers[*r2],
            Instruction::Sub(r1, r2) => vm.registers[*r1] -= vm.registers[*r2],
            Instruction::Mul(r1, r2) => vm.registers[*r1] *= vm.registers[*r2],
            Instruction::Div(r1, r2) => vm.registers[*r1] /= vm.registers[*r2],
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
        }
    }
}
