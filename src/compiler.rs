use crate::{Bytecodes, debug};
use crate::codegen::Codegen;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::scope::Scope;
use crate::token::TokenType::TokenEof;

pub struct Compiler {
    pub(crate) scope: Scope,
    pub(crate) codegen: Codegen,
}


pub fn compile(input: String) -> Bytecodes {
    let mut parser = Parser::new(Lexer::new(input));
    parser.advance();

    while !parser.match_advance(&TokenEof) {
        parser.declaration();
    }

    parser.consume(&TokenEof, "Expect end of expression");

    parser.emit_return();

    if parser.had_error {
        debug::debug_bytecode(&parser.codegen.bytecodes, "MAIN");
    }

    parser.codegen.bytecodes.to_owned()
}