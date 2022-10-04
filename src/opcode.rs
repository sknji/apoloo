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
        }
    }
}