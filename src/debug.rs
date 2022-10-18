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
                OpCode::OpReturn => simple_instruction(&op, offset),
                OpCode::OpConstant => constant_instruction(&op, bytecodes, offset),
                OpCode::OpNegate => simple_instruction(&op, offset),
                OpCode::OpAdd => simple_instruction(&op, offset),
                OpCode::OpSubtract => simple_instruction(&op, offset),
                OpCode::OpMultiple => simple_instruction(&op, offset),
                OpCode::OpDivide => simple_instruction(&op, offset),
                OpCode::OpNil => simple_instruction(&op, offset),
                OpCode::OpTrue => simple_instruction(&op, offset),
                OpCode::OpFalse => simple_instruction(&op, offset),
                OpCode::OpNot => simple_instruction(&op, offset),
                OpCode::OpEqual => simple_instruction(&op, offset),
                OpCode::OpGreater => simple_instruction(&op, offset),
                OpCode::OpLess => simple_instruction(&op, offset),
                OpCode::OpPrint => simple_instruction(&op, offset),
                OpCode::OpPop => simple_instruction(&op, offset),
                OpCode::OpDefineGlobal => constant_instruction(&op, bytecodes, offset),
                OpCode::OpGetGlobal => constant_instruction(&op, bytecodes, offset),
                OpCode::OpSetGlobal => constant_instruction(&op, bytecodes, offset),
                OpCode::OpPopN => constant_instruction(&op, bytecodes, offset),
                OpCode::OpGetLocal => byte_instruction(&op, bytecodes, offset),
                OpCode::OpSetLocal => byte_instruction(&op, bytecodes, offset),
                OpCode::OpJumpIfFalse => jump_instruction(&op, 1,bytecodes, offset),
                OpCode::OpJump => jump_instruction(&op, 1, bytecodes, offset),
                OpCode::OpLoop => jump_instruction(&op, -1, bytecodes, offset),
                OpCode::OpUnKnown => {
                    println!("Unknown opcode {:?}", op);
                    offset + 1
                }
            }
        }
    }
}

fn simple_instruction(op: &OpCode, offset: usize) -> usize {
    print!("{}\n", op);
    offset + 1
}

fn jump_instruction(op: &OpCode, sign: i16, bytecodes: &Bytecodes, offset: usize) -> usize {
    let jump: u16 = (bytecodes.code.get(offset + 1).unwrap().clone() as u16) << 8;
    let jump = jump | bytecodes.code.get(offset + 2).unwrap().clone() as u16;

    let str_pad = calc_str_op_padding(op);

    println!("{:->width$} {} -> {}", op, offset, (offset as i16) + 3 + (sign * (jump as i16)), width = str_pad);

    offset + 3
}

fn byte_instruction(op: &OpCode, bytecodes: &Bytecodes, offset: usize) -> usize {
    let slot = bytecodes.code.get(offset + 1).unwrap();

    let str_pad = calc_str_op_padding(op);
    println!("{:->width$} {}", op, slot, width = str_pad);

    offset + 2
}

fn calc_str_op_padding(op: &OpCode) -> usize {
    let str_len = op.to_string().len();
    let str_pad = if str_len > 16 {
        0
    } else {
        16 - str_len
    };
    str_pad
}

fn constant_instruction(op: &OpCode, bytecodes: &Bytecodes, offset: usize) -> usize {
    let constant = match bytecodes.code.get(offset + 1) {
        Some(constant) => {
            let str_pad = calc_str_op_padding(op);
            print!("{:->width$} {} ", op, constant, width = str_pad);
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