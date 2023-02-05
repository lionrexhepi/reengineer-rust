use std::{ path::Path, collections::hash_map::Entry };

use log::error;
use metrohash::MetroHashMap;
use once_cell::sync::Lazy;
use shared::{ dimension::chunk::*, util::pos::ChunkPos };

pub struct DiskChunkLoader {
    save_folder: Box<Path>,
}

impl DiskChunkLoader {
    fn get_region_pos(chunk: &ChunkPos) -> (i32, i32) {
        (chunk.x() / 32, chunk.z() / 32)
    }
}

impl ChunkLoader for DiskChunkLoader {
    fn get_chunk(&self, _pos: &shared::util::pos::ChunkPos) -> Option<Chunk> {
        None
    }
}

trait ChunkGenerator {
    fn try_generate(&self, pos: &ChunkPos) -> anyhow::Result<Chunk>;
}

pub struct ServerChunkStorage {
    file_loader: DiskChunkLoader,
    generator: Box<dyn ChunkGenerator>,
    chunk_map: MetroHashMap<u64, Chunk>,
}

static EMPTY_CHUNK: Lazy<Chunk> = Lazy::new(Chunk::empty);

impl ChunkStorage for ServerChunkStorage {
    fn is_chunk_cached(&self, pos: &ChunkPos) -> bool {
        self.chunk_map.contains_key(&pos.as_long())
    }

    fn get_chunk(&mut self, pos: &ChunkPos) -> anyhow::Result<&Chunk> {
        let id = pos.as_long();

        let entry = self.chunk_map.entry(id);

        if let Entry::Occupied(inner) = entry {
            Ok(inner.get())
        } else if let Some(chunk) = self.file_loader.get_chunk(pos) {
            Ok(entry.or_insert(chunk))
        } else {
            Ok(entry.or_insert(self.generator.try_generate(pos)?))
        }
    }
}