use anyhow::anyhow;
use metrohash::MetroHashMap;
use once_cell::sync::Lazy;
pub use shared::dimension::chunk::*;
use shared::{util::chunk_pos::ChunkPos, dimension::storage::ChunkStorage};
use tokio::sync::mpsc::{ UnboundedSender };

pub struct ClientWorldStorage {
    chunk_map: MetroHashMap<u64, Chunk>,
    request_chunks: UnboundedSender<ChunkPos>,
}

static EMPTY_CHUNK: Lazy<Chunk> = Lazy::new(Chunk::empty);

pub enum ClientChunkStorageError {
    RequestChannelClosed,
}

impl From<ClientChunkStorageError> for anyhow::Error {
    fn from(value: ClientChunkStorageError) -> Self {
        match value {
            ClientChunkStorageError::RequestChannelClosed =>
                anyhow!(
                    "The channel to the main loop has been closed. As such, no new chunks can be loaded from the server. "
                ),
        }
    }
}

impl ChunkStorage for ClientWorldStorage {
    fn is_chunk_cached(&self, pos: &ChunkPos) -> bool {
        self.chunk_map.contains_key(&pos.as_long())
    }

    fn get_chunk(&mut self, pos: &ChunkPos) -> anyhow::Result<&Chunk> {
        let id = pos.as_long();

        if let Some(chunk) = self.chunk_map.get(&id) {
            Ok(chunk)
        } else {
            self.request_chunks
                .send(pos.clone())
                .or(Err(anyhow!(ClientChunkStorageError::RequestChannelClosed)))?;
            Ok(&EMPTY_CHUNK)
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
            request_chunks,
        }
    }
}