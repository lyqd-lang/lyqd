use crate::ast::{ASTNode, Ast, Statement};
use shared::BCInstruction;

use shared::Bytecode;

#[derive(Default)]
struct CompilerData {
    indexer: usize,
}

pub struct Compiler;

impl Compiler {
    pub fn from(ast: Ast) -> Bytecode {
        let mut bytecode = Bytecode::default();
        let mut data = CompilerData::default();

        for func in ast.functions {
            Self::translate_function(&mut data, &func, &mut bytecode);
        }
        bytecode
    }

    fn translate_function(data: &mut CompilerData, func: &ASTNode, bytecode: &mut Bytecode) {
        let (name, statements) = match func {
            ASTNode::Function {
                name,
                arguments: _,
                statements,
                return_type: _,
            } => (
                match &**name {
                    ASTNode::Ident(name) => name,
                    _ => panic!("bad token in ast"),
                },
                statements,
            ),
            _ => panic!("bad token in ast"),
        };
        bytecode.labels.insert(name.to_string(), data.indexer);
        for statement_node in statements {
            let statement = match statement_node {
                ASTNode::Statement(statement) => statement,
                _ => panic!("bad token in AST"),
            };

            match statement {
                Statement::Print(text) => {
                    bytecode
                        .instructions
                        .push(BCInstruction::StackPushString(text.to_string()));
                    bytecode.instructions.push(BCInstruction::Print);
                    data.indexer += 2;
                }
            }
        }
        if name == "main" {
            bytecode.instructions.push(BCInstruction::Return);
        }
    }
}
