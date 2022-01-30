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
