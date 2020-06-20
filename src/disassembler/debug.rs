use crate::chunk::chunk::*;
use OpCode::*;

use crate::value::value::{Value, print_value};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let code = &chunk.code;
    let mut line: usize = 0;

    for (i, code) in code.iter().enumerate() {
        if code.0 == line {
            print!("   | ");
        } else {
            print!("{num:>pad$} ", num=code.0, pad=4);
            line = code.0;
        }

        print!("{index:>0pad$} ", index=i, pad=4);

        disassemble_instruction(code);
    }
}

fn disassemble_instruction(code: &(usize, OpCode)) {
    match code.1 {
        OP_CONSTANT(value) => constant_instruction("OP_CONSTANT", value),
        OP_RETURN => simple_instruction("OP_RETURN")
    }
}

fn simple_instruction(name: &str) {
    println!("{}", name)
}

fn constant_instruction(name: &str, value: Value) {
    print!("{name:<pad$} ", name=name, pad=12);
    print_value(&value);
    println!();
}