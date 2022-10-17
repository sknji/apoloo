use std::fmt;
use std::fmt::Formatter;
use crate::value::Value;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
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
            20 => OpCode::OpGetLocal,
            21 => OpCode::OpSetLocal,
            _ => OpCode::OpUnKnown,
        }
    }
}


impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        match op {
            OpCode::OpReturn => 0,
            OpCode::OpConstant => 1,
            OpCode::OpNegate => 2,
            OpCode::OpUnKnown => 99,
            OpCode::OpAdd => 3,
            OpCode::OpSubtract => 4,
            OpCode::OpMultiple => 5,
            OpCode::OpDivide => 6,
            OpCode::OpNil => 7,
            OpCode::OpTrue => 8,
            OpCode::OpFalse => 9,
            OpCode::OpNot => 10,
            OpCode::OpEqual => 11,
            OpCode::OpGreater => 12,
            OpCode::OpLess => 13,
            OpCode::OpPrint => 14,
            OpCode::OpPop => 15,
            OpCode::OpDefineGlobal => 16,
            OpCode::OpGetGlobal => 17,
            OpCode::OpSetGlobal => 18,
            OpCode::OpJumpIfFalse => 19,
            OpCode::OpPopN => 20,
            OpCode::OpGetLocal => 21,
            OpCode::OpSetLocal => 21,
        }
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
            OpCode::OpJumpIfFalse => "OP_SET_GLOBAL",
            OpCode::OpPopN => "OP_POP_N",
        })
    }
}