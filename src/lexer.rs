use std::str;
use crate::token::{Token, TokenType};
use crate::token::TokenType::*;

pub struct Lexer<'a> {
    len: usize,
    start: usize,
    current: usize,
    line: i64,
    input: &'a [u8],
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Self {
            input: input.as_ref(),
            len: input.len(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn peek_token(&self) -> Token {
        self.make_token(TokenEof)
    }

    fn make_token(&self, type_: TokenType) -> Token {
        Token::new(
            type_,
            str::from_utf8(&self.input[self.start..self.current])
                .unwrap(),
            self.line,
        )
    }

    fn error_token(&self, msg: &str) -> Token {
        Token::new(TokenError, msg, self.line)
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        *self.input.get(self.current - 1).unwrap()
    }

    fn matches(&mut self, ch: char) -> bool {
        if self.is_end() {
            return false;
        }

        let next = *self.input.get(self.current + 1).unwrap();
        if ch != (next as char) {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        self.input.get(self.current + 1).unwrap() as char
    }

    fn skip_whitespaces(&mut self) {
        loop {
            let ch = self.peek();
            match ch {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {}
                _ => {}
            }
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.len
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();

        self.start = self.current;
        if self.is_end() {
            return Some(self.make_token(TokenEof));
        }

        let ch = self.advance();

        let token = match ch as char {
            '(' => self.make_token(TokenLeftParen),
            ')' => self.make_token(TokenRightParen),
            '{' => self.make_token(TokenLeftBrace),
            '}' => self.make_token(TokenRightBrace),
            ';' => self.make_token(TokenSemicolon),
            ',' => self.make_token(TokenComma),
            '.' => self.make_token(TokenDot),
            '-' => self.make_token(TokenMinus),
            '+' => self.make_token(TokenPlus),
            '/' => self.make_token(TokenSlash),
            '*' => self.make_token(TokenStar),
            '!' => {
                let tok_type = match self.matches('=') {
                    true => TokenBangEqual,
                    false => TokenBang,
                };
                self.make_token(tok_type)
            }
            '=' => {
                let tok_type = match self.matches('=') {
                    true => TokenEqualEqual,
                    false => TokenEqual,
                };
                self.make_token(tok_type)
            }
            '<' => {
                let tok_type = match self.matches('=') {
                    true => TokenLessEqual,
                    false => TokenEqual,
                };
                self.make_token(tok_type)
            }
            '>' => {
                let tok_type = match self.matches('=') {
                    true => TokenGreaterEqual,
                    false => TokenGreater,
                };
                self.make_token(tok_type)
            }
            _ => self.error_token(format!("unexpected character {ch}").as_ref())
        };

        Some(token)
    }
}