mod chunk;
mod disassembler;
mod value;

use chunk::chunk::*;
use OpCode::*;

use disassembler::debug::disassemble_chunk;

use value::value::Value;

fn main() {
    let mut chunk = Chunk { 
        code: Vec::new(),
    };

    write_chunk(&mut chunk, OP_RETURN, 123);
    add_constant(&mut chunk, 2.7 as Value, 123);

    disassemble_chunk(&chunk, "test_chunk");

    free_chunk(&mut chunk);
}
