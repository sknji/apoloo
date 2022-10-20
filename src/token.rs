use std::fmt;
use std::fmt::Formatter;

use crate::token::TokenType::{
    TokenAnd, TokenClass, TokenElse, TokenFalse, TokenFor, TokenFun, TokenIdentifier, TokenIf, TokenNil, TokenOr,
    TokenPrint, TokenReturn, TokenSuper, TokenThis, TokenTrue, TokenVar, TokenWhile,
};

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) raw: String,
    pub(crate) line: i64,
    pub(crate) col: i64,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum TokenType {
    // Single-character tokens.
    TokenLeftParen,
    TokenRightParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenMinus,
    TokenPlus,
    TokenSemicolon,
    TokenSlash,
    TokenStar,

    // One or two character tokens.
    TokenBang,
    TokenBangEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenGreater,
    TokenGreaterEqual,
    TokenLess,
    TokenLessEqual,

    // Literals.
    TokenIdentifier,
    TokenString,
    TokenNumber,

    // Keywords.
    TokenAnd,
    TokenClass,
    TokenElse,
    TokenFalse,
    TokenFor,
    TokenFun,
    TokenIf,
    TokenNil,
    TokenOr,
    TokenPrint,
    TokenReturn,
    TokenSuper,
    TokenThis,
    TokenTrue,
    TokenVar,
    TokenWhile,

    TokenError,
    TokenEof,
}

impl Token {
    pub fn new(type_: TokenType, raw: &str, line: i64, col: i64) -> Self {
        Self { token_type: type_, raw: raw.into(), line, col }
    }

    pub fn is(&self, tok_type: TokenType) -> bool {
        self.token_type == tok_type
    }
}

pub fn kw_type_from_str(token_type: &str) -> TokenType {
    match token_type {
        "and" => TokenAnd,
        "class" => TokenClass,
        "else" => TokenElse,
        "if" => TokenIf,
        "nil" => TokenNil,
        "or" => TokenOr,
        "print" => TokenPrint,
        "return" => TokenReturn,
        "super" => TokenSuper,
        "var" => TokenVar,
        "while" => TokenWhile,
        "false" => TokenFalse,
        "for" => TokenFor,
        "fun" => TokenFun,
        "this" => TokenThis,
        "true" => TokenTrue,
        _ => TokenIdentifier,
    }
}

impl TokenType {
    pub(crate) fn is(&self, rhs: &TokenType) -> bool {
        self == rhs
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}] {:?} {}", self.line, self.col, self.token_type, self.raw)
    }
}
