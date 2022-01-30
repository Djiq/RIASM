# RIASM
RIASM stands for Rust Interpreter for ASM. It is a basic, asm-like language interpreter that allows you to write your own instructions and define your own registers, allowing you to embed this in for example some kind of game,
allowing users to interface with a basic interpreter. The big benefit of using RIASM is that it is extremely bare bones, it posses no native instructions of it's own, and has no default registers, the end user must define
every single one of these, allowing you to leverage it to define each instruction to mean whatever you like.

# TODO

- [x] Make a String to AST scanner
- [ ] Unshit *mut ASTDefinition to not use a raw pointer, but something like Rc or Arc
- [ ] Add labels
- [ ] Figure out how to make defining instructions not complete shit


# How to build

``` shell
cargo build --release
```

# How To Use
Simply import 

``` rust
riasm::asm_definition::*;
```



and then use provided builder functions to construct and execute the asm interpreter you just built.
Example:

``` rust
use riasm::asm_definition::*;

fn main() {
    let text: String = "
MOV [R1] 0
MOV [R2] 1
loop:
ADD [R1] [R2]
ADD [R2] [R1]
PRINT [R1]
PRINT [R2]
JMP loop
"
    .into();

    ASMDefinition::new()
        .insert_register("R1")
        .insert_register("R2")
        .insert_register("R3")
        .insert_instruction("MOV", |state, arg| {
            arg[0].try_modify_register(arg[1].resolve())
        })
        .insert_instruction("ADD", |state, arg| {
            arg[0].try_modify_register(arg[0].resolve() + arg[1].resolve())
        })
        .insert_instruction("PRINT", |state, arg| println!("{}", arg[0].resolve()))
        .insert_instruction("JMP", |state, arg| state.jump_to_label(arg[0].resolve()))
        .interpret(text);
}
```

	
This example will first push 1 to R1, push 1 to R2, sum them and place the sum in R1 and then display R1.
