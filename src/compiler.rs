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

    pub fn compile(&mut self) -> &Bytecodes {
        self.parser.advance();

        loop {
            let tok = self.parser.curr_tok_type();
            if tok.is(&TokenEof) {
                break;
            }

            self.parser.declaration();
        }

        self.parser.consume(&TokenEof, "Expect end of expression");

        self.end_compiler();

        let bytecodes = &self.parser.codegen.bytecodes;
        if self.parser.had_error {
            debug::debug_bytecode(bytecodes, "MAIN");
        }

        bytecodes
    }
}
