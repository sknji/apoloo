use crate::Bytecodes;
use crate::OpCode::OpReturn;

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

    pub(crate) fn emit_return(&mut self) -> usize {
        self.bytecodes.write(OpReturn.into())
    }

    pub(crate) fn emit_constant(&mut self, value: f64) -> usize {
        let addr = self.bytecodes.add_const_val(value);
        // TODO: add max constant check
        self.bytecodes.write2(OpReturn.into(), addr as u8)
    }
}