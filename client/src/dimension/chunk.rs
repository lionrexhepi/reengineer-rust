use metrohash::MetroHashMap;
use once_cell::sync::Lazy;
pub use shared::dimension::chunk::*;
use shared::util::pos::ChunkPos;
use tokio::sync::mpsc::{ UnboundedSender };

pub struct ClientWorldStorage {
    chunk_map: MetroHashMap<u64, Chunk>,
    request_chunks: UnboundedSender<ChunkPos>,
}

static EMPTY_CHUNK: Lazy<Chunk> = Lazy::new(Chunk::empty);

impl ChunkStorage for ClientWorldStorage {
    fn is_chunk_cached(&self, pos: &ChunkPos) -> bool {
        self.chunk_map.contains_key(&pos.as_long())
    }

    fn get_chunk(&mut self, pos: &ChunkPos) -> &Chunk {
        let id = pos.as_long();

        if let Some(chunk) = self.chunk_map.get(&id) {
            chunk
        } else {
            self.request_chunks.send(pos.clone()).expect("Chunk Storage should not be used if its client loop has gone out of scope/Closed messenger channel!");
            &EMPTY_CHUNK
        }
    }
}

impl ClientWorldStorage {
    fn receive_chunk(&mut self, pos: &ChunkPos, chunk: Chunk) {
        self.chunk_map.insert(pos.as_long(), chunk);
    }

    fn new(request_chunks: UnboundedSender<ChunkPos>) -> Self {
        Self {
            chunk_map: MetroHashMap::default(),
            request_chunks
        }
    }
}