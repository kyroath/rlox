use crate::chunk::chunk::*;
use OpCode::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let code = &chunk.code;

    for (i, code) in code.iter().enumerate() {
        disassemble_instruction(code, i);
    }
}

fn disassemble_instruction(code: &OpCode, index: usize) {
    print!("{index:>0pad$} ", index=index, pad=4);

    match code {
        OP_RETURN => simple_instruction("OP_RETURN")
    }
}

fn simple_instruction(name: &str) {
    println!("{}", name)
}