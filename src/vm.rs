use std::collections::HashMap;

pub struct VM {
    pub registers: [i32; 8],  // Public to allow Instruction to access it
    pub memory: [i32; 256],
    pub variables: HashMap<String, usize>,
    pub functions: HashMap<String, (usize, Vec<crate::instruction::Instruction>)>,
    pub pc: usize,
    pub sp: usize,            // Stack pointer
    pub running: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            registers: [0; 8],
            memory: [0; 256],
            variables: HashMap::new(),
            functions: HashMap::new(),
            pc: 0,
            sp: 255,          // Initialize stack pointer at the end of memory
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

    pub fn declare_variable(&mut self, name: String, value: i32) {
        let address = self.memory.iter().position(|&x| x == 0).unwrap_or(0); // Find first empty spot
        self.memory[address] = value; // Assign the value to memory
        self.variables.insert(name, address); // Store the variable name and its address
    }

    pub fn get_variable(&self, name: &str) -> Option<i32> {
        if let Some(&address) = self.variables.get(name) {
            Some(self.memory[address])
        } else {
            None
        }
    }

    pub fn set_variable(&mut self, name: &str, value: i32) {
        if let Some(&address) = self.variables.get(name) {
            self.memory[address] = value;
        }
    }
}