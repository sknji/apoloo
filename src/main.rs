use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::{env, io};

use crate::bytecodes::Bytecodes;
use crate::compiler::Compiler;
use crate::debug::debug_bytecode;
use crate::opcode::OpCode;
use crate::vm::{InterpretResult, VM};

mod bytecodes;
mod codegen;
mod compiler;
mod debug;
mod helpers;
mod lexer;
mod opcode;
mod parser;
mod parser_rules;
mod scope;
mod token;
mod value;
mod vm;

fn repl() {
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();

    let mut machine = VM::new();

    for line in stdin.lock().lines() {
        let str = line.unwrap();
        let code = compile(str);

        machine.interpret(code);

        print!("apoloo> ");
        io::stdout().flush().unwrap();
    }

    machine.free();
}

fn run_file(file_name: &str) {
    let input = read_file(file_name);
    let result = {
        let code = compile(input);
        run(code)
    };
    match result {
        InterpretResult::InterpretOk => {},
        InterpretResult::InterpretCompileError => exit(65),
        InterpretResult::InterpretRuntimeError => exit(70),
    }
}

fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    data
}

fn compile(input: String) -> Bytecodes {
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
