use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}
#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Illegal, // Unknown token
    Eof,     // End of file

    Ident,
    Int,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Ident => write!(f, "Ident"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Lparen => write!(f, "("),
            TokenKind::Rparen => write!(f, ")"),
            TokenKind::Lbrace => write!(f, "{{"),
            TokenKind::Rbrace => write!(f, "}}"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Bang => write!(f, "!"),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::Gt => write!(f, ">"),
        }
    }
}

pub fn lookup_ident(identifier: &String) -> TokenKind {
    match identifier.as_str() {
        "fn" => TokenKind::Function,
        "let" => TokenKind::Let,
        _ => TokenKind::Ident,
    }
}
