use std::vec;

use instruction::Instruction;

mod instruction;
mod vm;
mod symbol;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        Instruction::LoadLiteral(0, 123),
        Instruction::DeclareVar(0, "immutableVar".to_string()),
        Instruction::LoadLiteral(1, 456),
        Instruction::DeclareMutVar(1, "mutableVar".to_string()),

        Instruction::LoadVar(0, "immutableVar".to_string()),
        Instruction::Out(0),

        Instruction::LoadVar(0, "mutableVar".to_string()),
        Instruction::Out(0),

        Instruction::LoadLiteral(1, 12),
        Instruction::Add(0, 1),
        Instruction::StoreVar(0, "mutableVar".to_string()),

        Instruction::LoadVar(0, "immutableVar".to_string()),
        Instruction::LoadLiteral(0, 1),
        Instruction::Add(0, 1),
        Instruction::StoreVar(0, "immutableVar".to_string()),
    ];

    vm.execute(program);
}