use crate::symbol::{Scope, Symbol, DataType};

pub struct VM {
    pub pc: usize,
    pub running: bool,

    pub memory: Vec<DataType>,
    pub registers: [i32; 8],
    pub scopes: Vec<Scope>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            pc: 0,
            running: true,
            memory: vec![DataType::Number(0); 1024],
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

    fn get_free_address(&self) -> usize {
        self.memory.iter().position(|data| *data == DataType::Number(0)).unwrap()
    }

    pub fn declare_variable(&mut self, name: String, value: i32) {
        let address = self.get_free_address();
        self.memory[address] = DataType::Number(value);

        self.declare_variable_from_memory(name, address)
    }

    pub fn declare_variable_from_memory(&mut self, name: String, address: usize) {
        if let Some(current_scope) = self.scopes.last_mut() {
            let symbol = Symbol {
                name: name.clone(),
                address,
            };
            
            current_scope.symbols.insert(name, symbol);
        }
    }

    fn get_variable_base(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol);
            }
        }

        panic!("Error: Variable '{}' not found.", name);
        None
    }

    pub fn get_variable(&self, name: &str) -> Option<DataType> {
        if let Some(symbol) = self.get_variable_base(name) {
            Some(self.memory[symbol.address].clone())
        } else {
            None
        }
    }

    pub fn get_variable_address(&self, name: &str) -> Option<usize> {
        if let Some(symbol) = self.get_variable_base(name) {
            Some(symbol.address)
        } else {
            None
        }
    }

    pub fn set_variable(&mut self, name: &str, value: i32) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                self.memory[symbol.address] = DataType::Number(value);
                return;
            }
        }
        panic!("Error: Variable '{}' not found.", name);
    }

    pub fn declare_function(&mut self, name: String, params: Vec<String>, instructions: Vec<crate::instruction::Instruction>) {
        let function_address = self.get_free_address();
        let function = Symbol {
            name: name.clone(),
            address: function_address,
        };
        let joined_scopes = self.scopes.iter().rev().cloned().fold(Scope::default(), |mut acc, scope| {
            for (name, symbol) in scope.symbols.iter() {
                acc.symbols.insert(name.clone(), symbol.clone());
            }
            acc.symbols.insert(name.clone(), function.clone()); // Add the function that's being declared
            acc
        });

        if let Some(current_scope) = self.scopes.last_mut() {
            self.memory[function_address] = DataType::Function(params.clone(), instructions.clone(), joined_scopes);
            
            current_scope.symbols.insert(name, function);
        }
    }

    pub fn call_function(&mut self, name: &str, args_indices: Vec<usize>) {
        // Find the function definition immutably (use self.get_variable)
        let (params, instructions, captured_scope) = {
            let function = self.get_variable(name).unwrap();
            if let DataType::Function(params, instructions, captured_scope) = function {
                (params, instructions, captured_scope)
            } else {
                panic!("Error: '{}' is not a function.", name);
            }
        };
    
        // Check the number of arguments
        if args_indices.len() != params.len() {
            panic!("Error: Function '{}' expects {} arguments, but {} were provided.", name, params.len(), args_indices.len());
        }
    
        // Get the values from the registers at the argument indices
        let values = args_indices.iter().map(|&i| self.registers[i as usize]).collect::<Vec<i32>>();
    
        // Mutable operations
        let old_scopes = self.scopes.clone();
        let return_address = self.pc;

        self.scopes = vec![captured_scope];
        self.pc = 0;
    
        // Declare the parameter variables with the values
        for (param, &value) in params.iter().zip(values.iter()) {
            self.declare_variable(param.clone(), value);
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