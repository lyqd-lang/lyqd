use std::fs::File;

use rmp_serde::from_read;
use shared::Bytecode;

use clap::Parser;

#[derive(Parser)]
struct LyqdViewBytecode {
    file: String,
}

fn main() {
    let args = LyqdViewBytecode::parse();
    let data = File::open(args.file).unwrap();
    let bytecode = from_read::<File, Bytecode>(data).unwrap();
    println!("{:?}", bytecode);
}
