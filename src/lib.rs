use std::fs::File;
use std::io::Read;
use std::process::exit;
use crate::bytecodes::Bytecodes;
use crate::debug::debug_bytecode;
use crate::vm::{InterpretResult, VM};

pub mod bytecodes;
pub mod codegen;
pub mod compiler;
pub mod debug;
pub mod helpers;
pub mod lexer;
pub mod opcode;
pub mod parser;
pub mod parser_rules;
pub mod localscope;
pub mod token;
pub mod value;
pub mod vm;
pub mod ast;

fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    data
}

pub fn run_file(file_name: &str) {
    let input = read_file(file_name);
    let result = {
        let code = compile(input);
        run(code)
    };
    match result {
        InterpretResult::InterpretOk => {}
        InterpretResult::InterpretCompileError => exit(65),
        InterpretResult::InterpretRuntimeError => exit(70),
    }
}

pub fn compile(input: String) -> Bytecodes {
    let code = compiler::compile(input);

    debug_bytecode(&code, "MAIN");

    code
}

fn run(code: Bytecodes) -> InterpretResult {
    let mut machine = VM::new();
    machine.interpret(code);

    machine.free();

    InterpretResult::InterpretOk
}