// src/vm.rs
pub struct VM {
    pub registers: [i32; 8], // Public to allow Instruction to access it
    pub memory: [i32; 256],
    pub pc: usize,
    pub running: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 8],
            memory: [0; 256],
            pc: 0,
            running: true,
        }
    }

    pub fn execute(&mut self, program: Vec<crate::instruction::Instruction>) {
        while self.running && self.pc < program.len() {
            let instruction = &program[self.pc];
            self.pc += 1;
            instruction.execute(self);
        }
    }
}
