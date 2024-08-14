use std::vec;

use instruction::Instruction;

mod instruction;
mod vm;
mod symbol;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        Instruction::LoadLiteral(0, 1),
        Instruction::If(0, vec![
            Instruction::LoadLiteral(1, 123),
            Instruction::Out(1),

            Instruction::LoadLiteral(0, 0),
            Instruction::LoadLiteral(1, 1),
            
            Instruction::If(0, vec![
                Instruction::LoadLiteral(1, 456),
                Instruction::Out(1)
                ]),
            Instruction::ElseIf(1, vec![
                Instruction::LoadLiteral(1, 789),
                Instruction::Out(1),
            ]),
            Instruction::Else(vec![
                Instruction::LoadLiteral(1, 987),
                Instruction::Out(1),
            ]),
        ]),
        Instruction::Else(vec![
            Instruction::LoadLiteral(1, 321),
            Instruction::Out(1),
        ]),

        Instruction::Halt,
    ];

    vm.execute(program);
}