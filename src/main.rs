use riasm::asm_definition::*;

fn main() {
    let test_vec: Vec<ASTNode> = vec![
        ASTNode::ASTInstruction("MOV".into()),
        ASTNode::ASTRegister("R1".into()),
        ASTNode::ASTValue(1.into()),
        ASTNode::ASTExprEnd,
        ASTNode::ASTInstruction("MOV".into()),
        ASTNode::ASTRegister("R2".into()),
        ASTNode::ASTValue(1.into()),
        ASTNode::ASTExprEnd,
        ASTNode::ASTInstruction("ADD".into()),
        ASTNode::ASTRegister("R1".into()),
        ASTNode::ASTRegister("R2".into()),
        ASTNode::ASTExprEnd,
        ASTNode::ASTInstruction("PRINT".into()),
        ASTNode::ASTRegister("R1".into()),
        ASTNode::ASTExprEnd,
    ];

    ASMDefinition::new()
        .insert_register("R1")
        .insert_register("R2")
        .insert_instruction("MOV", |arg| arg[0].try_modify_register(arg[1].resolve()))
        .insert_instruction("ADD", |arg| {
            arg[0].try_modify_register(arg[0].resolve() + arg[1].resolve())
        }).insert_instruction("PRINT", |arg|println!("{}",arg[0].resolve())).run(test_vec);
}
