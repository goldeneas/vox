use chunk::Chunk;
use types::{ChunkHeight, ChunkRow, CoordinateBound};

mod chunk; 
mod types;

pub fn run() {
    let chunk = Chunk::new();
    let chunk_height = ChunkHeight::parse(0).unwrap();
    chunk.generate_quads(chunk_height);
}
