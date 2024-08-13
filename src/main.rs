mod instruction;
mod vm;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        instruction::Instruction::Load(0, 10),
        instruction::Instruction::Load(1, 11),
        instruction::Instruction::Add(0, 1),
        instruction::Instruction::Store(0, 12),
        instruction::Instruction::Out(0),
        instruction::Instruction::Halt,
    ];

    vm.memory[10] = 5;
    vm.memory[11] = 10;

    vm.execute(program);

    println!("Memory[12]: {}", vm.memory[12]); // Should output 15
}
