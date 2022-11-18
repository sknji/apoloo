use std::{env, io};
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use apoloo::compiler::compile;
use apoloo::vm::VM;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc == 1 {
        repl();
    } else if argc == 2 {
        apoloo::run_file(args.get(1).unwrap());
    } else {
        eprintln!("Usage: apoloo [path]");
        exit(64)
    }
}
