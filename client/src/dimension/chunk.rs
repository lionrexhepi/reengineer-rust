use metrohash::MetroHashMap;
pub use shared::dimension::chunk::*;
use shared::util::pos::ChunkPos;


pub struct ClientWorldStorage {
    chunk_map: MetroHashMap<u64, Chunk>,
    request_chunk_handler: dyn Fn(&ChunkPos),
}

impl ChunkStorage for ClientWorldStorage {
    fn is_chunk_cached(&self, pos: &shared::util::pos::ChunkPos) -> bool {
        self.chunk_map.contains_key(&pos.as_long())
    }

    fn get_chunk(&mut self, pos: &shared::util::pos::ChunkPos) -> &Chunk {
        todo!()
    }
}