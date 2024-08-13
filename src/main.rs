mod instruction;
mod vm;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        instruction::Instruction::DeclareVar("a".to_string(), 10),
        instruction::Instruction::DeclareVar("b".to_string(), 4),
        instruction::Instruction::DeclareVar("result".to_string(), 0),

        instruction::Instruction::LoadVar("a".to_string()),
        instruction::Instruction::LoadVar("b".to_string()),
        instruction::Instruction::Add(0, 1),
        instruction::Instruction::StoreVar("result".to_string()),

        instruction::Instruction::Out(0),
        instruction::Instruction::Halt,
    ];

    vm.execute(program);

    println!("Value of result: {}", vm.get_variable("result").unwrap());
}