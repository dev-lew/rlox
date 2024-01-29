use std::iter::Peekable;

use crate::chunk::{Chunk, Opcode};
use crate::compiler;
use crate::debug::Disassembler;
use crate::value;
use crate::value::Value;

macro_rules! binary_op {
    ($vm:ident, $op:tt) => {{
	let right = $vm.pop();
	let left = $vm.pop();

	$vm.push(left $op right);
    }};
}

const STACK_MAX: usize = 256;

pub(crate) enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub(crate) struct VM<'a> {
    pub(crate) chunk: &'a Chunk,
    pub(crate) debug_trace_execution: bool,
    ip: Peekable<std::slice::Iter<'a, u8>>,
    stack: Vec<Value>,
}
impl<'a> VM<'a> {
    pub(crate) fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip: chunk.code.iter().peekable(),
            debug_trace_execution: false,
            stack: Vec::with_capacity(STACK_MAX),
        }
    }

    pub(crate) fn new_with_debug(chunk: &'a Chunk, debug_trace_execution: bool) -> Self {
        Self {
            chunk,
            ip: chunk.code.iter().peekable(),
            debug_trace_execution,
            stack: Vec::with_capacity(STACK_MAX),
        }
    }

    pub(crate) fn interpret(&mut self, source: &str) -> InterpretResult {
	compiler::compile(source);
	return InterpretResult::InterpretOk
    }

    pub(crate) fn run(&mut self) -> InterpretResult {
        loop {
            if self.debug_trace_execution {
                print!("          ");

                for slot in self.stack.iter() {
                    print!("[ ");
                    value::print_value(*slot);
                    print!(" ]");
                }
                println!();

                let mut dis = Disassembler::new(self.chunk, "VM_DISASSEMBLER");
                dis.disassemble_instruction((**self.ip.peek().unwrap()).into());
            }

            match Opcode::from(*self.ip.next().unwrap()) {
                Opcode::OpReturn => {
                    value::print_value(self.pop());
                    println!();
                    return InterpretResult::InterpretOk;
                }

                Opcode::OpConstant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }

                Opcode::OpNegate => {
                    let operand = self.pop();
                    self.push(-operand);
                }

                Opcode::OpAdd => self.do_binary_op('+'),
                Opcode::OpSubtract => self.do_binary_op('-'),
                Opcode::OpMultiply => self.do_binary_op('*'),
                Opcode::OpDivide => self.do_binary_op('/'),
                _ => panic!(),
            }
        }
    }

    pub(crate) fn do_binary_op(&mut self, op: char) {
        match op {
            '+' => binary_op!(self, +),
            '-' => binary_op!(self, -),
            '*' => binary_op!(self, *),
            '/' => binary_op!(self, /),
            _ => panic!("Invalid operator"),
        }
    }

    pub(crate) fn read_constant(&mut self) -> Value {
        return self.chunk.constants[*self.ip.next().unwrap() as usize];
    }

    pub(crate) fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub(crate) fn pop(&mut self) -> Value {
        self.stack.pop().expect("The stack is empty!")
    }
}
