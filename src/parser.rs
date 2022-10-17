use std::collections::HashMap;

use crate::Bytecodes;
use crate::codegen::Codegen;
use crate::lexer::Lexer;
use crate::OpCode::*;
use crate::parser_rules::{ParsePrecedence, ParseRule};
use crate::parser_rules::ParsePrecedence::*;
use crate::scope::Scope;
use crate::token::{Token, TokenType};
use crate::token::TokenType::*;

pub(crate) struct Parser {
    pub(crate) lex: Lexer,
    pub(crate) codegen: Codegen,
    curr_tok: Option<Token>,
    prev_tok: Option<Token>,
    pub(crate) parse_rules: HashMap<TokenType, ParseRule>,
    pub(crate) had_error: bool,
    pub(crate) scope: Scope,
}

impl<'a> Parser {
    pub(crate) fn new(lex: Lexer) -> Parser {
        let mut p = Parser {
            lex,
            curr_tok: None,
            prev_tok: None,
            codegen: Codegen::new(),
            had_error: false,
            parse_rules: HashMap::new(),
            scope: Scope::new(),
        };

        p.parse_rules = p.rules();

        p
    }

    fn curr_is(&self, tok_type: &TokenType) -> bool {
        self.curr_tok_type().is(tok_type)
    }

    pub(crate) fn advance(&mut self) {
        self.prev_tok = self.curr_tok.clone();

        loop {
            self.curr_tok = Some(self.lex.scan_next());
            match &self.curr_tok {
                None => break,
                Some(t) if !t.is(TokenError) => return,
                _ => {}
            }

            self.error_at_curr("Error at advance current")
        }
    }

    pub(crate) fn emit_return(&mut self) -> usize {
        self.codegen.emit_return()
    }

    pub(crate) fn consume(&mut self, tok_type: &TokenType, err_msg: &str) {
        if self.curr_is(tok_type) {
            self.advance();
            return;
        }

        self.error_at_curr(err_msg);
    }

    pub(crate) fn match_advance(&mut self, tok_type: &TokenType) -> bool {
        if !self.curr_is(tok_type) {
            return false;
        }
        self.advance();

        true
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
        self.consume(&TokenRightParen, "Expect ')' after expression.")
    }

    pub(crate) fn expression(&mut self) {
        self.parse(&PrecedenceAssignment);
    }

    pub(crate) fn block(&mut self) {
        while !self.curr_is(&TokenRightBrace) &&
            !self.curr_is(&TokenEof) {
            self.declaration();
        }

        self.consume(&TokenRightBrace, "Expected '}' after block");
    }

    pub(crate) fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name");
        if self.match_advance(&TokenEqual) {
            self.expression();
        } else {
            self.codegen.emit_byte(OpNil.into());
        }

        self.consume(&TokenSemicolon, "Expect ';' after variable declaration.");

        self.define_var(global as u8);
    }

    pub(crate) fn expression_statement(&mut self) {
        self.expression();
        self.consume(&TokenSemicolon, "Expect ';' after value.");
        self.codegen.emit_byte(OpPop.into());
    }

    pub(crate) fn if_statement(&mut self) {
        self.consume(&TokenLeftParen, "Expect '(' after 'if'.");
        self.expression();
        self.consume(&TokenRightParen, "Expect ')' after condition.");

        let then_jump = self.codegen.emit_jump(OpJumpIfFalse.into());
        self.codegen.emit_byte(OpPop.into());
        self.statement();

        let else_jump = self.codegen.emit_jump(OpJump.into());

        self.codegen.patch_jump(then_jump);
        self.codegen.emit_byte(OpPop.into());

        if self.match_advance(&TokenElse) {
            self.codegen.patch_jump(else_jump);
            self.statement();
        }
    }

    pub(crate) fn print_statement(&mut self) {
        self.expression();
        self.consume(&TokenSemicolon, "Expect ';' after value.");
        self.codegen.emit_byte(OpPrint.into());
    }

    pub(crate) fn declaration(&mut self) {
        if self.match_advance(&TokenVar) {
            self.var_declaration();
        } else {
            self.statement();
        }
    }

    pub(crate) fn statement(&mut self) {
        if self.match_advance(&TokenPrint) {
            self.print_statement();
        } else if self.match_advance(&TokenIf) {
            self.if_statement();
        } else if self.match_advance(&TokenLeftBrace) {
            self.scope.begin_scope();
            self.block();
            self.codegen.emit_bytes(&[OpPopN.into(), self.scope.end_scope()]);
        } else {
            self.expression_statement();
        }
    }

    pub(crate) fn number(&mut self) {
        let value = self.prev_tok.as_ref();
        let value: f64 = match value {
            None => 0.0,
            Some(val) => {
                val.raw.parse().unwrap_or(0.0)
            }
        };

        self.codegen.emit_const_f64(value);
    }

    pub(crate) fn string(&mut self) {
        let value = self.prev_tok.as_ref();
        let value = match value {
            None => "",
            Some(val) => val.raw
                .trim_matches('"'),
        };

        self.codegen.emit_const_string(value.to_owned());
    }

    pub(crate) fn named_variable(&mut self) {
        let mut get_op = OpUnKnown;
        let mut set_op = OpUnKnown;
        let mut arg = 0;

        let name = &self.prev_tok.as_ref().unwrap().raw;

        match self.scope.resolve_local(name) {
            None => {
                arg = self.ident_const() as u8;
                get_op = OpGetGlobal;
                set_op = OpSetGlobal;
            }
            Some(v) => {
                arg = v;
                get_op = OpGetLocal;
                set_op = OpSetLocal;
            }
        }

        match self.match_advance(&TokenEqual) {
            true => {
                self.expression();
                self.codegen.emit_bytes(&[set_op.into(), arg]);
            }
            false => {
                self.codegen.emit_bytes(&[get_op.into(), arg]);
            }
        }
    }

    pub(crate) fn variable(&mut self) {
        self.named_variable()
    }

    pub(crate) fn literal(&mut self) {
        let prev_tok_type = self.prev_tok_type();
        match prev_tok_type {
            TokenFalse => self.codegen.emit_byte(OpFalse.into()),
            TokenTrue => self.codegen.emit_byte(OpTrue.into()),
            TokenNil => self.codegen.emit_byte(OpNil.into()),
            TokenPrint => self.codegen.emit_byte(OpPrint.into()),
            _ => return,
        };
    }

    pub(crate) fn unary(&mut self) {
        let prev_tok_type = self.prev_tok_type();

        self.parse(&PrecedenceUnary);

        match prev_tok_type {
            TokenMinus => self.codegen.emit_byte(OpNegate.into()),
            TokenBang => self.codegen.emit_byte(OpNot.into()),
            _ => return,
        };
    }

    pub(crate) fn binary(&mut self) {
        let prev_tok_type = self.prev_tok_type();
        let rule = self.get_rule(&prev_tok_type);

        let precedence = &rule.precedence.add(1);

        self.parse(precedence);

        match prev_tok_type {
            TokenPlus => self.codegen.emit_byte(OpAdd.into()),
            TokenMinus => self.codegen.emit_byte(OpSubtract.into()),
            TokenStar => self.codegen.emit_byte(OpMultiple.into()),
            TokenSlash => self.codegen.emit_byte(OpDivide.into()),
            TokenBangEqual => self.codegen.emit_bytes(&[OpEqual.into(), OpNot.into()]),
            TokenEqualEqual => self.codegen.emit_byte(OpEqual.into()),
            TokenGreater => self.codegen.emit_byte(OpGreater.into()),
            TokenGreaterEqual => self.codegen.emit_bytes(&[OpLess.into(), OpNot.into()]),
            TokenLess => self.codegen.emit_byte(OpLess.into()),
            TokenLessEqual => self.codegen.emit_byte(OpDivide.into()),
            _ => return,
        };
    }

    pub(crate) fn error_at_curr(&mut self, msg: &str) {
        self.error_at(&self.curr_tok.clone(), msg)
    }

    pub(crate) fn error(&mut self, msg: &str) {
        self.error_at(&self.prev_tok.clone(), msg)
    }

    pub(crate) fn error_at(&mut self, token: &Option<Token>, msg: &str) {
        let tok = token.as_ref().unwrap();

        eprint!("[line {}] Error", tok.line);
        match tok.token_type {
            TokenEof => eprint!(" at end"),
            TokenError => {}
            _ => {
                eprint!(" at {}", tok.raw)
            }
        }

        eprintln!(": {}", msg);

        self.had_error = true
    }

    pub(crate) fn parse(&mut self, precedence: &ParsePrecedence) {
        self.advance();

        let tok_type: &TokenType = &self.prev_tok_type();
        let rule: &ParseRule = self.get_rule(tok_type);

        let prefix_rule = rule.prefix;

        match prefix_rule {
            None => { self.error("Expected expression") }
            Some(p) => { p(self) }
        }

        loop {
            let curr_tok_type: &TokenType = &self.curr_tok_type();
            let curr_rule: &ParseRule = self.get_rule(curr_tok_type);

            let curr_precedence: &ParsePrecedence = &curr_rule.precedence;

            if precedence > curr_precedence {
                break;
            }

            self.advance();

            let prev_tok_type: &TokenType = &self.prev_tok_type();
            let infix: &ParseRule = self.get_rule(prev_tok_type);
            let infix_rule = infix.infix;

            match infix_rule {
                None => { self.error("Expected expression") }
                Some(i) => { i(self) }
            }
        }
    }

    pub(crate) fn parse_variable(&mut self, msg: &str) -> usize {
        self.consume(&TokenIdentifier, msg);

        self.declare_var();

        if self.scope.scope_depth > 0 {
            return 0;
        }

        self.ident_const()
    }

    pub(crate) fn ident_const(&mut self) -> usize {
        let value = match self.prev_tok.as_ref() {
            None => "",
            Some(val) => &val.raw,
        };

        self.codegen.emit_const_string(value.to_owned())
    }

    pub(crate) fn add_local(&mut self, tok: &Token) {
        if self.scope.local_count >= i8::MAX {
            // TODO: error too many variables in function
            return;
        }

        if self.scope.contains(tok.raw.as_ref()) {
            // TODO: Already a variable with this name in this scope.
            eprintln!("Already a variable with this name in this scope.");
            return;
        }

        self.scope.add_local(&tok.raw)
    }

    pub(crate) fn declare_var(&mut self) {
        if self.scope.scope_depth == 0 {
            return;
        }

        let prev_tok = self.prev_tok.as_ref().unwrap().clone();
        self.add_local(&prev_tok);
    }

    pub(crate) fn define_var(&mut self, global: u8) {
        if self.scope.scope_depth > 0 {
            return;
        }

        self.codegen.emit_bytes(&[OpDefineGlobal.into(), global]);
    }
}


#[cfg(test)]
mod tests {}
