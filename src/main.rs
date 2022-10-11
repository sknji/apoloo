use std::{env, io};
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use crate::bytecodes::Bytecodes;
use crate::compiler::Compiler;
use crate::debug::debug_bytecode;
use crate::opcode::OpCode;
use crate::vm::{InterpretResult, VM};

mod helpers;
mod value;
mod bytecodes;
mod opcode;
mod debug;
mod token;
mod lexer;
mod parser_rules;
mod parser;
mod codegen;
mod compiler;
mod vm;

fn repl() {
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let str = line.unwrap();
        interpret(str);

        print!("> ");
        io::stdout().flush().unwrap();
    }
}

fn run_file(file_name: &str) {
    let input = read_file(file_name);
    let result = interpret(input);
    match result {
        InterpretResult::InterpretOk => {}
        InterpretResult::InterpretCompileError => { exit(65) }
        InterpretResult::InterpretRuntimeError => { exit(70) }
    }
}

fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    data
}

fn interpret(input: String) -> InterpretResult {
    let mut compiler = Compiler::new(input);

    let code = compiler.compile();

    // debug_bytecode(code, "MAIN");

    let mut machine = VM::new(code);
    machine.interpret();

    machine.free();

    InterpretResult::InterpretOk
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc == 1 {
        repl();
    } else if argc == 2 {
        run_file(args.get(1).unwrap());
    } else {
        eprintln!("Usage: apoloo [path]");
        exit(64)
    }
}
