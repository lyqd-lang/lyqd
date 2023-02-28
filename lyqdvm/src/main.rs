use std::{fmt::Display, fs::File};

use rmp_serde::from_read;
use shared::{BCInstruction, Bytecode};

use clap::Parser;

#[derive(Parser)]
struct LyqdVM {
    file: String,
}

#[derive(Debug)]
enum RuntimeValue {
    String(String),
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(data) => f.write_str(&data[1..data.len() - 1]),
        }
    }
}

fn main() {
    let args = LyqdVM::parse();
    let data = File::open(args.file).unwrap();
    let bytecode = from_read::<File, Bytecode>(data).unwrap();
    // interpret the bytecode
    let mut call_stack: Vec<usize> = vec![];
    let mut value_stack: Vec<RuntimeValue> = vec![];
    if !bytecode.labels.contains_key("main") {
        panic!("Tried to run a library");
    }
    let mut code_ptr = bytecode.labels["main"];
    call_stack.push(code_ptr);

    'running: loop {
        let current_inst = &bytecode.instructions[code_ptr];
        match current_inst {
            BCInstruction::Return => {
                call_stack.pop();
                if call_stack.is_empty() {
                    break 'running;
                }
                code_ptr = call_stack[call_stack.len() - 1];
            }
            BCInstruction::StackPushString(data) => {
                value_stack.push(RuntimeValue::String(data.to_string()))
            }
            BCInstruction::Print => println!("{}", value_stack[value_stack.len() - 1]),
        }
        code_ptr += 1;
    }
}
