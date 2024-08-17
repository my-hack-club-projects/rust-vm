use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::fmt;

use crate::ast::parser::ASTNode;
use crate::vm::instruction::Instruction;

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

#[derive(Clone, Debug)]
pub enum DataType {
    Number(i32),
    Function(Vec<String>, Vec<ASTNode>, Scope),
    Null(),
}

impl std::ops::Add for DataType {
    type Output = DataType;

    fn add(self, other: DataType) -> DataType {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => DataType::Number(a + b),
            _ => panic!("Expected numbers"),
        }
    }
}

impl std::ops::Sub for DataType {
    type Output = DataType;

    fn sub(self, other: DataType) -> DataType {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => DataType::Number(a - b),
            _ => panic!("Expected numbers"),
        }
    }
}

impl std::ops::Mul for DataType {
    type Output = DataType;

    fn mul(self, other: DataType) -> DataType {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => DataType::Number(a * b),
            _ => panic!("Expected numbers"),
        }
    }
}

impl std::ops::Div for DataType {
    type Output = DataType;

    fn div(self, other: DataType) -> DataType {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => DataType::Number(a / b),
            _ => panic!("Expected numbers"),
        }
    }
}

impl std::ops::Rem for DataType {
    type Output = DataType;

    fn rem(self, other: DataType) -> DataType {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => DataType::Number(a % b),
            _ => panic!("Expected numbers"),
        }
    }
}

impl std::ops::Not for DataType {
    type Output = DataType;

    fn not(self) -> DataType {
        match self {
            DataType::Number(n) => DataType::Number(!n),
            _ => panic!("Expected number"),
        }
    }
}

impl std::ops::BitAnd for DataType {
    type Output = DataType;

    fn bitand(self, other: DataType) -> DataType {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => DataType::Number(a & b),
            _ => panic!("Expected numbers"),
        }
    }
}

impl std::ops::BitOr for DataType {
    type Output = DataType;

    fn bitor(self, other: DataType) -> DataType {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => DataType::Number(a | b),
            _ => panic!("Expected numbers"),
        }
    }
}

impl std::ops::BitXor for DataType {
    type Output = DataType;

    fn bitxor(self, other: DataType) -> DataType {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => DataType::Number(a ^ b),
            _ => panic!("Expected numbers"),
        }
    }
}

impl std::cmp::PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => a == b,
            (DataType::Function(_, _, _), DataType::Function(_, _, _)) => false,
            (DataType::Null(), DataType::Null()) => true,
            _ => false,
        }
    }
}

impl std::cmp::PartialOrd for DataType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => a.partial_cmp(b),
            _ => panic!("Expected numbers"),
        }
    }
}

impl std::cmp::Eq for DataType {}

impl std::cmp::Ord for DataType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (DataType::Number(a), DataType::Number(b)) => a.cmp(b),
            _ => panic!("Expected numbers"),
        }
    }
}

// neg
impl std::ops::Neg for DataType {
    type Output = DataType;

    fn neg(self) -> DataType {
        match self {
            DataType::Number(n) => DataType::Number(-n),
            _ => panic!("Expected number"),
        }
    }
}

impl DataType {
    /// Returns `true` if the data type is [`Null`].
    ///
    /// [`Null`]: DataType::Null
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null(..))
    }

    /// Returns `true` if the data type is [`Number`].
    ///
    /// [`Number`]: DataType::Number
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
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