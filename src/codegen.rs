use crate::bytecodes::Bytecodes;
use crate::opcode::OpCode;
use crate::opcode::OpCode::{OpConstant, OpReturn};
use crate::value::{Value, ValueRepr};

pub struct Codegen {
    pub bytecodes: Bytecodes,
}

impl Codegen {
    pub fn new() -> Self {
        Codegen { bytecodes: Bytecodes::new() }
    }

    pub fn emit_op_operand2(&mut self, op: OpCode, o1: u8, o2: u8) -> usize {
        self.emit_op(op);
        self.emit_byte(o1);
        self.emit_byte(o2)
    }

    pub fn emit_op_operand(&mut self, op: OpCode, o: u8) -> usize {
        self.emit_op(op);
        self.emit_byte(o)
    }

    pub fn emit_op2(&mut self, op1: OpCode, op2: OpCode) -> usize {
        self.emit_op(op1);
        self.emit_op(op2)
    }

    pub fn emit_op(&mut self, op: OpCode) -> usize {
        self.emit_byte(op.into())
    }

    pub fn emit_byte(&mut self, b: u8) -> usize {
        self.bytecodes.write(b)
    }

    pub fn emit_bytes(&mut self, bytes: &[u8]) -> usize {
        let mut size: usize = 0;
        for b in bytes {
            size = self.bytecodes.write(*b)
        }
        size
    }

    pub fn emit_return(&mut self) -> usize {
        self.bytecodes.write(OpReturn.into())
    }

    pub fn emit_const_f64(&mut self, value: f64) -> usize {
        let addr = self.bytecodes.add_const_val(value);
        // TODO: add max constant check
        self.bytecodes.write2(OpConstant.into(), addr as u8);

        addr
    }

    pub fn emit_const_string(&mut self, str: String) -> usize {
        let addr = self.bytecodes.add_const(Value(ValueRepr::String(str)));
        // TODO: add max constant check
        self.bytecodes.write2(OpConstant.into(), addr as u8);

        addr
    }

    pub fn emit_jump(&mut self, op: OpCode) -> usize {
        self.emit_op(op);
        self.emit_bytes(&[0xFF, 0xFF]);
        self.bytecodes.code_count - 2
    }

    pub fn patch_jump(&mut self, offset: usize) {
        let jump = self.bytecodes.code_count - offset - 2;
        if (jump as u16) > u16::MAX {
            eprintln!("Too much code to jump over.");
        }

        self.bytecodes.code[offset as usize] = ((jump >> 8) & 0xFF) as u8;
        self.bytecodes.code[(offset + 1) as usize] = (jump & 0xFF) as u8;
    }

    pub fn emit_loop(&mut self, start: usize) {
        self.emit_op(OpCode::OpLoop);

        let offset = (self.bytecodes.code_count - start + 2) as u16;
        if offset > u16::MAX {
            eprintln!("Loop body too large.");
        }

        self.emit_byte((offset >> 8 & 0xFF) as u8);
        self.emit_byte((offset & 0xFF) as u8);
    }
}
