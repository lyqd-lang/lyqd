use std::{
    fs::{read_to_string, remove_file, File},
    io::Write,
};

use ast::{ASTNode, Statement};
use clap::Parser;
use serde::Serialize;

use crate::bytecode::Compiler;

pub mod ast;
pub mod bytecode;
pub mod lexer;

#[derive(Parser)]
struct Lyqd {
    file: String,
}

// TODO: better error handling
fn main() {
    let args = Lyqd::parse();
    // lexerifications
    let src = read_to_string(&args.file).unwrap();
    let mut lex = crate::lexer::Lexer::from(&src);
    let mut _lex = crate::lexer::Lexer::from(&src);
    println!("{:?}", _lex.collect::<Vec<_>>());
    let ast = crate::ast::Ast::build(&mut lex);

    // compile to bytecode
    let bytecode = Compiler::from(ast);
    let mut out_file = args.file;
    out_file.push('c');
    let mut buf = vec![];
    bytecode
        .serialize(&mut rmp_serde::Serializer::new(&mut buf))
        .unwrap();
    let _ = remove_file(&out_file); // ignore any error coming from here
    let mut file = File::create(&out_file).unwrap();
    file.write_all(&buf).unwrap();
}
