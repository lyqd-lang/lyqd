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

    // print some debug info
    println!();
    println!();
    for i in 0..ast.functions.len() {
        let func = &ast.functions[i];
        // get data
        let (name, statements) = match func {
            ASTNode::Function {
                name,
                arguments: _,
                statements,
                return_type: _,
            } => (
                match &**name {
                    // this is not fucked at all
                    ASTNode::Ident(name) => name,
                    _ => panic!("bad function name"),
                },
                statements,
            ),
            _ => panic!("bad function node"),
        };
        println!("Function: {name}");
        for statement in statements {
            match statement {
                ASTNode::Statement(statement) => match statement {
                    Statement::Print(print_data) => println!("print: {print_data}"),
                },
                ASTNode::Assignment {
                    to,
                    value,
                    taipe,
                    mutuability,
                } => {
                    println!(
                        "assigment to {:?}, {:?}, of type: {:?}, mutuable: {:?}",
                        **to, **value, taipe, mutuability
                    );
                }
                _ => panic!("bad statement"),
            }
        }
        println!();
    }
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
