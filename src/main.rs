use std::vec;

use instruction::Instruction;

mod instruction;
mod vm;
mod symbol;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        Instruction::LoadLiteral(0, 0),
        Instruction::DeclareMutVar(0, "x".to_string()),

        // Instruction::While() // TODO: implement

        Instruction::Halt,
    ];

    vm.execute(program);
}