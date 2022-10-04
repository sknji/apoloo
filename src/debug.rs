use crate::{Bytecodes, OpCode};
use crate::value::print_value;


pub fn debug_bytecode(bytecodes: &Bytecodes, name: &str) {
    print!("=={}==\n", name);

    let mut offset = 0usize;

    loop {
        if offset >= bytecodes.code.len() {
            break;
        }

        offset = debug_instruction(&bytecodes, offset);
    }
}

fn debug_instruction(bytecodes: &Bytecodes, offset: usize) -> usize {
    print!("{offset:0>width$} ", offset = offset, width = 5);

    match bytecodes.code.get(offset) {
        None => {
            print!("Unknown opcode\n");
            offset + 1
        }
        Some(op) => {
            let op: OpCode = op.clone().into();
            match op {
                OpCode::OpReturn => simple_instruction("OP_RETURN", offset),
                OpCode::OpConstant => constant_instruction("OP_CONSTANT", bytecodes, offset),
                OpCode::OpNegate => simple_instruction("OP_NEGATE", offset),
                OpCode::OpAdd => simple_instruction("OP_ADD", offset),
                OpCode::OpSubtract => simple_instruction("OP_SUBTRACT", offset),
                OpCode::OpMultiple => simple_instruction("OP_MULTIPLY", offset),
                OpCode::OpDivide => simple_instruction("OP_DIVIDE", offset),
                OpCode::OpUnKnown => {
                    print!("Unknown opcode {:?} \n", op);
                    offset + 1
                }
            }
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    print!("{}\n", name);
    offset + 1
}

fn constant_instruction(name: &str, bytecodes: &Bytecodes, offset: usize) -> usize {
    let constant = match bytecodes.code.get(offset + 1) {
        Some(constant) => {
            let str_len = name.len();
            let str_pad = if str_len > 16 {
                0
            } else {
                16 - str_len
            };

            print!("{:->width$} {} ", name, num = constant, width = str_pad);
            constant.clone()
        }
        None => {
            print!("unknown value on index {}", offset);
            0 // TODO: fix(me) this will read wrong constant if reached
        }
    };

    match bytecodes.values.get(constant as usize) {
        Some(val) => print_value(val),
        None => print!("Unknown constant value\n"),
    }

    offset + 2
}