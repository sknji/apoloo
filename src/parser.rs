use crate::Bytecodes;
use crate::codegen::Codegen;
use crate::lexer::Lexer;
use crate::OpCode::{OpNegate, OpReturn};
use crate::token::{Token, TokenType};
use crate::token::TokenType::{TokenEof, TokenMinus, TokenRightParen};

pub(crate) struct ParseRule {}

pub(crate) struct Parser<'a> {
    lex: Lexer<'a>,
    pub(crate) codegen: Codegen,
    curr_tok: Option<Token>,
    prev_tok: Option<Token>,
}

pub(crate) enum Precedence {
    PrecedenceNone,
    PrecedenceAssignment,
    PrecedenceOr,
    PrecedenceAnd,
    PrecedenceEquality,
    PrecedenceComparison,
    PrecedenceTerm,
    PrecedenceFactor,
    PrecedenceUnary,
    PrecedenceCall,
    PrecedencePrimary,
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
        false
    }

    pub(crate) fn advance(&mut self) {
        self.prev_tok = self.curr_tok.take();

        loop {
            self.curr_tok = Some(self.lex.scan_next());
            match &self.curr_tok {
                None => break,
                Some(t) if !&t.is(TokenEof) => break,
                _ => {}
            }

            println!("{:?}", self.curr_tok.as_ref().unwrap())
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

        eprintln!("Error!! {}", err_msg);
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

    pub(crate) fn grouping(&mut self) {
        self.expression();
        self.consume(TokenRightParen, "Expect ')' after expression.")
    }

    pub(crate) fn unary(&mut self) {
        let tok_type = match &self.prev_tok.clone() {
            Some(tok) => Some(&tok.token_type),
            None => None,
            _ => None
        }.unwrap().clone();

        self.expression();

        if tok_type.is(&TokenMinus)  {
            self.codegen.emit_byte(OpNegate.into());
        }
    }
}
