use riasm::asm_definition::*;

fn main() {
    let text: String = "
MOV R1 1 ;; This is how you write comments!
MOV R2 1
ADD R1 R2
PRINT R1"
        .into();

    ASMDefinition::new()
        .insert_register("R1")
        .insert_register("R2")
        .insert_instruction("MOV", |arg| arg[0].try_modify_register(arg[1].resolve()))
        .insert_instruction("ADD", |arg| {
            arg[0].try_modify_register(arg[0].resolve() + arg[1].resolve())
        })
        .insert_instruction("PRINT", |arg| println!("{}", arg[0].resolve()))
        .interpret(text);
}
