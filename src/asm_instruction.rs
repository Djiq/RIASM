use crate::{asm_value::ASMValue, ASMDefinition};

#[derive(Clone)]
pub struct ASMInstruction {
    function: fn(Vec<ASMValue>),
    lang_definition: *mut ASMDefinition,
}

impl ASMInstruction {
    pub fn new(_function: fn(Vec<ASMValue>), _lang_definition: *mut ASMDefinition) -> Self {
        ASMInstruction {
            function: _function,
            lang_definition: _lang_definition,
        }
    }

    pub fn call(&self, args: Vec<ASMValue>) {
        (self.function)(args);
    }

    pub fn call_with_slice(&self, args: &[ASMValue]) {
        (self.function)(args.to_vec())
    }
}
