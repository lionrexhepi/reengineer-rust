use crate::util::chunk_pos::ChunkPos;

use super::chunk::Chunk;

pub trait ChunkLoader {
    fn get_chunk(&self, pos: &ChunkPos) -> Option<Chunk>;
}

pub trait ChunkStorage {
    fn is_chunk_cached(&self, pos: &ChunkPos) -> bool;

    fn get_chunk(&mut self, pos: &ChunkPos) -> anyhow::Result<&Chunk>;
}