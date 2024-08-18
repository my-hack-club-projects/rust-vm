use core::panic;
use std::{rc::Rc, result};
use std::cell::RefCell;

use crate::ast::parser::ASTNode;

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

const MEM_SIZE: usize = 1024;

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

    pub fn truthy_check(&self, value: DataType) -> bool {
        match value {
            DataType::Number(n) => n != 0,
            DataType::Null() => false,
            _ => true, // This WILL break if we add null
        }
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

    pub fn add_to_memory(&mut self, data: DataType) -> Result<Rc<RefCell<DataType>>, String> {
        let address = self.get_free_address_index();
        if address == MEM_SIZE - 1 {
            return Err("Memory full.".to_string());
        }
        
        let new_data = Rc::new(RefCell::new(data));
        self.memory[address] = new_data.clone();
    
        Ok(self.memory[address].clone())
    }

    pub fn get_from_memory(&self, address: usize) -> Result<Rc<RefCell<DataType>>, String> {
        if address < MEM_SIZE {
            Ok(self.memory[address].clone())
        } else {
            Err("Memory address out of bounds.".to_string())
        }
    }

    pub fn get_or_add_to_memory(&mut self, data: DataType) -> Result<Rc<RefCell<DataType>>, String> {
        if let Some(address) = self.memory.iter().position(|d| *d.borrow() == data) {
            Ok(self.memory[address].clone())
        } else {
            self.add_to_memory(data)
        }
    }

    pub fn load_value_into_register(&mut self, register: usize, value: DataType) -> Result<(), String> {
        let result = self.get_or_add_to_memory(value);
        match result {
            Ok(address) => {
                self.registers.as_mut().unwrap()[register].address = address;
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_register_value(&self, register: usize) -> Result<DataType, String> {
        if let Some(registers) = &self.registers {
            Ok(Rc::clone(&registers[register].address).borrow().clone())
        } else {
            Err("Registers not initialized.".to_string())
        }
    }

    pub fn get_register_address(&self, register: usize) -> Result<Rc<RefCell<DataType>>, String> {
        if let Some(registers) = &self.registers {
            Ok(registers[register].address.clone())
        } else {
            Err("Registers not initialized.".to_string())
        }
    }

    pub fn get_register_address_index(&self, register: usize) -> Result<usize, String> {
        if let Some(registers) = &self.registers {
            Ok(self.memory.iter().position(|data| Rc::ptr_eq(data, &registers[register].address)).unwrap())
        } else {
            Err("Registers not initialized.".to_string())
        }
    }

    pub fn declare_variable(&mut self, name: String, value: DataType, mutable: bool) -> Result<(), String> {
        let result = self.get_or_add_to_memory(value);
        match result {
            Ok(address) => {
                self.declare_variable_from_memory(name, address, mutable)
            },
            Err(e) => {
                panic!("{}", e);
            },
        }
    }

    pub fn declare_variable_from_memory(&mut self, name: String, address: Rc<RefCell<DataType>>, mutable: bool) -> Result<(), String> {
        if let Some(current_scope) = self.scopes.last_mut() {
            // Check if the variable is already declared
            if current_scope.get_all_symbols().contains_key(&name) {
                return Err(format!("Variable '{}' already declared in this scope.", name));
            }

            let symbol = Symbol {
                name: name.clone(),
                address,
                mutable,
            };
            
            current_scope.symbols.insert(name, symbol);
            Ok(())
        } else {
            Err("No scope to declare variable in.".to_string())
        }
    }

    fn get_variable_base(&self, name: &str) -> Result<Option<&Symbol>, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.symbols.get(name) {
                return Ok(Some(symbol));
            }
        }

        Err(format!("Variable '{}' not found.", name))
    }

    pub fn get_variable(&self, name: &str) -> Result<Option<DataType>, String> {
        let result = self.get_variable_base(name);
        match result {
            Ok(Some(symbol)) => Ok(Some(Rc::clone(&symbol.address).borrow().clone())),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn get_variable_address(&self, name: &str) -> Result<Option<Rc<RefCell<DataType>>>, String> {
        let result = self.get_variable_base(name);
        match result {
            Ok(Some(symbol)) => Ok(Some(symbol.address.clone())),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn set_variable_address(&mut self, name: &str, address: Rc<RefCell<DataType>>) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.symbols.get_mut(name) {
                if !symbol.mutable {
                    return Err(format!("Variable '{}' is not mutable.", name));
                }
    
                symbol.address = address;
                return Ok(());
            }
        }

        Err(format!("Variable '{}' not found.", name))
    }

    pub fn declare_function(&mut self, name: String, params: Vec<String>, instructions: Vec<ASTNode>) -> Result<(), String> {
        let function_address_index = self.get_free_address_index();
        let function_placeholder = self.memory[function_address_index].clone();

        let function_symbol = Symbol {
            name: name.clone(),
            address: function_placeholder.clone(),
            mutable: false,
        };

        let joined_scopes = self.scopes.iter().rev().cloned().fold(Scope::new(None), |mut acc, scope| {
            for (name, symbol) in scope.symbols.iter() {
                acc.symbols.insert(name.clone(), symbol.clone());
            }
            acc.symbols.insert(name.clone(), function_symbol.clone()); // Insert the function itself
            acc
        });

        if let Some(current_scope) = self.scopes.last_mut() {
            let function = DataType::Function(params, instructions, joined_scopes);
            *function_placeholder.borrow_mut() = function; // Update the placeholder with the actual function
            current_scope.symbols.insert(name, function_symbol);
        };
        Ok(())
    }

    pub fn get_function(&self, name: &str) -> Result<(Vec<String>, Vec<ASTNode>, Scope), String> {
        match self.get_variable_base(name) {
            Ok(Some(symbol)) => {
                if let DataType::Function(params, instructions, captured_scope) = Rc::clone(&symbol.address).borrow().clone() {
                    Ok((params, instructions, captured_scope))
                } else {
                    Err(format!("Expected function, got: {:?}", Rc::clone(&symbol.address).borrow().clone()))
                }
            },
            Ok(None) => Err(format!("Function '{}' not found.", name)),
            Err(e) => Err(e),
        }
    }
}