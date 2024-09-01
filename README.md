# unnamed programming language

This is a simple interpreted programming language written in Rust. I plan on making it a math-oriented programming language, perhaps to be used with a raspberry pi calculator?

![image](https://github.com/user-attachments/assets/443e80e6-57b8-48ca-891d-20ce0fd40dbe)

## Supported features:
- Variable declaration & assignment (integers only for now)
- Immutable variables by default, mutable variables using the 'mut' keyword
- Functions
- While loops & if-statements
- a basic REPL

## WIP Feature: Math equation code blocks, for example `x + 2 = y` would be valid syntax.
## Example code:
```
var x = 1
mut y = 0

while y < 10 {
    y += x
}

out x y # Outputs the values of 'x' and 'y'
```

## How to run
1. Clone this repository
2. Make sure you have 'cargo' installed (rust's package manager)
3. run `cargo run`. This will start a REPL
