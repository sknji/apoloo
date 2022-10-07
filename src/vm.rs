use crate::{Bytecodes, compiler, OpCode};
use crate::compiler::Compiler;
use crate::value::{print_value, Value};

pub const STACK_MAX: usize = 256;

pub struct VM<'b> {
    code: &'b  Bytecodes,
    // instruction pointer
    ip: usize,
    // stack pointer
    stack_top: usize,
    stack: Vec<Value>,
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl<'b> VM<'b> {
    pub fn new(code: &'b Bytecodes) -> VM {
        Self { code, ip: 0, stack: Vec::new(), stack_top: 0 }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        loop {
            if self.is_end() {
                return InterpretResult::InterpretRuntimeError;
            }

            let op: OpCode = self.read_opcode();

            match op {
                OpCode::OpReturn => {
                    print_value(&self.pop());
                    println!();
                    return InterpretResult::InterpretOk;
                }
                OpCode::OpConstant => {
                    let constant = self.read_constant();
                    self.push(constant)
                }
                OpCode::OpNegate => {
                    let val = -self.pop();
                    self.push(val)
                }
                OpCode::OpAdd => {
                    let r = self.pop();
                    let l = self.pop();
                    self.push(l + r)
                }
                OpCode::OpSubtract => {
                    let r = self.pop();
                    let l = self.pop();
                    self.push(l - r)
                }
                OpCode::OpMultiple => {
                    let r = self.pop();
                    let l = self.pop();
                    self.push(l * r)
                }
                OpCode::OpDivide => {
                    let r = self.pop();
                    let l = self.pop();
                    self.push(l / r)
                }
                OpCode::OpUnKnown => {}
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack_top += 1;
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack.pop().unwrap()
    }

    fn read_byte(&mut self) -> u8 {
        let instr = self.code.code.get(self.ip)
            .unwrap();
        self.ip += 1;
        return instr.clone();
    }

    fn read_opcode(&mut self) -> OpCode {
        self.read_byte().into()
    }

    fn read_constant(&mut self) -> Value {
        let idx = self.read_byte();
        self.code.values.get(idx as usize).unwrap().clone()
    }

    pub fn free(&mut self) {
        // TODO:
    }

    pub fn is_end(&self) -> bool {
        self.ip >= self.code.code.len()
    }
}

pub fn interpret(input: String) -> InterpretResult {
    let mut compiler = Compiler::new(&input);

    let code = compiler.compile();

    let mut machine = VM::new(code);
    machine.interpret();

    machine.free();
    InterpretResult::InterpretOk
}