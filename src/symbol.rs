use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::instruction::Instruction;
use std::fmt;

// This programming language is supposed to be number-only. There are no datatypes like strings or booleans.
// Only numbers and functions.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Register {
    pub address: usize,
}

impl Register {
    pub fn new(address: usize) -> Register {
        Register {
            address,
        }
    }

    pub fn get_value(&self, memory: &[Rc<RefCell<DataType>>]) -> Option<DataType> {
        // Some(Rc::clone(&memory[self.address]))
        if self.address < memory.len() {
            Some(Rc::clone(&memory[self.address]).borrow().clone())
        } else {
            panic!("Error: Register address out of bounds.");
        }
    }

    pub fn set_value(&self, memory: &mut Vec<DataType>, value: DataType) {
        if self.address < memory.len() {
            println!("Setting mem[{}] to {}", self.address, value);
            memory[self.address] = value;
        } else {
            panic!("Error: Register address out of bounds.");
        }
    }
    
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Number(i32),
    Function(Vec<String>, Vec<Instruction>, Scope),
    Null(),
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Number(n) => write!(f, "{}", n),
            DataType::Function(_, _, _) => write!(f, "Function"),
            DataType::Null() => write!(f, "Null"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Symbol {
    pub name: String,
    pub address: usize,
    pub mutable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Scope {
        Scope {
            symbols: HashMap::new(),
            parent: match parent {
                Some(p) => Some(Box::new(p)),
                None => None,
            },
        }
    }

    pub fn get_all_symbols(&self) -> HashMap<String, Symbol> {
        let mut symbols = self.symbols.clone();
        if let Some(parent) = &self.parent {
            for (key, value) in parent.get_all_symbols() {
                symbols.insert(key, value);
            }
        }
        symbols
    }
}