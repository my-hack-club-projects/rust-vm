use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Function {
    params: Vec<String>,
    instructions: Vec<crate::instruction::Instruction>,
    captured_scope: Scope,
}

#[derive(Clone, Debug)]
pub struct Scope {
    variables: HashMap<String, usize>,
    functions: HashMap<String, Function>,
}

impl Default for Scope {
    fn default() -> Self {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}

pub struct VM {
    pub pc: usize,
    pub running: bool,

    pub memory: Vec<i32>,
    pub registers: [i32; 8],
    pub scopes: Vec<Scope>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            pc: 0,
            running: true,
            memory: vec![0; 1024],
            registers: [0; 8],
            scopes: vec![Scope::default()],
        }
    }

    pub fn execute(&mut self, program: Vec<crate::instruction::Instruction>) -> Vec<i32> {
        let mut output = Vec::new();
        while self.running && self.pc < program.len() {
            let instruction = &program[self.pc];
            self.pc += 1;
            let result = instruction.execute(self);

            if let Some(result) = result {
                output = result;
            }
        }
        output
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare_variable(&mut self, name: String, value: i32) {
        if let Some(current_scope) = self.scopes.last_mut() {
            let address = self.memory.iter().position(|&x| x == 0).unwrap_or(0);
            self.memory[address] = value; // Assign the value to memory
            current_scope.variables.insert(name, address); // Store the variable name and its address
        }
    }

    pub fn declare_variable_from_memory(&mut self, name: String, address: usize) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.variables.insert(name, address);
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<i32> {
        for scope in self.scopes.iter().rev() {
            if let Some(&address) = scope.variables.get(name) {
                return Some(self.memory[address]);
            }
        }

        eprintln!("Error: Variable '{}' not found.", name);
        None
    }

    pub fn get_variable_address(&self, name: &str) -> Option<usize> {
        for scope in self.scopes.iter().rev() {
            if let Some(&address) = scope.variables.get(name) {
                return Some(address);
            }
        }

        eprintln!("Error: Variable '{}' not found.", name);
        None
    }

    pub fn set_variable(&mut self, name: &str, value: i32) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(&address) = scope.variables.get(name) {
                self.memory[address] = value;
                return;
            }
        }
        eprintln!("Error: Variable '{}' not found.", name);
    }

    pub fn declare_function(&mut self, name: String, params: Vec<String>, instructions: Vec<crate::instruction::Instruction>) {
        let joined_scopes = self.scopes.iter().rev().cloned().fold(Scope::default(), |mut acc, scope| {
            for (key, value) in scope.variables {
                acc.variables.insert(key, value);
            }
            for (key, value) in scope.functions {
                acc.functions.insert(key, value);
            }
            acc
        });
        
        if let Some(current_scope) = self.scopes.last_mut() {
            let function = Function {
                params: params.clone(),
                instructions: instructions.clone(),
                captured_scope: joined_scopes,
            };

            current_scope.functions.insert(name, function);
        }
    }

    pub fn call_function(&mut self, name: &str, args: Vec<String>) {
        // Find the function definition immutably
        let (params, instructions, captured_scope) = {
            let mut found = None;
            for scope in self.scopes.iter().rev() {
                if let Some(function) = scope.functions.get(name) {
                    found = Some((function.params.clone(), function.instructions.clone(), function.captured_scope.clone()));
                    break;
                }
            }
            if let Some(found) = found {
                found
            } else {
                eprintln!("Error: Function '{}' not found.", name);
                return;
            }
        };
    
        // Check the number of arguments
        if args.len() != params.len() {
            eprintln!("Error: Function '{}' expects {} arguments, but {} were provided.", name, params.len(), args.len());
            return;
        }
    
        let addresses: Vec<usize> = args.iter().map(|arg| self.get_variable_address(arg).unwrap()).collect();
    
        // Mutable operations
        let old_scopes = self.scopes.clone();
        let return_address = self.pc;

        self.scopes = vec![captured_scope];
        self.pc = 0;
    
        // Create the parameter variables that point to the addresses of the arguments
        for (param, &address) in params.iter().zip(addresses.iter()) {
            self.declare_variable_from_memory(param.clone(), address);
        }
    
        // Execute the function
        let return_values = self.execute(instructions);

        // Restore the previous state
        self.pc = return_address;
        self.scopes = old_scopes;

        // Set the return values to the registers
        for (i, value) in return_values.iter().enumerate() {
            self.registers[i] = *value;
        }
    }
}