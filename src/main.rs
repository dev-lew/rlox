mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use std::env;
use std::process;
use std::fs;
use std::io;
use std::io::Write;

use chunk::{Chunk, Opcode};
use debug::Disassembler;
use vm::{InterpretResult, VM};

fn repl(vm: &mut VM) {
    let mut buf = String::with_capacity(1024);

    print!("> ");
    io::stdout().flush().ok().expect("Failed to flush stdout");
    while let Ok(_) = io::stdin().read_line(&mut buf) {
        vm.interpret(&buf);
        print!("> ");
	io::stdout().flush().ok().expect("Failed to flush stdout");
    }
}

fn run_file(file_path: &str, vm: &mut VM) {
    let source =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Failed to read {}", file_path));

    match vm.interpret(&source) {
	InterpretResult::InterpretCompileError => process::exit(65),
	InterpretResult::InterpretRuntimeError => process::exit(70),
	_ => panic!(),
    }


}
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut dummy_chunk = Chunk::new();
    let mut vm = VM::new(&dummy_chunk);

    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&args[1], &mut vm),
        _ => eprintln!("Usage: clox [path]"),
    }
}
