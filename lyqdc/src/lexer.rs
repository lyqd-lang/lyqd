use std::iter::Peekable;

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Lex {
    #[token("void")]
    Void,

    #[token("mut")]
    Mutuable,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token(";")]
    Semi,

    #[token("=")]
    Eq,

    #[token("print")]
    Print,

    // regex courtesy of spwn, as always
    #[regex(r#"[a-z0-9]*("(?:\\.|[^\\"])*"|'(?:\\.|[^\\'])*')"#, |lex| lex.slice().to_string())]
    StringLiteral(String),

    #[regex("[a-zA-Z_]+", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    IntNumber(isize),

    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    #[error]
    Err,
}

pub struct Lexer<'a>(Peekable<logos::Lexer<'a, Lex>>);

impl Iterator for Lexer<'_> {
    type Item = Lex;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.next();
        println!("Advanced iterator to: {:?}", next);
        next
    }
}

impl<'a> Lexer<'a> {
    pub fn from(text: &'a str) -> Self {
        Self(Lex::lexer(text).peekable())
    }

    pub fn peek(&mut self) -> Option<&Lex> {
        let peek = self.0.peek();
        println!("Peeked: {:?}", peek);
        peek
    }
}
