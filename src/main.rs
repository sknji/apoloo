use std::{env, io};
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use crate::opcode::OpCode;
use crate::bytecodes::Bytecodes;
use crate::vm::{InterpretResult};

mod helpers;
mod value;
mod bytecodes;
mod opcode;
mod debug;
mod token;
mod lexer;
mod parser;
mod codegen;
mod compiler;
mod vm;

fn repl() {
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        vm::interpret(line.unwrap());

        print!("> ");
        io::stdout().flush().unwrap();
    }
}

fn run_file(file_name: &str) {
    let input = read_file(file_name);
    let result = vm::interpret(input);
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
