use crate::{asm_value::ASMValue, ASMDefinition};

#[derive(Clone)]
pub struct ASMInstruction {
    function: fn(&mut ASMDefinition, Vec<ASMValue>),
    lang_definition: *mut ASMDefinition,
}

impl ASMInstruction {
    pub fn new(
        _function: fn(&mut ASMDefinition, Vec<ASMValue>),
        _lang_definition: *mut ASMDefinition,
    ) -> Self {
        ASMInstruction {
            function: _function,
            lang_definition: _lang_definition,
        }
    }

    pub fn call(&self, state: &mut ASMDefinition, args: Vec<ASMValue>) {
        (self.function)(state, args);
    }

    pub fn call_with_slice(&self, state: &mut ASMDefinition, args: &[ASMValue]) {
        (self.function)(state, args.to_vec())
    }
}
