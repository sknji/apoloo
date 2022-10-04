use crate::opcode::OpCode;
use crate::bytecodes::Bytecodes;
use crate::vm::VM;

mod value;
mod bytecodes;
mod opcode;
mod lexer;
mod debug;
mod vm;

fn main() {
    let mut code: Bytecodes = Bytecodes::new();
    let const_idx1 = code.add_const_val(1.2);
    code.write(OpCode::OpConstant.into());
    code.write(const_idx1 as u8);

    let const_idx2 = code.add_const_val(3.8);
    code.write(OpCode::OpConstant.into());
    code.write(const_idx2 as u8);

    code.write(OpCode::OpAdd.into());

    let const_idx3 = code.add_const_val(5.0);
    code.write(OpCode::OpConstant.into());
    code.write(const_idx3 as u8);

    code.write(OpCode::OpDivide.into());

    code.write(OpCode::OpNegate.into());
    code.write(OpCode::OpReturn.into());

    debug::debug_bytecode(&code, "test chunk");

    let mut machine = VM::new(code);
    machine.interpret();

    machine.free();
}
