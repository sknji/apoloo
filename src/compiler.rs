use crate::{Bytecodes, debug};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::TokenType::TokenEof;

pub struct Compiler {
    parser: Parser,
}

impl Compiler {
    pub fn new(input: String) -> Self {
        Self { parser: Parser::new(Lexer::new(input)) }
    }

    pub fn end_compiler(&mut self) {
        self.parser.emit_return();
    }

    pub fn compile(&mut self) -> Bytecodes {
        self.parser.advance();

        while !self.parser.match_advance(&TokenEof) {
            self.parser.declaration();
        }

        self.parser.consume(&TokenEof, "Expect end of expression");

        self.end_compiler();

        if self.parser.had_error {
            debug::debug_bytecode(&self.parser.codegen.bytecodes, "MAIN");
        }

        self.parser.codegen.bytecodes.to_owned()
    }
}
