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
}

#[derive(Clone, Debug, PartialEq)]
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
