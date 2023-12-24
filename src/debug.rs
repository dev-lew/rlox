use crate::chunk::{Chunk, Opcode};
use crate::value;

pub(crate) struct Disassembler<'a> {
    chunk: &'a Chunk,
    name: &'a str,
    offset: usize,
}

impl<'a> Disassembler<'a> {
    pub(crate) fn new(chunk: &'a Chunk, name: &'a str) -> Self {
        Self {
            chunk,
            name,
            offset: 0usize,
        }
    }

    pub(crate) fn disassemble_chunk(&mut self) {
        println!("== {} ==", self.name);

        while self.offset < self.chunk.code.len() {
            self.disassemble_instruction(self.chunk.code[self.offset])
        }
    }

    fn disassemble_instruction(&mut self, instruction: Opcode) {
        print!("{:04} ", self.offset);

        if self.offset > 0 && self.chunk.lines[self.offset] == self.chunk.lines[self.offset] {
            print!("   | ");
        } else {
            print!("{:4} ", self.chunk.lines[self.offset])
        }

        match instruction {
            Opcode::OpReturn => self.simple_instruction("OP_RETURN"),
            Opcode::OpConstant => self.constant_instruction("OP_CONSTANT"),
        }
    }

    fn simple_instruction(&mut self, name: &str) {
        println!("{}", name);
        self.offset += 1;
    }

    fn constant_instruction(&mut self, name: &str) {
        let constant_offset: u8 = self.chunk.code[self.offset + 1].into();

        print!("{:<-16} {:4} '", name, constant_offset);
        value::print_value(self.chunk.constants[constant_offset as usize]);
        println!("'");

        self.offset += 2;
    }
}
