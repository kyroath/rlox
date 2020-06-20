#[allow(non_camel_case_types)]
pub enum OpCode {
    OP_RETURN,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
}

pub fn write_chunk(chunk: &mut Chunk, byte: OpCode) {
    chunk.code.push(byte);
}

pub fn free_chunk(chunk: &mut Chunk) {
    chunk.code.clear();
}