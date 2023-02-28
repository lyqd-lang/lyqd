use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum BCType {
    Void,
    IntPtr,
    // Custom {
    //
    // }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BCValue {
    IntPtr(isize),
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Register {
    R1, // used for printing
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BCInstruction {
    RegisterConstString(String, Register),
    RegisterVariable(String, Register),
    CreateVariable(String, BCType, bool), // <name>, <type>, <mutuability>
    SetVariable(String, BCType, BCValue), // <name>, <type>, <value>
    Print,
    Return,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Bytecode {
    pub instructions: Vec<BCInstruction>,
    pub labels: HashMap<String, usize>,
}
