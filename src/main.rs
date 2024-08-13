mod instruction;
mod vm;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        instruction::Instruction::DeclareVar("GLOBAL_VAR".to_string(), 99),

        instruction::Instruction::DeclareFunc("myFunction".to_string(), vec!["a".to_string(), "b".to_string()], vec![
            instruction::Instruction::LoadVar(0, "GLOBAL_VAR".to_string()),
            instruction::Instruction::Out(0),

            instruction::Instruction::LoadVar(0, "myVar1".to_string()), // Should error, myVar1 is not in captured scope
            instruction::Instruction::Out(0),
            instruction::Instruction::DeclareVar("testVar".to_string(), 99),

            instruction::Instruction::LoadVar(0, "a".to_string()),
            instruction::Instruction::LoadVar(1, "b".to_string()),
            instruction::Instruction::Out(0),
            instruction::Instruction::Out(1),

            instruction::Instruction::Add(0, 1),
            instruction::Instruction::RetFunc(vec![0]),
        ]),

        instruction::Instruction::DeclareVar("myVar1".to_string(), 10),
        instruction::Instruction::DeclareVar("myVar2".to_string(), 20),
        instruction::Instruction::CallFunc("myFunction".to_string(), vec!["myVar1".to_string(), "myVar2".to_string()]),
        // instruction::Instruction::Out(0), // Should output 30

        // instruction::Instruction::LoadVar(0, "testVar".to_string()), // Should error, testVar is not in scope

        instruction::Instruction::Halt,
    ];

    vm.execute(program);
}