use std::collections::HashMap;

use crate::codegen::Codegen;
use crate::lexer::Lexer;
use crate::localscope::LocalScope;
use crate::opcode::OpCode::*;
use crate::parser_rules::{ParsePrecedence, ParseRule};
use crate::parser_rules::ParsePrecedence::*;
use crate::token::{Token, TokenType};
use crate::token::TokenType::*;

pub struct Parser {
    pub lex: Lexer,
    pub codegen: Codegen,
    curr_tok: Option<Token>,
    prev_tok: Option<Token>,
    pub parse_rules: HashMap<TokenType, ParseRule>,
    pub had_error: bool,
    pub scope: LocalScope,
}

impl<'a> Parser {
    pub fn new(lex: Lexer) -> Parser {
        let mut p = Parser {
            lex,
            curr_tok: None,
            prev_tok: None,
            codegen: Codegen::new(),
            had_error: false,
            parse_rules: HashMap::new(),
            scope: LocalScope::new(),
        };

        p.parse_rules = p.rules();

        p
    }

    fn curr_is(&self, tok_type: &TokenType) -> bool {
        self.curr_tok_type().is(tok_type)
    }

    pub fn advance(&mut self) {
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

    pub fn emit_return(&mut self) -> usize {
        self.codegen.emit_return()
    }

    pub fn consume(&mut self, tok_type: &TokenType, err_msg: &str) {
        if self.curr_is(tok_type) {
            self.advance();
            return;
        }

        self.error_at_curr(err_msg);
    }

    pub fn match_advance(&mut self, tok_type: &TokenType) -> bool {
        if !self.curr_is(tok_type) {
            return false;
        }
        self.advance();

        true
    }

    pub fn prev_tok_type(&self) -> TokenType {
        match &self.prev_tok.clone() {
            Some(tok) => Some(&tok.token_type),
            None => None,
            _ => None,
        }
            .unwrap()
            .clone()
    }

    pub fn curr_tok_type(&self) -> TokenType {
        match &self.curr_tok.clone() {
            Some(tok) => Some(&tok.token_type),
            None => None,
            _ => None,
        }
            .unwrap()
            .clone()
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.consume(&TokenRightParen, "Expect ')' after expression.")
    }

    pub fn expression(&mut self) {
        self.parse(&PrecedenceAssignment);
    }

    pub fn block(&mut self) {
        while !self.curr_is(&TokenRightBrace) && !self.curr_is(&TokenEof) {
            self.declaration();
        }

        self.consume(&TokenRightBrace, "Expected '}' after block");
    }

    pub fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name");
        if self.match_advance(&TokenEqual) {
            self.expression();
        } else {
            self.codegen.emit_op(OpNil);
        }

        self.consume(&TokenSemicolon, "Expect ';' after variable declaration.");

        self.define_var(global as u8);
    }

    pub fn expression_statement(&mut self) {
        self.expression();
        self.consume(&TokenSemicolon, "Expect ';' after value.");
        self.codegen.emit_op(OpPop);
    }

    pub fn if_statement(&mut self) {
        self.consume(&TokenLeftParen, "Expect '(' after 'if'.");
        self.expression();
        self.consume(&TokenRightParen, "Expect ')' after condition.");

        let then_jump = self.codegen.emit_jump(OpJumpIfFalse);
        self.codegen.emit_op(OpPop);
        self.statement();

        let else_jump = self.codegen.emit_jump(OpJump);

        self.codegen.patch_jump(then_jump);
        self.codegen.emit_op(OpPop);

        if self.match_advance(&TokenElse) {
            self.statement();
        }

        self.codegen.patch_jump(else_jump);
    }

    pub fn print_statement(&mut self) {
        self.expression();
        self.consume(&TokenSemicolon, "Expect ';' after value.");
        self.codegen.emit_op(OpPrint);
    }

    pub fn for_statement(&mut self) {
        self.scope.begin_scope();
        self.consume(&TokenLeftParen, "Expect '(' after 'for'.");

        if self.match_advance(&TokenSemicolon) {
            // No initializer
        } else if self.match_advance(&TokenVar) {
            self.var_declaration();
        } else {
            self.expression_statement();
        }

        let mut loop_start = self.codegen.bytecodes.code_count;
        let mut exit_jump = None;

        if !self.match_advance(&TokenSemicolon) {
            self.expression();
            self.consume(&TokenSemicolon, "Expect ';' after loop condition.");

            exit_jump = Some(self.codegen.emit_jump(OpJumpIfFalse));
            self.codegen.emit_op(OpPop);
        }

        if !self.match_advance(&TokenRightParen) {
            let body_jump = self.codegen.emit_jump(OpJump);
            let increment_start = self.codegen.bytecodes.code_count;
            self.expression();
            self.codegen.emit_op(OpPop);
            self.consume(&TokenRightParen, "Expect ')' after for clauses.");

            self.codegen.emit_loop(loop_start);
            loop_start = increment_start;
            self.codegen.patch_jump(body_jump);
        }

        self.statement();
        self.codegen.emit_loop(loop_start);

        if let Some(i) = exit_jump {
            self.codegen.patch_jump(i);
            self.codegen.emit_op(OpPop);
        }

        self.scope.end_scope();
    }

    pub fn while_statement(&mut self) {
        let loop_start = self.codegen.bytecodes.code_count;
        self.consume(&TokenLeftParen, "Expect '(' after 'while'.");
        self.expression();
        self.consume(&TokenRightParen, "Expect ')' after condition.");

        let exit_jump = self.codegen.emit_jump(OpJumpIfFalse);
        self.codegen.emit_op(OpPop);

        self.statement();

        self.codegen.emit_loop(loop_start);

        self.codegen.patch_jump(exit_jump);
        self.codegen.emit_op(OpPop);
    }

    pub fn declaration(&mut self) {
        if self.match_advance(&TokenVar) {
            self.var_declaration();
        } else {
            self.statement();
        }
    }

    pub fn statement(&mut self) {
        if self.match_advance(&TokenPrint) {
            self.print_statement();
        } else if self.match_advance(&TokenIf) {
            self.if_statement();
        } else if self.match_advance(&TokenWhile) {
            self.while_statement();
        } else if self.match_advance(&TokenFor) {
            self.for_statement();
        } else if self.match_advance(&TokenLeftBrace) {
            self.scope.begin_scope();
            self.block();
            self.codegen.emit_op_operand(OpPopN, self.scope.end_scope());
        } else {
            self.expression_statement();
        }
    }

    pub fn number(&mut self) {
        let value = self.prev_tok.as_ref();
        let value: f64 = match value {
            None => 0.0,
            Some(val) => val.raw.parse().unwrap_or(0.0),
        };

        self.codegen.emit_const_f64(value);
    }

    pub fn string(&mut self) {
        let value = self.prev_tok.as_ref();
        let value = match value {
            None => "",
            Some(val) => val.raw.trim_matches('"'),
        };

        self.codegen.emit_const_string(value.to_owned());
    }

    pub fn named_variable(&mut self) {
        let name = &self.prev_tok.as_ref().unwrap().raw;

        let (get_op, set_op, arg) = match self.scope.resolve_local(name) {
            None => (OpGetGlobal, OpSetGlobal, self.ident_const() as u8),
            Some(v) => (OpGetLocal, OpSetLocal, v),
        };

        match self.match_advance(&TokenEqual) {
            true => {
                self.expression();
                self.codegen.emit_op_operand(set_op, arg);
            }
            false => {
                self.codegen.emit_op_operand(get_op, arg);
            }
        }
    }

    pub fn variable(&mut self) {
        self.named_variable()
    }

    pub fn literal(&mut self) {
        let prev_tok_type = self.prev_tok_type();
        match prev_tok_type {
            TokenFalse => self.codegen.emit_op(OpFalse),
            TokenTrue => self.codegen.emit_op(OpTrue),
            TokenNil => self.codegen.emit_op(OpNil),
            TokenPrint => self.codegen.emit_op(OpPrint),
            _ => return,
        };
    }

    pub fn unary(&mut self) {
        let prev_tok_type = self.prev_tok_type();

        self.parse(&PrecedenceUnary);

        match prev_tok_type {
            TokenMinus => self.codegen.emit_op(OpNegate),
            TokenBang => self.codegen.emit_op(OpNot),
            _ => return,
        };
    }

    pub fn binary(&mut self) {
        let prev_tok_type = self.prev_tok_type();
        let rule = self.get_rule(&prev_tok_type);

        let precedence = &rule.precedence.add(1);

        self.parse(precedence);

        match prev_tok_type {
            TokenPlus => self.codegen.emit_op(OpAdd),
            TokenMinus => self.codegen.emit_op(OpSubtract),
            TokenStar => self.codegen.emit_op(OpMultiple),
            TokenSlash => self.codegen.emit_op(OpDivide),
            TokenBangEqual => self.codegen.emit_op2(OpEqual, OpNot),
            TokenEqualEqual => self.codegen.emit_op(OpEqual),
            TokenGreater => self.codegen.emit_op(OpGreater),
            TokenGreaterEqual => self.codegen.emit_op2(OpLess, OpNot),
            TokenLess => self.codegen.emit_op(OpLess),
            TokenLessEqual => self.codegen.emit_op2(OpGreater, OpNot),
            _ => return,
        };
    }

    pub fn error_at_curr(&mut self, msg: &str) {
        self.error_at(&self.curr_tok.clone(), msg)
    }

    pub fn error(&mut self, msg: &str) {
        self.error_at(&self.prev_tok.clone(), msg)
    }

    pub fn error_at(&mut self, token: &Option<Token>, msg: &str) {
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

    pub fn parse(&mut self, precedence: &ParsePrecedence) {
        self.advance();

        let tok_type: &TokenType = &self.prev_tok_type();
        let rule: &ParseRule = self.get_rule(tok_type);

        let prefix_rule = rule.prefix;

        match prefix_rule {
            None => self.error("Expected expression"),
            Some(p) => p(self),
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
                None => self.error("Expected expression"),
                Some(i) => i(self),
            }
        }
    }

    pub fn parse_variable(&mut self, msg: &str) -> usize {
        self.consume(&TokenIdentifier, msg);

        self.declare_var();

        if self.scope.scope_depth > 0 {
            return 0;
        }

        self.ident_const()
    }

    pub fn ident_const(&mut self) -> usize {
        let value = match self.prev_tok.as_ref() {
            None => "",
            Some(val) => &val.raw,
        };

        self.codegen.emit_const_string(value.to_owned())
    }

    pub fn add_local(&mut self, tok: &Token) {
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

    pub fn declare_var(&mut self) {
        if self.scope.scope_depth == 0 {
            return;
        }

        let prev_tok = self.prev_tok.as_ref().unwrap().clone();
        self.add_local(&prev_tok);
    }

    pub fn define_var(&mut self, global: u8) {
        if self.scope.scope_depth > 0 {
            return;
        }

        self.codegen.emit_bytes(&[OpDefineGlobal.into(), global]);
    }

    pub fn and_(&mut self) {
        let end_jump = self.codegen.emit_op(OpJumpIfFalse);
        self.codegen.emit_op(OpPop);

        self.parse(&PrecedenceAnd);

        self.codegen.patch_jump(end_jump);
    }

    pub fn or_(&mut self) {
        let else_jump = self.codegen.emit_op(OpJumpIfFalse);
        let end_jump = self.codegen.emit_op(OpJump);

        self.codegen.patch_jump(else_jump);
        self.codegen.emit_op(OpPop);

        self.parse(&PrecedenceOr);

        self.codegen.patch_jump(end_jump);
    }
}

#[cfg(test)]
mod tests {}
