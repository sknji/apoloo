use crate::Bytecodes;
use crate::codegen::Codegen;
use crate::lexer::Lexer;
use crate::OpCode::{OpAdd, OpDivide, OpMultiple, OpNegate, OpReturn, OpSubtract};
use crate::token::{Token, TokenType};
use crate::token::TokenType::{TokenEof, TokenMinus, TokenRightParen};

pub(crate) struct ParseRule {}

pub(crate) struct Parser<'a> {
    lex: Lexer<'a>,
    pub(crate) codegen: Codegen,
    curr_tok: Option<Token>,
    prev_tok: Option<Token>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(lex: Lexer<'a>) -> Self {
        Self {
            lex,
            curr_tok: None,
            prev_tok: None,
            codegen: Codegen::new(),
        }
    }

    fn curr_is(&self, tok_type: TokenType) -> bool {
        self.curr_tok_type().is(&tok_type)
    }

    pub(crate) fn advance(&mut self) {
        self.prev_tok = self.curr_tok.take();

        loop {
            self.curr_tok = Some(self.lex.scan_next());
            println!("{:?}", self.curr_tok.as_ref().unwrap());
            match &self.curr_tok {
                None => break,
                Some(t) if t.is(TokenEof) => break,
                _ => {}
            }
        }
    }

    pub(crate) fn emit_return(&mut self) -> usize {
        self.codegen.emit_return()
    }

    pub(crate) fn consume(&mut self, tok_type: TokenType, err_msg: &str) {
        if self.curr_is(tok_type) {
            self.advance();
            return;
        }

        eprintln!("Error!! {err_msg}");
    }

    pub(crate) fn prev_tok_type(&self) -> TokenType {
        match &self.prev_tok.clone() {
            Some(tok) => Some(&tok.token_type),
            None => None,
            _ => None
        }.unwrap().clone()
    }

    pub(crate) fn curr_tok_type(&self) -> TokenType {
        match &self.curr_tok.clone() {
            Some(tok) => Some(&tok.token_type),
            None => None,
            _ => None
        }.unwrap().clone()
    }

    pub(crate) fn grouping(&mut self) {
        self.expression();
        self.consume(TokenRightParen, "Expect ')' after expression.")
    }

    pub(crate) fn expression(&mut self) {}

    pub(crate) fn number(&mut self) {
        let value = self.prev_tok.as_ref();
        let value: f64 = match value {
            None => 0.0,
            Some(val) => {
                val.raw.parse().unwrap_or(0.0)
            }
        };

        self.codegen.emit_constant(value);
    }

    pub(crate) fn unary(&mut self) {
        let prev_tok_type = self.prev_tok_type();

        self.expression();

        if prev_tok_type.is(&TokenMinus) {
            self.codegen.emit_byte(OpNegate.into());
        }
    }

    pub(crate) fn binary(&mut self) {
        let prev_tok_type = self.prev_tok_type();

        match prev_tok_type {
            TokenType::TokenPlus => { self.codegen.emit_byte(OpAdd.into()); }
            TokenType::TokenMinus => { self.codegen.emit_byte(OpSubtract.into()); }
            TokenType::TokenStar => { self.codegen.emit_byte(OpMultiple.into()); }
            TokenType::TokenSlash => { self.codegen.emit_byte(OpDivide.into()); }
            _ => return,
        }
    }
}
