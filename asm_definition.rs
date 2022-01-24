

pub struct ASMDefinition {
    registers: HashMap<String, ASMValue>,
    instructions: HashMap<String, ASMInstruction>,
    _priority: u16,
    ptr_to_self: Option<*mut ASMDefinition>,
}

impl ASMDefinition {
    pub fn dump_state(&self) {
        println!("== ASMDefinition STATE DUMP BEGIN ==");
        for (regs_name, reg_val) in self.registers.iter() {
            println!("REGISTER {0} is {1}", regs_name, reg_val);
        }
        for (instruction_name, _instruction) in self.instructions.iter() {
            println!("FOUND INSTRUCTION: {}", instruction_name);
        }
        println!("== ASMDefinition STATE DUMP END ==")
    }

    pub fn new() -> Self {
        let mut def = ASMDefinition {
            registers: HashMap::new(),
            instructions: HashMap::new(),
            _priority: 1,
            ptr_to_self: None,
        };
        let def_ptr: *mut ASMDefinition = &mut def;
        def.ptr_to_self = Some(def_ptr);
        def
    }

    pub fn insert_register(mut self, reg_name: &str) -> Self {
        self.registers.insert(
            reg_name.into(),
            ASMValue::new_empty(self.ptr_to_self.clone()),
        );
        self
    }

    pub fn insert_instruction(
        mut self,
        instruction_name: &str,
        closure: fn(Vec<ASMValue>),
    ) -> Self {
        self.instructions.insert(
            instruction_name.into(),
            ASMInstruction::new(closure, self.ptr_to_self.unwrap().clone()),
        );
        self
    }

    pub fn raise_exception(&self, error_message: &str, halt_execution: bool) {
        println!("{}", error_message);
        if halt_execution {
            panic!();
        }
    }

    pub fn run(&self, token_stream: Vec<ASTNode>) {
        let mut current_instruction: Option<ASMInstruction> = None;
        let mut current_args: Vec<ASMValue> = Vec::new();
        let mut current_position: usize = 0;
        while current_position < token_stream.len() {
            let token: ASTNode = token_stream[current_position].clone();
            match token {
                ASTNode::ASTValue(value) => {
                    if current_instruction.is_none() {
                        self.raise_exception(
                            "ASTValue encountered with no instruction present",
                            false,
                        );
                        return;
                    }
                    current_args.push(value.clone());
                }
                ASTNode::ASTInstruction(instruction) => {
                    if current_instruction.is_some() {
                        self.raise_exception(
                            "ASTInstruction encountered when another instruction is called",
                            false,
                        );
                        return;
                    }
                    let instruction_ref = match self.instructions.get(&instruction) {
                        Some(reference) => reference,
                        None => {
                            self.raise_exception("Not a valid instruction", false);
                            return;
                        }
                    };
                    current_instruction = Some((*instruction_ref).clone());
                }
                ASTNode::ASTRegister(reference) => {
                    if current_instruction.is_none() {
                        self.raise_exception(
                            "Register reference encountered with no instruction present",
                            false,
                        );
                        return;
                    }
                    let register_ref = match self.registers.get(&reference) {
                        Some(_) => ASMValue::new_reg(reference.clone(), self.ptr_to_self.clone()),
                        None => {
                            self.raise_exception("Register not defined in ASMDefinition", false);
                            return;
                        }
                    };

                    current_args.push(register_ref);
                }
                ASTNode::ASTExprEnd => match current_instruction {
                    Some(instruction) => {
                        instruction.call(current_args.clone());
                        current_instruction = None;
                        current_args.clear();
                    }
                    None => {}
                },
            }
            current_position += 1;
        }
    }

    pub fn parse_then_run(code: String) {}
}
