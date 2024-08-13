mod instruction;
mod vm;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        instruction::Instruction::DeclareFunc("myFunction".to_string(), 2, vec![
            instruction::Instruction::Out(0),
            instruction::Instruction::Out(1), // Print the parameters (for debugging/demonstration purposes)

            instruction::Instruction::Add(0, 1),
            instruction::Instruction::RetFunc(vec![0]),
        ]),

        instruction::Instruction::DeclareVar("myVar1".to_string(), 10),
        instruction::Instruction::DeclareVar("myVar2".to_string(), 123),
        instruction::Instruction::LoadVar(0, "myVar1".to_string()),
        instruction::Instruction::LoadVar(1, "myVar2".to_string()),
        instruction::Instruction::CallFunc("myFunction".to_string(), vec![0, 1]),
        instruction::Instruction::Out(0),

        instruction::Instruction::Halt,
    ];

    vm.execute(program);
}