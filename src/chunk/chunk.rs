use crate::value::value::*;

#[allow(non_camel_case_types)]
pub enum OpCode {
    OP_CONSTANT(Value),
    OP_RETURN,
}

pub struct Chunk {
    pub code: Vec<(usize, OpCode)>,
}

pub fn write_chunk(chunk: &mut Chunk, byte: OpCode, line: usize) {
    chunk.code.push((line, byte));
}

pub fn free_chunk(chunk: &mut Chunk) {
    chunk.code.clear();
}

pub fn add_constant(chunk: &mut Chunk, value: Value, line: usize) {
    write_chunk(chunk, OpCode::OP_CONSTANT(value), line);
}