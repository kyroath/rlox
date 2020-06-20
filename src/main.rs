mod chunk;
mod disassembler;

use chunk::chunk::*;
use OpCode::*;

use disassembler::debug::disassemble_chunk;

fn main() {
    let mut chunk = Chunk { code: Vec::new() };
    write_chunk(&mut chunk, OP_RETURN);

    disassemble_chunk(&chunk, "main");

    free_chunk(&mut chunk);
}
