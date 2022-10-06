use crate::Bytecodes;
use crate::lexer::Lexer;
use crate::token::TokenType::TokenEof;

struct Compiler {
    bytecodes: Bytecodes,
}

impl Compiler {
    pub fn new() -> Self {
        Self { bytecodes: Bytecodes::new() }
    }
}

pub fn compile(input: String) -> Compiler {
    let compiler = Compiler::new();
    let lex = Lexer::new(&input);

    for token in lex {
        println!("{:?}", token)
    }

    compiler
}
