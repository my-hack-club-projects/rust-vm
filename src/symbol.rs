use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use crate::instruction::Instruction;
use std::fmt;

// This programming language is supposed to be number-only. There are no datatypes like strings or booleans.
// Only numbers and functions.

#[derive(Clone, Debug, PartialEq)]
pub struct Register {
    pub address: Rc<RefCell<DataType>>,
}

impl Register {
    pub fn new(address: Rc<RefCell<DataType>>) -> Register {
        Register {
            address,
        }
    }

    pub fn get_value(&self, memory: &[Rc<RefCell<DataType>>]) -> Option<DataType> {
        if self.address.borrow().is_null() {
            return None;
        }
        let address = self.address.borrow();
        match &*address {
            DataType::Number(n) => Some(DataType::Number(*n)),
            DataType::Function(_, _, _) => Some(DataType::Function(vec![], vec![], Scope::new(None))),
            DataType::Null() => None,
        }
    }

    pub fn set_value(&self, memory: &mut Vec<DataType>, value: DataType) {
        *self.address.borrow_mut() = value;
    }
    
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Number(i32),
    Function(Vec<String>, Vec<Instruction>, Scope),
    Null(),
}

impl DataType {
    /// Returns `true` if the data type is [`Null`].
    ///
    /// [`Null`]: DataType::Null
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null(..))
    }
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
    pub address: Rc<RefCell<DataType>>,
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