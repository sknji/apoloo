use std::collections::HashMap;
use std::process::id;

use crate::{Bytecodes, OpCode};
use crate::compiler::Compiler;
use crate::debug::debug_bytecode;
use crate::InterpretResult::InterpretRuntimeError;
use crate::value::{Value, ValueRepr};

pub const STACK_MAX: usize = 256;

pub struct VM {
    code: Bytecodes,
    // instruction pointer
    ip: usize,
    // stack pointer
    stack_top: usize,
    stack: Vec<Value>,

    // global variable store
    globals: HashMap<String, Value>,
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn new() -> VM {
        Self {
            code: Default::default(),
            ip: 0,
            stack: Vec::new(),
            stack_top: 0,
            globals: HashMap::new(),
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
    }

    pub fn interpret(&mut self, code: Bytecodes) -> InterpretResult {
        self.code = code;
        self.reset();
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if self.is_end() {
                return InterpretRuntimeError;
            }

            let op: OpCode = self.read_opcode();

            // self.print_stack(&op, "BEFORE");
            match self.process(&op) {
                None => {}
                Some(e) => return e,
            }
            // self.print_stack(&op, "AFTER");
        }
    }

    fn process(&mut self, op: &OpCode) -> Option<InterpretResult> {
        match op {
            OpCode::OpSetLocal => {
                let slot = self.read_byte();
                let val = self.peek(0);
                self.stack.insert(slot as usize, val.clone())
            }
            OpCode::OpGetLocal => {
                let slot = self.read_byte();
                let val: &Value = self.stack.get(slot as usize).unwrap();
                self.push(val.clone());
            }
            OpCode::OpPopN => {
                let idx = self.read_byte();
                self.pop_n(idx);
            }
            OpCode::OpJump => {
                let offset = self.read_short();
                self.ip += offset;
            }
            OpCode::OpJumpIfFalse => {
                let offset = self.read_short();
                let val = self.peek(0);
                if self.is_falsey(val) {
                    self.ip += offset
                }
            }
            OpCode::OpSetGlobal => {
                let key = self.read_const_str();
                match self.globals.contains_key(&key) {
                    false => {
                        eprintln!("Undefined variable '{}'.", &key);
                        return Some(InterpretRuntimeError);
                    }
                    true => {
                        self.globals.insert(key.into(), self.peek(0).clone());
                    }
                }
            }
            OpCode::OpGetGlobal => {
                let key = self.read_const_str();
                match self.globals.get(&key) {
                    None => {
                        eprintln!("Undefined variable '{}'.", &key);
                        return Some(InterpretRuntimeError);
                    }
                    Some(v) => {
                        self.push(v.clone().into())
                    }
                }
            }
            OpCode::OpDefineGlobal => {
                let key = self.read_const_str();
                let val = self.peek(0).clone();
                self.globals.insert(key, val);
                self.pop();
            }
            OpCode::OpPop => {
                self.pop();
            }
            OpCode::OpPrint => {
                let _ = &self.pop().print();
            }
            OpCode::OpReturn => {
                return Some(InterpretResult::InterpretOk);
            }
            OpCode::OpConstant => {
                let constant = self.read_const();
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
            OpCode::OpNil => self.push(Value(ValueRepr::Nil())),
            OpCode::OpFalse => self.push(Value(ValueRepr::Boolean(false))),
            OpCode::OpTrue => self.push(Value(ValueRepr::Boolean(true))),
            OpCode::OpNot => {
                let value = self.pop();
                self.push(Value(ValueRepr::Boolean(self.is_falsey(&value))))
            }
            OpCode::OpEqual => {
                let r = self.pop();
                let l = self.pop();
                self.push(Value(ValueRepr::Boolean(l == r)))
            }
            OpCode::OpLess => {
                let r = self.pop();
                let l = self.pop();
                self.push(Value(ValueRepr::Boolean(l < r)))
            }
            OpCode::OpGreater => {
                let r = self.pop();
                let l = self.pop();
                self.push(Value(ValueRepr::Boolean(l > r)))
            }
            OpCode::OpUnKnown => {}
        };

        None
    }

    fn push(&mut self, value: Value) {
        self.stack_top += 1;
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack.pop().unwrap()
    }

    fn pop_n(&mut self, idx: u8) {
        self.stack_top -= idx as usize;
        self.stack.drain(self.stack_top..);
    }

    fn peek(&self, distance: usize) -> &Value {
        self.stack.get(self.stack_top - 1 - distance).unwrap()
    }

    fn peek_byte(&mut self, pos: usize) -> u8 {
        let instr = self.code.code.get(self.ip + pos)
            .unwrap();
        return instr.clone();
    }

    fn read_byte(&mut self) -> u8 {
        let instr = self.code.code.get(self.ip)
            .unwrap();
        self.ip += 1;
        return instr.clone();
    }

    fn read_short(&mut self) -> u16 {
        let short = i16(self.peek_byte(1) << 8);
        let short = short | self.peek_byte(2);

        self.ip += 2;

        short
    }

    fn read_opcode(&mut self) -> OpCode {
        self.read_byte().into()
    }

    fn read_const(&mut self) -> Value {
        let idx = self.read_byte();
        self.code.values.get(idx as usize).unwrap().clone()
    }

    fn read_const_str(&mut self) -> String {
        let constant = self.read_const();
        match constant.0 {
            ValueRepr::String(v) => { v }
            _ => "".to_string()
        }
    }

    pub fn free(&mut self) {
        // TODO:
    }

    pub fn is_falsey(&self, value: &Value) -> bool {
        match value.0 {
            ValueRepr::Nil() => true,
            ValueRepr::Boolean(v) => !v,
            _ => false,
        }
    }

    pub fn is_end(&self) -> bool {
        self.ip >= self.code.code.len()
    }

    fn print_stack(&self, op: &OpCode, action: &str) {
        eprintln!("OP: {} {}", &op, action);
        for (k, v) in self.stack.iter().enumerate() {
            eprintln!("\tStack {}: '{}'", k, v);
        }
        eprintln!()
    }
}