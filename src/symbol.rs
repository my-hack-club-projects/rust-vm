use std::collections::HashMap;
use crate::instruction::Instruction;

// This programming language is supposed to be number-only. There are no datatypes like strings or booleans.
// Only numbers and functions.

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Number(i32),
    Function(Vec<String>, Vec<Instruction>, Scope),
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