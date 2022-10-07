use crate::Bytecodes;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::TokenType::TokenEof;

pub struct Compiler<'a> {
    parser: Parser<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(input: &'a str) -> Self {
        let lex = Lexer::new(&input);
        Self {
            parser: Parser::new(lex),
        }
    }

    pub fn compile(&mut self) -> &Bytecodes {
        self.parser.advance();
        self.parser.expression();
        self.parser.consume(TokenEof, "Expect end of expression");
        self.parser.emit_return();

        &self.parser.codegen.bytecodes
    }
}
