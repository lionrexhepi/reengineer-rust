use metrohash::MetroHashMap;

use crate::util::pos::ChunkPos;

pub struct Chunk;

#[derive(Debug)]
pub enum ChunkProviderError {
    IOError(std::io::Error),
    ChunkNotGeneratedError,
}

pub trait ChunkProvider {
    fn get_chunk(&self, pos: &ChunkPos) -> Result<&Chunk, ChunkProviderError>;
}

pub trait ChunkStorage {
    fn is_chunk_cached(&self, pos: &ChunkPos) -> bool;

    fn get_chunk(&mut self, pos: &ChunkPos) -> &Chunk;
}