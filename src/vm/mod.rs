use core::panic;
use std::rc::Rc;
use std::cell::RefCell;

pub mod instruction;
pub mod symbol;

use symbol::{Register, Scope, Symbol, DataType};

pub struct VMState {
    pub if_statement_met: bool,
    pub loop_break: bool,
}

impl VMState {
    pub fn new() -> Self {
        VMState {
            if_statement_met: false,
            loop_break: false,
        }
    }
}

pub struct VM {
    pub pc: usize,
    pub running: bool,

    pub state: VMState,

    pub memory: Vec<Rc<RefCell<DataType>>>,
    pub registers: Option<Vec<Register>>,
    pub scopes: Vec<Scope>,
}

const MEM_SIZE: usize = 32; //1024;

impl VM {
    pub fn new() -> Self {
        let mut vm_memory = Vec::with_capacity(MEM_SIZE);
        for _ in 0..MEM_SIZE {
            vm_memory.push(Rc::new(RefCell::new(DataType::Null())));
        }

        let mut vm = VM {
            pc: 0,
            running: true,

            state: VMState::new(),

            memory: vm_memory,
            registers: None,
            scopes: vec![Scope::new(None)], // The first scope has no parent, but how do we represent that? None does not work.
        };
    
        // Initialize the registers to point to the last memory address
        let mut registers = Vec::with_capacity(8);
        for _ in 0..8 {
            registers.push(Register::new(vm.memory[MEM_SIZE - 1].clone()));
        }
        vm.registers = Some(registers);
        
    
        vm
    }

    pub fn execute(&mut self, program: Vec<instruction::Instruction>) -> Vec<Rc<RefCell<DataType>>> {
        let mut output = Vec::new();
        while self.running && self.pc < program.len() {
            let instruction = &program[self.pc];
            self.pc += 1;
            let result = instruction.execute(self, program.clone());

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

    fn get_free_address_index(&self) -> usize {
        self.memory.iter().position(|data| Rc::strong_count(data) == 1).unwrap()
    }

    fn get_free_address(&self) -> Rc<RefCell<DataType>> {
        self.memory[self.get_free_address_index()].clone()
    }

    pub fn add_to_memory(&mut self, data: DataType) -> Rc<RefCell<DataType>> {
        let address = self.get_free_address_index();
        if address == MEM_SIZE - 1 {
            panic!("Error: Memory full.");
        }
        
        let new_data = Rc::new(RefCell::new(data));
        self.memory[address] = new_data.clone();
    
        self.memory[address].clone()
    }

    pub fn get_from_memory(&self, address: usize) -> Rc<RefCell<DataType>> {
        if address < MEM_SIZE {
            self.memory[address].clone()
        } else {
            panic!("Error: Memory address out of bounds.");
        }
    }

    pub fn get_or_add_to_memory(&mut self, data: DataType) -> Rc<RefCell<DataType>> {
        if let Some(address) = self.memory.iter().position(|d| *d.borrow() == data) {
            self.memory[address].clone()
        } else {
            self.add_to_memory(data)
        }
    }

    pub fn get_register_value(&self, register: usize) -> DataType {
        if let Some(registers) = &self.registers {
            Rc::clone(&registers[register].address).borrow().clone()
        } else {
            panic!("Registers not initialized.");
        }
    }

    pub fn get_register_address(&self, register: usize) -> Rc<RefCell<DataType>> {
        if let Some(registers) = &self.registers {
            registers[register].address.clone()
        } else {
            panic!("Registers not initialized.");
        }
    }

    pub fn get_register_address_index(&self, register: usize) -> usize {
        if let Some(registers) = &self.registers {
            self.memory.iter().position(|data| Rc::ptr_eq(data, &registers[register].address)).unwrap()
        } else {
            panic!("Registers not initialized.");
        }
    }

    pub fn declare_variable(&mut self, name: String, value: DataType, mutable: bool) {
        let address = self.get_or_add_to_memory(value);

        self.declare_variable_from_memory(name, address, mutable);
    }

    pub fn declare_variable_from_memory(&mut self, name: String, address: Rc<RefCell<DataType>>, mutable: bool) {
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
            Some(Rc::clone(&symbol.address).borrow().clone())
        } else {
            None
        }
    }

    pub fn get_variable_address(&self, name: &str) -> Option<Rc<RefCell<DataType>>> {
        if let Some(symbol) = self.get_variable_base(name) {
            Some(symbol.address.clone())
        } else {
            None
        }
    }

    pub fn set_variable_address(&mut self, name: &str, address: Rc<RefCell<DataType>>) {
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

    pub fn declare_function(&mut self, name: String, params: Vec<String>, instructions: Vec<instruction::Instruction>) {
        let function_address_index = self.get_free_address_index();
        let function_symbol = Symbol {
            name: name.clone(),
            address: self.memory[function_address_index].clone(),
            mutable: false,
        };
        let joined_scopes = self.scopes.iter().rev().cloned().fold(Scope::new(None), |mut acc, scope| {
            for (name, symbol) in scope.symbols.iter() {
                acc.symbols.insert(name.clone(), symbol.clone());
            }
            acc.symbols.insert(name.clone(), function_symbol.clone()); // Add the function that's being declared
            acc
        });

        if let Some(current_scope) = self.scopes.last_mut() {
            let function = DataType::Function(params.clone(), instructions.clone(), joined_scopes);
            let function_address = Rc::new(RefCell::new(function));
            self.memory[function_address_index] = function_address.clone(); // This increments the reference count
            
            current_scope.symbols.insert(name, function_symbol);
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
        let addresses = args_indices.iter().map(|&i| self.get_register_address(i)).collect::<Vec<Rc<RefCell<DataType>>>>();

        // Mutable operations
        let old_scopes = self.scopes.clone();
        let return_address = self.pc;

        self.scopes = vec![captured_scope];
        self.pc = 0;
    
        // Declare the parameter variables with the values
        for (param, address) in params.iter().zip(addresses.iter()) {
            self.declare_variable_from_memory(param.clone(), address.clone(), false);
        }

        // Execute the function
        let return_addresses = self.execute(instructions);

        // Restore the previous state
        self.pc = return_address;
        self.scopes = old_scopes;

        // Set the return values to the registers
        for (i, value) in return_addresses.iter().enumerate() {
            self.registers.as_mut().unwrap()[i].address = value.clone();
        }
    }
}