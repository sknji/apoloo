use crate::bytecodes::Bytecodes;
use crate::codegen::Codegen;
use crate::debug;
use crate::lexer::Lexer;
use crate::localscope::LocalScope;
use crate::parser::Parser;
use crate::token::TokenType::TokenEof;
use crate::value::ValueRepr;

pub struct Compiler {
    pub function: ValueRepr,
    pub scope: LocalScope,
    pub codegen: Codegen,
}

pub struct CompilerScope {

}

impl Compiler {
    pub fn new() -> Self {
        Self {
            function: Default::default(),
            scope: LocalScope::new(),
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
