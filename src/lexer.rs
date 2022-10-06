use std::str;
use crate::helpers::*;
use crate::token::*;
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

    fn advance(&mut self) -> char {
        self.current += 1;
        *self.input.get(self.current - 1).unwrap() as char
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

    fn peek(&self, pos: Option<usize>) -> Option<char> {
        if self.is_end() {
            return None;
        }

        let p = pos.unwrap_or(1);
        match self.input.get(self.current + p) {
            None => None,
            Some(val) => Some(*val as char)
        }
    }

    fn peek_is(&self, ch: char, pos: Option<usize>) -> bool {
        let c = self.peek(pos);
        c.unwrap_or_default() == ch
    }

    fn peek_is_match(&self, pos: Option<usize>, f: fn(ch: char) -> bool) -> bool {
        let c = self.peek(pos);
        f(c.unwrap_or_default())
    }

    fn peek1(&self) -> Option<char> {
        return self.peek(None);
    }

    fn peek1_is(&self, ch: char) -> bool {
        return self.peek_is(ch, None);
    }

    fn peek1_is_match(&self, f: fn(ch: char) -> bool) -> bool {
        self.peek_is_match(None, f)
    }

    fn skip_whitespaces(&mut self) {
        loop {
            let ch = match self.peek1() {
                None => return,
                Some(c) => c
            };

            match ch {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' if self.peek1_is('/') => {
                    while !self.peek1_is('\n') && !self.is_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn number(&mut self) -> Token {
        while self.peek1_is_match(is_digit) {
            self.advance();
        }

        if self.peek1_is('.') && self.peek_is_match(Some(2), is_digit) {
            self.advance();

            while self.peek1_is_match(is_digit) {
                self.advance();
            }
        }

        self.make_token(TokenNumber)
    }

    fn string(&mut self) -> Token {
        while !self.is_end() && !self.peek1_is('"') {
            if self.peek1_is('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            return self.error_token("unterminated string.");
        }

        self.advance();

        self.make_token(TokenString)
    }

    fn ident(&mut self) -> Token {
        while self.peek1_is_match(is_alpha_num) {
            self.advance();
        }

        let ident_type = self.ident_type();
        self.make_token(ident_type)
    }

    fn ident_type(&mut self) -> TokenType {
        let str = &self.input[self.start..self.current];
        let str = str::from_utf8(&str).unwrap();
        kw_type_from_str(str)
    }

    fn is_end(&self) -> bool {
        self.current >= self.len
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();
        println!("Next!!! ");
        self.start = self.current;
        if self.is_end() {
            return Some(self.make_token(TokenEof));
        }

        let ch = self.advance();
        if is_digit(ch) { return Some(self.number()); }
        if is_alpha(ch) { return Some(self.ident()); }

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
            '"' => self.string(),
            _ => self.error_token(format!("unexpected character {ch}").as_ref())
        };

        Some(token)
    }
}