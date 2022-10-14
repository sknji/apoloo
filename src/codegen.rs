use crate::Bytecodes;
use crate::OpCode::{OpConstant, OpReturn};
use crate::value::{Value, ValueRepr};

pub(crate) struct Codegen {
    pub(crate) bytecodes: Bytecodes,
}

impl Codegen {
    pub(crate) fn new() -> Self {
        Codegen { bytecodes: Bytecodes::new() }
    }

    pub(crate) fn emit_byte(&mut self, b: u8) -> usize {
        self.bytecodes.write(b)
    }

    pub(crate) fn emit_bytes(&mut self, bytes: &[u8]) -> usize {
        let mut size: usize = 0;
        for b in bytes {
            size = self.bytecodes.write(*b)
        }
        size
    }

    pub(crate) fn emit_return(&mut self) -> usize {
        self.bytecodes.write(OpReturn.into())
    }

    pub(crate) fn emit_const_f64(&mut self, value: f64) -> usize {
        let addr = self.bytecodes.add_const_val(value);
        // TODO: add max constant check
        self.bytecodes.write2(OpConstant.into(), addr as u8);

        addr
    }

    pub(crate) fn emit_const_string(&mut self, str: String) -> usize {
        let addr = self.bytecodes.add_const(Value(ValueRepr::String(str)));
        // TODO: add max constant check
        self.bytecodes.write2(OpConstant.into(), addr as u8);

        addr
    }
}