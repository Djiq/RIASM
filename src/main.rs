use std::{any::Any, collections::HashMap, convert::TryInto, fmt::Result, vec};
pub mod asm_instruction;
pub mod asm_value;

use crate::asm_instruction::*;
use crate::asm_value::*;

#[derive(Clone)]
enum ASTNode {
    ASTValue(ASMValue),
    ASTInstruction(String),
    ASTRegister(String),
    ASTExprEnd,
}

pub struct ASMDefinition {
    registers: HashMap<String, ASMValue>,
    instructions: HashMap<String, ASMInstruction>,
    priority: u16,
    ptr_to_self: Option<*mut ASMDefinition>
}

impl ASMDefinition {
    pub fn dump_state(&self) {
        println!("== ASMDefinition STATE DUMP BEGIN ==");
        for (regs_name, reg_val) in self.registers.iter(){
            println!("REGISTER {0} is {1}",regs_name,reg_val);
        }
        for (instruction_name,_instruction) in self.instructions.iter(){
            println!("FOUND INSTRUCTION: {}", instruction_name);
        }
        println!("== ASMDefinition STATE DUMP END ==")
    }
}

pub struct ASMProgram {
    instruction_set: ASMDefinition,
    token_vec: Vec<ASTNode>,
}

fn build_definition() -> ASMDefinition {
    let mut definition = ASMDefinition {
        registers: HashMap::new(),
        instructions: HashMap::new(),
        priority: 1,
        ptr_to_self:None
    };

    let definition_pointer: *mut ASMDefinition = &mut definition;
    definition.ptr_to_self = Some(definition_pointer);
    definition
        .registers
        .insert("R1".into(), ASMValue::new_empty(Some(definition_pointer)));
    definition
        .registers
        .insert("R2".into(), ASMValue::new_empty(Some(definition_pointer)));

    definition.instructions.insert(
        "ADD".into(),
        ASMInstruction::new(
            |a| a[0].try_modify_register(a[0].resolve() + a[1].resolve()),
            definition_pointer,
        ),
    );
    definition.instructions.insert(
        "MOV".into(),
        ASMInstruction::new(
            |a| {
                a[0].try_modify_register(a[1].resolve());
                println!("{0} is {1}",a[0] , a[0].resolve());
                println!("{0} is {1}",a[1] ,a[1].resolve())
            },
            definition_pointer,
        ),
    );
    definition.instructions.insert(
        "PRINT".into(),
        ASMInstruction::new(
            |a| {
                if let ASMValueHolder::Register(reference) = a[0].get_value_holder() {
                    println!("VALUE OF REGISTER {0} IS {1}", a[0], a[0].resolve())
                } else {
                    println!("VALUE OF {}", a[0])
                }
            },
            definition_pointer,
        ),
    );
    definition
}

fn run_with_definition(
    definition: ASMDefinition,
    token_stream: Vec<ASTNode>,
) -> Option<&'static str> {
    let mut current_instruction: Option<ASMInstruction> = None;
    let mut current_args: Vec<ASMValue> = Vec::new();
    let mut current_position: usize = 0;
    while current_position < token_stream.len() {
        let token: ASTNode = token_stream[current_position].clone();
        match token {
            ASTNode::ASTValue(value) => {
                if current_instruction.is_none() {
                    return Some("ASTValue encountered with no instruction being present!");
                }
                println!("Value! Encountered {}", value);
                current_args.push(value.clone());
            }
            ASTNode::ASTInstruction(instruction) => {
                if current_instruction.is_some() {
                    return Some("ASTInstruction encountered with instruction being present!");
                }
                let instruction_ref = match definition.instructions.get(&instruction) {
                    Some(reference) => reference,
                    None => return Some("No instruction found"),
                };
                println!("Instruction! Encountered {}", instruction);
                current_instruction = Some((*instruction_ref).clone());
            }
            ASTNode::ASTRegister(reference) => {
                if current_instruction.is_none() {
                    return Some("ASTRegister encountered with no instruction being present!");
                }
                let register_ref = match definition.registers.get(&reference) {
                    Some(register) => {
                        ASMValue::new_reg(reference.clone(), definition.ptr_to_self.clone())
                    },
                    None => return Some("No register found"),
                };

                println!("Register! Encountered {}", reference);
                current_args.push(register_ref);
            }
            ASTNode::ASTExprEnd => match current_instruction {
                Some(instruction) => {
                    println!("ExprEnd");
                    instruction.call(current_args.clone());
                    current_instruction = None;
                    current_args.clear();
                }
                None => return Some("ASTExpreEnd reached without any instruction being called!"),
            },
        }
        current_position += 1;
    }
    None
}

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

    println!(
        "{}",
        run_with_definition(build_definition(), test_vec).unwrap_or("No errors")
    );

    println!("Hello, world!");
}
