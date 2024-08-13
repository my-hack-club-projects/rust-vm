mod instruction;
mod vm;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        instruction::Instruction::DeclareFunc("myFunction".to_string(), 2, vec![
            instruction::Instruction::Add(0, 1),
            instruction::Instruction::RetFunc(0),
        ]),

        instruction::Instruction::CallFunc("myFunction".to_string(), vec![3, 4]),
        instruction::Instruction::Out(0),

        instruction::Instruction::Halt,
    ];

    vm.execute(program);
}