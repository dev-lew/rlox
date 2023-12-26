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

pub(crate) struct LineEncoding {
    pub(crate) count: u8,
    pub(crate) line: i32,
}

impl LineEncoding {
    pub(crate) fn new(line: i32) -> Self {
	Self {
	    count: 1,
	    line,
	}
    }
}

#[derive(Default)]
pub(crate) struct Chunk {
    pub(crate) code: Vec<Opcode>,
    pub(crate) constants: Vec<Value>,
    pub(crate) lines: Vec<LineEncoding>,
}

impl Chunk {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn write_chunk(chunk: &mut Chunk, byte: u8, line: i32) {
        chunk.code.push(byte.into());

	if let Some(prev) = chunk.lines.last_mut() {
	    if prev.line == line {
		prev.count += 1;
	    } else {
		chunk.lines.push(LineEncoding::new(line));
	    }
	} else {
	    chunk.lines.push(LineEncoding::new(line));
	}
    }

    pub(crate) fn add_constant(chunk: &mut Chunk, value: Value) -> u8 {
        chunk.constants.push(value);
        (chunk.constants.len() - 1).try_into().unwrap()
    }

    pub(crate) fn get_line(index: usize, lines: &Vec<LineEncoding>) -> i32 {
	let mut total = 0;

	for line_encoding in lines.iter() {
	    total += line_encoding.count;

	    if index <= total.into() {
		return line_encoding.line
	    }
	}

	panic!()
    }
}
