use std::collections::HashMap;

pub struct Scope {
    variables: HashMap<String, usize>,
    functions: HashMap<String, (Vec<String>, Vec<crate::instruction::Instruction>)>,
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
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.functions.insert(name, (params, instructions));
        }
    }

    pub fn call_function(&mut self, name: &str, args: Vec<String>) {
        // Find the function definition immutably
        let (params, instructions) = {
            let mut found = None;
            for scope in self.scopes.iter().rev() {
                if let Some(&(ref params, ref instructions)) = scope.functions.get(name) {
                    found = Some((params.clone(), instructions.clone()));
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
    
        // 'args' is a list of variable names, we need to get their values
        // Get the addresses of the variables and save them to be restored later
        let addresses: Vec<usize> = args.iter().map(|arg| self.get_variable_address(arg).unwrap()).collect();
    
        // Mutable operations
        self.push_scope();
    
        // Create the parameter variables that point to the addresses of the arguments
        for (param, &address) in params.iter().zip(addresses.iter()) {
            println!("Declaring variable {} at address {}", param, address);
            self.declare_variable_from_memory(param.clone(), address); // This also assigns them the values of the arguments
        }
    
        // Execute the function
        let return_address = self.pc;
        self.pc = 0;
        let return_values = self.execute(instructions); // How do we handle the return values? 

        // Restore the previous state
        self.pc = return_address;
    
        self.pop_scope();

        // Set the return values to the registers
        for (i, value) in return_values.iter().enumerate() {
            self.registers[i] = *value;
        }
    }
}