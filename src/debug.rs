use crate::{Bytecodes, OpCode};

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
            println!("Unknown opcode");
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
                OpCode::OpNil => simple_instruction("OP_NIL", offset),
                OpCode::OpTrue => simple_instruction("OP_TRUE", offset),
                OpCode::OpFalse => simple_instruction("OP_FALSE", offset),
                OpCode::OpNot => simple_instruction("OP_NOT", offset),
                OpCode::OpEqual => simple_instruction("OP_EQUAL", offset),
                OpCode::OpGreater => simple_instruction("OP_GREATER", offset),
                OpCode::OpLess => simple_instruction("OP_LESS", offset),
                OpCode::OpPrint => simple_instruction("OP_PRINT", offset),
                OpCode::OpUnKnown => {
                    println!("Unknown opcode {:?}", op);
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
        Some(val) => val.print(),
        None => println!("Unknown constant value"),
    }

    offset + 2
}