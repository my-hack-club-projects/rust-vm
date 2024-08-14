use core::panic;

use crate::symbol::{Register, Scope, Symbol, DataType};

pub struct VMState {
    pub if_statement_met: bool,
}

impl VMState {
    pub fn new() -> Self {
        VMState {
            if_statement_met: false,
        }
    }
}

pub struct VM {
    pub pc: usize,
    pub running: bool,

    pub state: VMState,

    pub memory: Vec<DataType>,
    pub registers: Option<[Register; 8]>,
    pub scopes: Vec<Scope>,
}

impl VM {
    pub fn new() -> Self {
        let vm_memory = vec![DataType::Number(0); 1024]; // TODO: use null instead of the number 0

        let mut vm = VM {
            pc: 0,
            running: true,

            state: VMState::new(),

            memory: vm_memory,
            registers: None, // Temporary placeholder
            scopes: vec![Scope::new(None)], // The first scope has no parent, but how do we represent that? None does not work.
        };
    
        // Initialize the registers
        vm.registers = Some([Register::new(0); 8]);
    
        vm
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
        self.scopes.push(Scope::new(Some(self.scopes.last().unwrap().clone())));
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn get_free_address(&self) -> usize {
        self.memory.iter().position(|data| *data == DataType::Number(0)).unwrap()
    }

    pub fn add_to_memory(&mut self, data: DataType) -> usize {
        let address = self.get_free_address();
        self.memory[address] = data;
        address
    }

    pub fn get_register_value(&self, register: usize) -> DataType {
        if let Some(registers) = &self.registers {
            registers[register].get_value(&self.memory).unwrap()
        } else {
            panic!("Registers not initialized.");
        }
    }

    pub fn get_register_address(&self, register: usize) -> usize {
        if let Some(registers) = &self.registers {
            registers[register].address
        } else {
            panic!("Registers not initialized.");
        }
    }

    pub fn declare_variable(&mut self, name: String, value: DataType, mutable: bool) {
        let address = self.get_free_address();
        self.memory[address] = value;

        self.declare_variable_from_memory(name, address, mutable);
    }

    pub fn declare_variable_from_memory(&mut self, name: String, address: usize, mutable: bool) {
        if let Some(current_scope) = self.scopes.last_mut() {
            // Check if the variable is already declared
            if current_scope.get_all_symbols().contains_key(&name) {
                panic!("Error: Variable '{}' already declared in this scope.", name);
            }

            let symbol = Symbol {
                name: name.clone(),
                address,
                mutable,
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

    pub fn set_variable(&mut self, name: &str, value: DataType) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                if !symbol.mutable {
                    panic!("Error: Variable '{}' is not mutable.", name);
                }

                self.memory[symbol.address] = value;
                return;
            }
        }
        panic!("Error: Variable '{}' not found.", name);
    }

    pub fn set_variable_address(&mut self, name: &str, address: usize) {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.symbols.get_mut(name) {
                if !symbol.mutable {
                    panic!("Error: Variable '{}' is not mutable.", name);
                }
    
                symbol.address = address;
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
            mutable: false,
        };
        let joined_scopes = self.scopes.iter().rev().cloned().fold(Scope::new(None), |mut acc, scope| {
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
    
        // Get the addresses of the arguments
        let addresses = args_indices.iter().map(|&i| self.registers.as_ref().unwrap()[i].address).collect::<Vec<usize>>();

        // Mutable operations
        let old_scopes = self.scopes.clone();
        let return_address = self.pc;

        self.scopes = vec![captured_scope];
        self.pc = 0;
    
        // Declare the parameter variables with the values
        for (param, &address) in params.iter().zip(addresses.iter()) {
            self.declare_variable_from_memory(param.clone(), address, false);
        }
    
        // Execute the function
        let return_addresses = self.execute(instructions);

        // Restore the previous state
        self.pc = return_address;
        self.scopes = old_scopes;

        // Set the return values to the registers
        for (i, value) in return_addresses.iter().enumerate() {
            self.registers.as_mut().unwrap()[i].address = *value as usize;
        }
    }
}