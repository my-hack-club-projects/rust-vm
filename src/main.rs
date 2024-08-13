use std::vec;

use instruction::Instruction;

mod instruction;
mod vm;
mod symbol;

fn main() {
    let mut vm = vm::VM::new();

    // let program = vec![
    //     instruction::Instruction::DeclareFunc("myFunction".to_string(), vec!["a".to_string(), "b".to_string()], vec![
    //         instruction::Instruction::LoadVar(0, "a".to_string()),
    //         instruction::Instruction::LoadVar(1, "b".to_string()),
    //         instruction::Instruction::Out(0),
    //         instruction::Instruction::Out(1),

    //         instruction::Instruction::Add(0, 1),
    //         instruction::Instruction::RetFunc(vec![0]),
    //     ]),

    //     instruction::Instruction::DeclareVar("myVar1".to_string(), 10),
    //     instruction::Instruction::DeclareVar("myVar2".to_string(), 20),
    //     instruction::Instruction::CallFunc("myFunction".to_string(), vec!["myVar1".to_string(), "myVar2".to_string()]),
    //     instruction::Instruction::Out(0), // Should output 30

    //     instruction::Instruction::Halt,
    // ];

    let program = vec![
        instruction::Instruction::DeclareFunc("recursion".to_string(), vec!["n".to_string()], vec![
            instruction::Instruction::LoadVar(0, "n".to_string()),
            instruction::Instruction::Out(0),
            instruction::Instruction::LoadLiteral(1, 1), // Load 1 into register 1
            instruction::Instruction::Add(0, 1),
            instruction::Instruction::CallFunc("recursion".to_string(), vec![0]),
            instruction::Instruction::RetFunc(vec![0]), // doesn't do anything now, as the recursion never ends
        ]),

        instruction::Instruction::LoadLiteral(0, 999),
        instruction::Instruction::CallFunc("recursion".to_string(), vec![0]),
    ];

    vm.execute(program);
}