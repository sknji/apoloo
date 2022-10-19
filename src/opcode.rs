use std::fmt;
use std::fmt::Formatter;

use crate::value::Value;

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
    OpNegate = 2,
    OpAdd = 3,
    OpSubtract = 4,
    OpMultiple = 5,
    OpDivide = 6,
    OpNil = 7,
    OpTrue = 8,
    OpFalse = 9,
    OpNot = 10,
    OpEqual = 11,
    OpGreater = 12,
    OpLess = 13,
    OpPrint = 14,
    OpPop = 15,
    OpDefineGlobal = 16,
    OpGetGlobal = 17,
    OpSetGlobal = 18,
    OpJumpIfFalse = 19,
    OpPopN = 20,
    OpGetLocal = 21,
    OpSetLocal = 22,
    OpJump = 23,
    OpLoop = 24,
    OpUnKnown = 99,
}

impl From<u8> for OpCode {
    fn from(op: u8) -> Self {
        match op {
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            2 => OpCode::OpNegate,
            3 => OpCode::OpAdd,
            4 => OpCode::OpSubtract,
            5 => OpCode::OpMultiple,
            6 => OpCode::OpDivide,
            7 => OpCode::OpNil,
            8 => OpCode::OpTrue,
            9 => OpCode::OpFalse,
            10 => OpCode::OpNot,
            11 => OpCode::OpEqual,
            12 => OpCode::OpGreater,
            13 => OpCode::OpLess,
            14 => OpCode::OpPrint,
            15 => OpCode::OpPop,
            16 => OpCode::OpDefineGlobal,
            17 => OpCode::OpGetGlobal,
            18 => OpCode::OpSetGlobal,
            19 => OpCode::OpJumpIfFalse,
            20 => OpCode::OpPopN,
            21 => OpCode::OpGetLocal,
            22 => OpCode::OpSetLocal,
            23 => OpCode::OpJump,
            24 => OpCode::OpLoop,
            99 => OpCode::OpUnKnown,
            o => {
                eprintln!("Opcode '{}' not found. Compiler error", o);
                OpCode::OpUnKnown
            },
        }
    }
}


impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        op as u8
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            OpCode::OpReturn => "OP_RETURN",
            OpCode::OpConstant => "OP_CONSTANT",
            OpCode::OpNegate => "OP_NEGATE",
            OpCode::OpUnKnown => "OP_UNKNOWN",
            OpCode::OpAdd => "OP_ADD",
            OpCode::OpSubtract => "OP_SUBTRACT",
            OpCode::OpMultiple => "OP_MULTIPLE",
            OpCode::OpDivide => "OP_DIVIDE",
            OpCode::OpNil => "OP_NIL",
            OpCode::OpTrue => "OP_TRUE",
            OpCode::OpFalse => "OP_FALSE",
            OpCode::OpNot => "OP_NOT",
            OpCode::OpEqual => "OP_EQUAL",
            OpCode::OpGreater => "OP_GREATER",
            OpCode::OpLess => "OP_LESS",
            OpCode::OpPrint => "OP_PRINT",
            OpCode::OpPop => "OP_POP",
            OpCode::OpDefineGlobal => "OP_DEFINE_GLOBAL",
            OpCode::OpGetGlobal => "OP_GET_GLOBAL",
            OpCode::OpSetGlobal => "OP_SET_GLOBAL",
            OpCode::OpGetLocal => "OP_GET_LOCAL",
            OpCode::OpSetLocal => "OP_SET_LOCAL",
            OpCode::OpJumpIfFalse => "OP_JUMP_IF_FALSE",
            OpCode::OpPopN => "OP_POP_N",
            OpCode::OpJump => "OP_JUMP",
            OpCode::OpLoop => "OP_LOOP",
        })
    }
}