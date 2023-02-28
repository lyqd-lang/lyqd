use crate::lexer::{Lex, Lexer};

#[derive(Debug)]
pub enum Type {
    Void,
    IntPtr,
    // Custom {
    //
    // }
}

#[derive(Debug)]
pub enum ASTNode {
    Function {
        name: Box<ASTNode>,
        arguments: Vec<ASTNode>,
        statements: Vec<ASTNode>,
        return_type: Type,
    },
    Ident(String),
    FunctionArgument(Box<ASTNode>, Type),
    Statement(Statement),
    Assignment {
        to: Box<ASTNode>,
        value: Box<ASTNode>,
        taipe: Option<Type>,
        mutuability: bool,
    },
    Number(isize),
    Root,
}

#[derive(Debug)]
pub enum Statement {
    Print(String),
    // TODO: FnCall(String)
}

#[derive(Default)]
pub struct Ast {
    pub functions: Vec<ASTNode>,
}

impl Ast {
    pub fn build(lexer: &mut Lexer) -> Self {
        let mut ast = Ast::default();
        // rn we only have functions at the top index, so try and parse that
        while lexer.peek().is_some() {
            ast.functions.push(Self::parse_function(lexer));
        }
        ast
    }
    pub fn parse_function(lexer: &mut Lexer) -> ASTNode {
        // get the type
        let return_type = Self::parse_type(lexer).unwrap();
        let name = match lexer.next() {
            Some(Lex::Ident(name)) => name,
            _ => panic!("unexpected token"),
        };
        let arguments = Self::parse_arguments(lexer);
        let statements = Self::parse_code_block(lexer);
        ASTNode::Function {
            name: Box::new(ASTNode::Ident(name)),
            statements,
            arguments,
            return_type,
        }
    }

    pub fn parse_type(lexer: &mut Lexer) -> Option<Type> {
        let taipe = lexer.peek();
        let ast_type = match taipe {
            Some(Lex::Void) => Some(Type::Void),
            Some(Lex::Ident(name)) => match name.as_str() {
                "nint" => Some(Type::IntPtr),
                _ => None,
            },
            _ => panic!("unexpected type"),
        };
        if ast_type.is_some() {
            lexer.next();
        }
        ast_type
    }

    // TODO: this literally returns an empty vector lmao
    fn parse_arguments(lexer: &mut Lexer) -> Vec<ASTNode> {
        let args = vec![];
        assert!(lexer.next() == Some(Lex::LParen));
        // TODO: actually parse args lmaoo
        while lexer.next() != Some(Lex::RParen) {} // TODO: this deadlocks if theres no matching RParen
        args
    }

    fn parse_code_block(lexer: &mut Lexer) -> Vec<ASTNode> {
        let mut statements = vec![];
        assert!(lexer.next() == Some(Lex::LBrace));
        while lexer.peek() != Some(&Lex::RBrace) {
            statements.push(Self::parse_statement(lexer));
        }
        lexer.next(); // advance the iterator to prevent a bug
        statements
    }

    fn parse_statement(lexer: &mut Lexer) -> ASTNode {
        // get first instruction
        return match lexer.peek() {
            Some(Lex::Print) => Self::parse_print_statement(lexer),
            Some(Lex::Mutuable) => Self::parse_assignment(lexer),
            Some(Lex::Ident(_)) => Self::parse_assignment(lexer),
            _ => panic!("unexpected token"),
        };
    }

    fn parse_assignment(lexer: &mut Lexer) -> ASTNode {
        let mutuability = matches!(lexer.peek(), Some(Lex::Mutuable));
        if mutuability {
            lexer.next();
        }
        let taipe = Self::parse_type(lexer);
        let ident = ASTNode::Ident(match lexer.next() {
            Some(Lex::Ident(name)) => name,
            _ => panic!("bad token"),
        });
        // next should be an = sign
        println!("{:?}", lexer.peek());
        assert!(matches!(lexer.next(), Some(Lex::Eq)));
        let value = Self::parse_value(lexer);
        assert!(matches!(lexer.next(), Some(Lex::Semi)));
        ASTNode::Assignment {
            to: Box::new(ident),
            value: Box::new(value),
            mutuability,
            taipe,
        }
    }

    fn parse_value(lexer: &mut Lexer) -> ASTNode {
        match lexer.next() {
            Some(Lex::IntNumber(num)) => ASTNode::Number(num),
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_print_statement(lexer: &mut Lexer) -> ASTNode {
        lexer.next(); // we know there's a print kw because of how parse_statement does shit
        let text_token = lexer.next();
        let text = match text_token {
            Some(Lex::StringLiteral(text)) => text,
            _ => panic!("unexpected token"),
        };
        assert!(lexer.next() == Some(Lex::Semi));
        ASTNode::Statement(Statement::Print(text))
    }
}
