use std::vec;

use instruction::Instruction;

mod instruction;
mod vm;
mod symbol;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        instruction::Instruction::LoadLiteral(0, 123),
        instruction::Instruction::DeclareVar(0, "immutableVar".to_string()),
        instruction::Instruction::LoadLiteral(1, 456),
        instruction::Instruction::DeclareMutVar(1, "mutableVar".to_string()),

        instruction::Instruction::LoadVar(0, "immutableVar".to_string()),
        instruction::Instruction::Out(0),

        instruction::Instruction::LoadVar(0, "mutableVar".to_string()),
        instruction::Instruction::Out(0),

        instruction::Instruction::LoadLiteral(1, 12),
        instruction::Instruction::Add(0, 1),
        instruction::Instruction::StoreVar(0, "mutableVar".to_string()),

        instruction::Instruction::LoadVar(0, "immutableVar".to_string()),
        instruction::Instruction::LoadLiteral(0, 1),
        instruction::Instruction::Add(0, 1),
        instruction::Instruction::StoreVar(0, "immutableVar".to_string()),
    ];

    vm.execute(program);
}