use crate::token::TokenType::{TokenAnd, TokenClass, TokenElse, TokenFalse, TokenFor, TokenFun, TokenIdentifier, TokenIf, TokenNil, TokenOr, TokenPrint, TokenReturn, TokenSuper, TokenThis, TokenTrue, TokenVar, TokenWhile};

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    raw: String,
    line: i64,
}

#[derive(Eq, PartialEq, Debug)]
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
    pub fn new(type_: TokenType, raw: &str, line: i64) -> Self {
        Self { token_type: type_, raw: raw.into(), line }
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