use crate::ast::{ASTNode, Ast, Statement};
use shared::BCInstruction;

use shared::Bytecode;
use shared::Register;

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

            Self::translate_statement(data, statement, bytecode);
        }
        if name == "main" {
            bytecode.instructions.push(BCInstruction::Return);
        }
    }

    fn translate_statement(
        data: &mut CompilerData,
        statement: &Statement,
        bytecode: &mut Bytecode,
    ) {
        match statement {
            Statement::PrintConstant(data) => {
                bytecode
                    .instructions
                    .push(BCInstruction::RegisterConstString(
                        data.to_string(),
                        Register::R1,
                    ));
                bytecode.instructions.push(BCInstruction::Print)
            }
            Statement::PrintVariable(ident) => {
                let ident = match &**ident {
                    ASTNode::Ident(ident) => ident,
                    _ => panic!("bad ast node"),
                };
                bytecode.instructions.push(BCInstruction::RegisterVariable(
                    ident.to_string(),
                    Register::R1,
                ))
            }
            Statement::Assignment {
                to,
                value,
                taipe,
                mutuability,
            } => {
                if taipe.is_some() {
                    // create a variable
                    let name = match &**to {
                        ASTNode::Ident(name) => name.to_string(),
                        _ => panic!("bad astnode"),
                    };
                    let value = match &**value {
                        ASTNode::Number(n) => shared::BCValue::IntPtr(*n),
                        _ => panic!("bad node"),
                    };
                    let u_type = taipe.unwrap();
                    bytecode.instructions.push(BCInstruction::CreateVariable(
                        name.to_string(),
                        u_type,
                        *mutuability,
                    ));
                    bytecode
                        .instructions
                        .push(BCInstruction::SetVariable(name, u_type, value));
                } else {
                    // reassign
                }
            }
        }
    }
}
