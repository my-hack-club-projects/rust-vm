# no name programming language
(haven't even decided what it's gonna be used for yet)

**This is a simple 'VM' (as in JVM, not oracle virtualbox :p) that interprets instructions into rust functions**
I've never done this kind of thing before and don't know if I'm doing it right and if the terms I'm using are correct.

**Currently supported instructions:**
- Variable assignment, reading (only integers)
- Function declarations, function calling
- While loops with break and continue
- If statements with elseif and else
- Basic arithmetic & logical operations such as +, -, AND, NOT

There's a sort of garbage collection mechanism that reuses old, unused data in memory.

## how to run
- Make sure you have 'cargo' (rust's package manager) installed
- Clone this repository and in the root, run ```cargo run```
- In the main.rs file, import the 'vm' module and run ```vm.execute``` with a Vec of Instructions, like this:
```rs
use std::vec;

mod vm;
use vm::instruction::Instruction;

fn main() {
    let mut vm = vm::VM::new();

    let program = vec![
        // Your instructions/program here
        Instruction::LoadLiteral(123, 2),
        Instruction::DeclareVar(0, "x".to_string()), // Holds the value of '123'

        Instruction::Halt,
    ];

    vm.execute(program);
}
```
**NOTE:** The main.rs file constantly changes, as I'm using it for debugging. This is NOT production ready in any way!