use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Add;

use crate::parser::Parser;
use crate::token::TokenType;
use crate::token::TokenType::*;

pub(crate) struct ParseRule {
    pub precedence: ParsePrecedence,
    pub prefix: Option<fn(&mut Parser)>,
    pub infix: Option<fn(&mut Parser)>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash, Debug)]
pub(crate) enum ParsePrecedence {
    PrecedenceNone = 1,
    PrecedenceAssignment = 2 /* = */,
    PrecedenceOr = 3 /* or */,
    PrecedenceAnd = 4 /* and */,
    PrecedenceEquality = 5 /* ==, != */,
    PrecedenceComparison = 6 /* <, >, <=, >= */,
    PrecedenceTerm = 7 /* +, - */,
    PrecedenceFactor = 8 /* *, / */,
    PrecedenceUnary = 9 /* !, - */,
    PrecedenceCall = 10 /* ., () */,

    PrecedencePrimary = 11,
}

impl ParsePrecedence {
    pub fn from_u8(value: u8) -> ParsePrecedence {
        match value {
            1 => ParsePrecedence::PrecedenceNone,
            2 => ParsePrecedence::PrecedenceAssignment,
            3 => ParsePrecedence::PrecedenceOr,
            4 => ParsePrecedence::PrecedenceAnd,
            5 => ParsePrecedence::PrecedenceEquality,
            6 => ParsePrecedence::PrecedenceComparison,
            7 => ParsePrecedence::PrecedenceTerm,
            8 => ParsePrecedence::PrecedenceFactor,
            9 => ParsePrecedence::PrecedenceUnary,
            10 => ParsePrecedence::PrecedenceCall,
            11 => ParsePrecedence::PrecedencePrimary,
            _ => unreachable!()
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            ParsePrecedence::PrecedenceNone => 1,
            ParsePrecedence::PrecedenceAssignment => 2,
            ParsePrecedence::PrecedenceOr => 3,
            ParsePrecedence::PrecedenceAnd => 4,
            ParsePrecedence::PrecedenceEquality => 5,
            ParsePrecedence::PrecedenceComparison => 6,
            ParsePrecedence::PrecedenceTerm => 7,
            ParsePrecedence::PrecedenceFactor => 8,
            ParsePrecedence::PrecedenceUnary => 9,
            ParsePrecedence::PrecedenceCall => 10,
            ParsePrecedence::PrecedencePrimary => 11,
            _ => unreachable!()
        }
    }

    pub fn add(&self, value: u8) -> ParsePrecedence {
        let cur: u8 = self.to_u8();
        ParsePrecedence::from_u8(cur + value)
    }
}
//
// impl Add for ParsePrecedence {
//     type Output = ();
//
//     fn add(self, rhs: Self) -> Self::Output {
//
//     }
// }

impl Parser {
    pub(crate) fn get_rule(&self, tok_type: &TokenType) -> &ParseRule {
        self.parse_rules.get(tok_type).unwrap().clone()
    }

    pub(crate) fn rules(&mut self) -> HashMap<TokenType, ParseRule> {
        let mut h = HashMap::new();
        h.insert(TokenLeftParen, ParseRule { prefix: Some(Parser::grouping), infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenRightParen, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenLeftBrace, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenRightBrace, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenComma, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenDot, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenMinus, ParseRule { prefix: Some(Parser::unary), infix: Some(Parser::binary), precedence: ParsePrecedence::PrecedenceTerm });
        h.insert(TokenPlus, ParseRule { prefix: None, infix: Some(Parser::binary), precedence: ParsePrecedence::PrecedenceTerm });
        h.insert(TokenSemicolon, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenSlash, ParseRule { prefix: None, infix: Some(Parser::binary), precedence: ParsePrecedence::PrecedenceFactor });
        h.insert(TokenStar, ParseRule { prefix: None, infix: Some(Parser::binary), precedence: ParsePrecedence::PrecedenceFactor });
        h.insert(TokenBang, ParseRule { prefix: Some(Parser::unary), infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenBangEqual, ParseRule { prefix: None, infix: Some(Parser::binary), precedence: ParsePrecedence::PrecedenceEquality });
        h.insert(TokenEqual, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenEqualEqual, ParseRule { prefix: None, infix: Some(Parser::binary), precedence: ParsePrecedence::PrecedenceEquality });
        h.insert(TokenGreater, ParseRule { prefix: None, infix: Some(Parser::binary), precedence: ParsePrecedence::PrecedenceComparison });
        h.insert(TokenGreaterEqual, ParseRule { prefix: None, infix: Some(Parser::binary), precedence: ParsePrecedence::PrecedenceComparison });
        h.insert(TokenLess, ParseRule { prefix: None, infix: Some(Parser::binary), precedence: ParsePrecedence::PrecedenceComparison });
        h.insert(TokenLessEqual, ParseRule { prefix: None, infix: Some(Parser::binary), precedence: ParsePrecedence::PrecedenceComparison });
        h.insert(TokenIdentifier, ParseRule { prefix: Some(Parser::variable), infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenString, ParseRule { prefix: Some(Parser::string), infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenNumber, ParseRule { prefix: Some(Parser::number), infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenAnd, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenClass, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenElse, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenFalse, ParseRule { prefix: Some(Parser::literal), infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenFor, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenFun, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenIf, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenNil, ParseRule { prefix: Some(Parser::literal), infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenOr, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenPrint, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenReturn, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenSuper, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenThis, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenTrue, ParseRule { prefix: Some(Parser::literal), infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenVar, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenWhile, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenError, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });
        h.insert(TokenEof, ParseRule { prefix: None, infix: None, precedence: ParsePrecedence::PrecedenceNone });

        h
    }
}