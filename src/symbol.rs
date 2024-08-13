use std::collections::HashMap;
use crate::instruction::Instruction;

// This programming language is supposed to be number-only. There are no datatypes like strings or booleans.
// Only numbers and functions.

#[derive(Clone, Debug)]
pub enum DataType {
    Number,
    Function(Vec<String>, Vec<Instruction>),
}

#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: String,
    pub address: usize,
    pub data_type: DataType,
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
}

impl Default for Scope {
    fn default() -> Self {
        Scope {
            symbols: HashMap::new(),
        }
    }
}
