use crate::{Bytecodes, debug};
use crate::codegen::Codegen;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::scope::Scope;
use crate::token::TokenType::TokenEof;
use crate::value::ValueRepr;

pub(crate) struct Compiler {
    pub function: ValueRepr,
    pub function_type: FunctionType,
    pub scope: Scope,
    pub codegen: Codegen,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            function: Default::default(),
            function_type: FunctionType::TypeFunction,
            scope: Scope::new(),
            codegen: Codegen::new(),
        }
    }
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
