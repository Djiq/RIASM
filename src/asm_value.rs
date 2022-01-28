use core::fmt;
use std::{any::Any, ops, result::Result};

use crate::asm_definition::ASMDefinition;

#[derive(Debug, Clone)]
pub enum ASMValueHolder {
    Int(i32),
    Str(String),
    Float(f32),
    Register(String),
    Invalid,
}

impl fmt::Display for ASMValueHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASMValueHolder::Int(val) => write!(f, "{}", val),
            ASMValueHolder::Str(val) => write!(f, "{}", val),
            ASMValueHolder::Float(val) => write!(f, "{}", val),
            ASMValueHolder::Register(reference) => write!(f, "{}", reference),
            ASMValueHolder::Invalid => write!(f, "NIL"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ASMValue {
    lang_definiton: Option<*mut ASMDefinition>,
    value: ASMValueHolder,
}

impl fmt::Display for ASMValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ASMValue {
    pub fn new_int(_value: i32, ldef: Option<*mut ASMDefinition>) -> Self {
        ASMValue {
            lang_definiton: ldef,
            value: ASMValueHolder::Int(_value),
        }
    }

    pub fn new_empty(ldef: Option<*mut ASMDefinition>) -> Self {
        ASMValue {
            lang_definiton: ldef,
            value: ASMValueHolder::Invalid,
        }
    }

    pub fn new_reg(_reg_name: String, ldef: Option<*mut ASMDefinition>) -> Self {
        ASMValue {
            lang_definiton: ldef,
            value: ASMValueHolder::Register(_reg_name),
        }
    }

    pub fn get_value_holder(&self) -> ASMValueHolder {
        self.value.clone()
    }

    pub fn try_into_i32(&self) -> Result<i32, &str> {
        if let ASMValueHolder::Int(value) = self.value {
            Ok(value)
        } else {
            Err("Wrong value type!")
        }
    }

    pub fn get_lang_definition(&self) -> Option<*mut ASMDefinition> {
        self.lang_definiton
    }

    pub fn resolve(&self) -> ASMValue {
        match self.value {
            ASMValueHolder::Int(_) => self.clone(),
            ASMValueHolder::Str(_) => self.clone(),
            ASMValueHolder::Float(_) => self.clone(),
            ASMValueHolder::Register(_) => self.try_resolve_register().unwrap(),
            ASMValueHolder::Invalid => self.clone(),
        }
    }

    pub fn try_resolve_register(&self) -> Result<ASMValue, &str> {
        if let ASMValueHolder::Register(reference) = self.value.clone() {
            match self.lang_definiton {
                Some(ptr) => {
                    unsafe{
                    let definition = &*ptr as &ASMDefinition;
                    return Ok(definition.registers.get(&reference).unwrap().clone());
                    }
                },
                None => return Err("ASMValue was a reference and wasnt holding a valid lanugage definition pointer!"),
            }
        }
        Err("ASMValue wasnt a reference!")
    }

    pub fn try_modify_register(&self, value: ASMValue) {
        if let ASMValueHolder::Register(reference) = self.value.clone() {
            match self.lang_definiton {
                Some(ptr) => unsafe {
                    let definition = &mut *ptr as &mut ASMDefinition;
                    definition.registers.insert(reference, value.clone());
                },
                None => {
                    println!("Failed to modify state of {}", reference);
                }
            }
        } else {
            println!("Value is not a register!");
        }
    }
}

impl From<i32> for ASMValue {
    fn from(item: i32) -> Self {
        ASMValue::new_int(item, None)
    }
}

impl ops::Add<ASMValue> for ASMValue {
    type Output = ASMValue;
    fn add(self, rhs: ASMValue) -> Self::Output {
        match self.value {
            ASMValueHolder::Int(lvalue) => {
                if let ASMValueHolder::Int(rvalue) = rhs.value {
                    ASMValue::new_int(rvalue + lvalue, self.lang_definiton)
                } else {
                    ASMValue::new_empty(self.lang_definiton)
                }
            }
            ASMValueHolder::Str(_) => todo!(),
            ASMValueHolder::Float(_) => todo!(),
            ASMValueHolder::Invalid => ASMValue::new_empty(self.lang_definiton),
            ASMValueHolder::Register(_) => todo!(),
        }
    }
}
