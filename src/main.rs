mod chunk;
mod debug;
mod value;

use chunk::{Chunk, Opcode};
use debug::Disassembler;

fn main() {
    let mut chunk = Chunk::new();

    let constant = Chunk::add_constant(&mut chunk, 1.2);
    Chunk::write_chunk(&mut chunk, Opcode::OpConstant.into(), 122);
    Chunk::write_chunk(&mut chunk, constant, 122);

    Chunk::write_chunk(&mut chunk, Opcode::OpReturn.into(), 122);

    let mut disassembler = Disassembler::new(&chunk, "test");
    disassembler.disassemble_chunk()
}
