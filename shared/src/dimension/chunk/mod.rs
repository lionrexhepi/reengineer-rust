
use metrohash::MetroHashMap;

use crate::util::pos::ChunkPos;

pub struct Chunk;

pub trait ChunkProvider {
    fn get_chunk(&self, pos: &ChunkPos) -> Option<Chunk>;
}

pub trait ChunkStorage {
    fn has_chunk(&self, pos: &ChunkPos) -> bool;

    fn get_chunk(&self, pos: &ChunkPos) -> anyhow::Result<Chunk>;

    
}

