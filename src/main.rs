mod chunk;
mod debug;
mod value;
mod vm;

use chunk::{Chunk, Opcode};
use debug::Disassembler;
use vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    let dummy_chunk = Chunk::new();

    let mut constant = Chunk::add_constant(&mut chunk, 1.2);
    // Chunk::write_chunk(&mut chunk, Opcode::OpConstantLong.into(), 122);
    Chunk::write_chunk(&mut chunk, Opcode::OpConstant.into(), 123);
    Chunk::write_chunk(&mut chunk, constant, 122);

    constant = Chunk::add_constant(&mut chunk, 3.4);
    Chunk::write_chunk(&mut chunk, Opcode::OpConstant.into(), 123);
    Chunk::write_chunk(&mut chunk, constant, 123);
    Chunk::write_chunk(&mut chunk, Opcode::OpAdd.into(), 123);

    constant = Chunk::add_constant(&mut chunk, 5.6);
    Chunk::write_chunk(&mut chunk, Opcode::OpConstant.into(), 123);
    Chunk::write_chunk(&mut chunk, constant, 123);
    Chunk::write_chunk(&mut chunk, Opcode::OpDivide.into(), 123);

    Chunk::write_chunk(&mut chunk, Opcode::OpNegate.into(), 123);
    Chunk::write_chunk(&mut chunk, Opcode::OpReturn.into(), 123);

    let mut disassembler = Disassembler::new(&chunk, "test");
    disassembler.disassemble_chunk();

    let mut vm = VM::new_with_debug(&dummy_chunk, true);
    vm.interpret(&chunk);

}
