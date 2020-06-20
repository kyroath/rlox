pub enum OpCode {
    OP_RETURN,
}

pub struct Chunk {
    code: Vec<u8>,
}

pub fn writeChunk(chunk: &mut Chunk, byte: u8) {
    chunk.code.push(byte);
}

pub fn freeChunk(chunk: &mut Chunk) {
    chunk.code.clear();
}