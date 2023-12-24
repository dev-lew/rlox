use crate::value::Value;

#[derive(Copy, Clone)]
pub(crate) enum Opcode {
    OpReturn,
    OpConstant,
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Opcode::OpReturn,
            1 => Opcode::OpConstant,
            _ => panic!(),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::OpReturn => 0,
            Opcode::OpConstant => 1,
            _ => panic!(),
        }
    }
}

#[derive(Default)]
pub(crate) struct Chunk {
    pub(crate) code: Vec<Opcode>,
    pub(crate) constants: Vec<Value>,
    pub(crate) lines: Vec<i32>,
}

impl Chunk {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn write_chunk(chunk: &mut Chunk, byte: u8, line: i32) {
        chunk.code.push(byte.into());
        chunk.lines.push(line);
    }

    pub(crate) fn add_constant(chunk: &mut Chunk, value: Value) -> u8 {
        chunk.constants.push(value);
        (chunk.constants.len() - 1).try_into().unwrap()
    }
}
