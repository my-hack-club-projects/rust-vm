mod instruction;
mod vm;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        instruction::Instruction::DeclareFunc("myFunction".to_string(), vec!["a".to_string(), "b".to_string()], vec![
            // instruction::Instruction::DeclareVar("testVar".to_string(), 99),

            instruction::Instruction::LoadVar(0, "a".to_string()),
            instruction::Instruction::LoadVar(1, "b".to_string()),
            instruction::Instruction::Out(0),
            instruction::Instruction::Out(1),

            instruction::Instruction::Add(0, 1),
            instruction::Instruction::RetFunc(vec![0]),
        ]),

        instruction::Instruction::DeclareVar("myVar1".to_string(), 10),
        instruction::Instruction::DeclareVar("myVar2".to_string(), 123),
        instruction::Instruction::CallFunc("myFunction".to_string(), vec!["myVar1".to_string(), "myVar2".to_string()]),
        instruction::Instruction::Out(0),

        // instruction::Instruction::LoadVar(0, "testVar".to_string()),
        // instruction::Instruction::Out(0),

        instruction::Instruction::Halt,
    ];

    vm.execute(program);
}