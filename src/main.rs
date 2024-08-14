use std::vec;

use instruction::Instruction;

mod instruction;
mod vm;
mod symbol;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        Instruction::LoadLiteral(0, 1),
        Instruction::DeclareMutVar(0, "x".to_string()),

        Instruction::While(
            vec![
                Instruction::LoadVar(0, "x".to_string()),
                Instruction::LoadLiteral(1, 10),
                Instruction::Gt(1, 0), // Careful: the first argument is the register that will be modified.
                // If we had written Lt(0, 1), the register 0, which is the value of x, would be modified.
                // This should be fixed by making all comparison instructions take the register to be modified as the first argument
                // and a Vec (2) of registers to be compared as the second argument.
                Instruction::RetFunc(vec![1]),
            ],
            vec![
                Instruction::LoadVar(0, "x".to_string()),
                Instruction::LoadLiteral(1, 1),
                Instruction::Debug(1),
                Instruction::Add(0, 1),
                Instruction::StoreVar(0, "x".to_string()),
                Instruction::Debug(0),

                Instruction::LoadLiteral(1, 5),
                Instruction::Lt(1, 0), // Test 'BreakWhile' by breaking the loop when x > 5
                Instruction::If(1, vec![
                    Instruction::BreakWhile,
                ]),
            ],
        ),

        Instruction::Halt,
    ];

    vm.execute(program);
}