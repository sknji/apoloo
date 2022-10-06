use crate::lexer::Lexer;
use crate::token::TokenType::TokenEof;

pub fn compile(input: String) {
    let lex = Lexer::new(&input);

    println!("Compiler is running");

    for token in lex {
        if token.is(TokenEof){
            break
        }
        println!("{:?}", token)
    }
}