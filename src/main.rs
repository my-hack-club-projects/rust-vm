use std::vec;

use instruction::Instruction;

mod instruction;
mod vm;
mod symbol;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        Instruction::DeclareFunc("addNums".to_string(), vec!["a".to_string(), "b".to_string()], vec![
            Instruction::LoadVar(0, "a".to_string()),
            Instruction::LoadVar(1, "b".to_string()),
            Instruction::Out(0),
            Instruction::Out(1),
            Instruction::Add(0, 1),
            Instruction::RetFunc(vec![0]),
        ]),

        Instruction::LoadLiteral(0, 5),
        Instruction::LoadLiteral(1, 10),
        Instruction::CallFunc("addNums".to_string(), vec![0, 1]),
        Instruction::Out(0),

        Instruction::Halt,
    ];

    vm.execute(program);
}