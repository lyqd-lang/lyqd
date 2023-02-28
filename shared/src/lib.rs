use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum BCInstruction {
    StackPushString(String),
    Print,
    Return,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Bytecode {
    pub instructions: Vec<BCInstruction>,
    pub labels: HashMap<String, usize>,
}
