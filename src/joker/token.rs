//! This file is joker token.rs.
//!
//!
use std::fmt::{Debug, Display};

use super::object::{literal_null, Object};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // ()
    LeftParen,
    RightParen,
    // []
    LeftBracket,
    RightBracket,
    // {}
    LeftBrace,
    RightBrace,
    // , .
    Comma,
    Dot,
    // -  +   ;  / *
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // = ==  ! !=
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    // > >=
    Greater,
    GreaterEqual,
    // < <=
    Less,
    LessEqual,
    // :  ?
    Colon, Question,
    // id string I32 F64
    Identifier,
    Str,
    I32,
    F64,
    // Keyword
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Break,
    Match,
    Struct,
    Eof,
}
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            // -  +   ;  / *
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            // ! != = ==
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            // > >=
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            // < <=
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            // : ? 
            TokenType::Colon => write!(f, ":"),
            TokenType::Question => write!(f, "?"),
            // id Str i32 f64
            TokenType::Identifier => write!(f, "ident"),
            TokenType::Str => write!(f, "str"),
            TokenType::I32 => write!(f, "i32"),
            TokenType::F64 => write!(f, "f64"),
            // Keyword
            TokenType::And => write!(f, "and"),
            TokenType::Class => write!(f, "class"),
            TokenType::Else => write!(f, "else"),
            TokenType::False => write!(f, "false"),
            TokenType::Fun => write!(f, "fun"),
            TokenType::For => write!(f, "for"),
            TokenType::If => write!(f, "if"),
            TokenType::Null => write!(f, "null"),
            TokenType::Or => write!(f, "or"),
            TokenType::Print => write!(f, "print"),
            TokenType::Return => write!(f, "return"),
            TokenType::Super => write!(f, "super"),
            TokenType::This => write!(f, "this"),
            TokenType::True => write!(f, "true"),
            TokenType::Var => write!(f, "var"),
            TokenType::While => write!(f, "while"),
            TokenType::Break => write!(f, "break"),
            TokenType::Match => write!(f, "match"),
            TokenType::Struct => write!(f, "struct"),

            TokenType::Eof => write!(f, "eof"),
        }
    }
}

// 词素和标记（词法单元）
#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Object, line: usize) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
    pub fn eof(line: usize) -> Token {
        Token::new(TokenType::Eof, String::new(), literal_null(), line)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ttype {
            TokenType::Identifier => write!(f, "{}", self.lexeme),
            TokenType::Str => write!(f, "{}", self.lexeme),
            TokenType::I32 => write!(f, "{}", self.lexeme),
            TokenType::F64 => write!(f, "{}", self.lexeme),
            _ => Display::fmt(&self.ttype, f),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token(ttype: {}, lexeme: {}, literal: {}, line: {})",
            self.ttype, self.lexeme, self.literal, self.line,
        )
    }
}

#[cfg(test)]
mod test {}
