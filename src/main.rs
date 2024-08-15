use std::vec;

use instruction::Instruction;

mod instruction;
mod vm;
mod symbol;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        Instruction::LoadLiteral(0, 1),
        Instruction::RegDump,
        // Instruction::DeclareMutVar(0, "x".to_string()),

        Instruction::Add(3, vec![7, 6]),
        Instruction::Debug(3),

        // Instruction::While(
        //     vec![
        //         Instruction::LoadVar(0, "x".to_string()),
        //         Instruction::LoadLiteral(1, 10),
        //         Instruction::Lt(0, vec![0, 1]), // Loop while x < 10
        //         Instruction::RetFunc(vec![0]),
        //     ],
        //     vec![
        //         Instruction::LoadVar(0, "x".to_string()),
        //         Instruction::LoadLiteral(1, 1),
        //         Instruction::Add(0, vec![0, 1]),
        //         Instruction::StoreVar(0, "x".to_string()),
        //         Instruction::Debug(0),

        //         Instruction::LoadLiteral(1, 5),
        //         Instruction::Gt(0, vec![0, 1]), // Test 'BreakWhile' by breaking the loop when x > 5
        //         Instruction::If(0, vec![
        //             Instruction::BreakWhile,
        //         ]),
        //     ],
        // ),

        Instruction::Halt,
    ];

    vm.execute(program);
}