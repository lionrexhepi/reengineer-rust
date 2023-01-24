use log::error;
use metrohash::MetroHashMap;
use shared::{ dimension::chunk::*, util::pos::ChunkPos };

pub struct DiskChunkLoader;

trait ChunkGenerator: ChunkProvider {}

impl ChunkProvider for DiskChunkLoader {
    fn get_chunk(&self, _pos: &shared::util::pos::ChunkPos) -> Result<Chunk, ChunkProviderError> {
        todo!()
    }
}

pub struct ServerChunkStorage {
    file_loader: DiskChunkLoader,
    generator: Box<dyn ChunkGenerator>,
    chunk_map: MetroHashMap<u64, Chunk>,
}

impl ServerChunkStorage {
    pub const EMPTY_CHUNK: Chunk = Chunk {};

    fn cache_chunk(&mut self, id: u64, chunk: Chunk) -> &Chunk {
        self.chunk_map.insert(id, chunk);
        self.chunk_map.get(&id).unwrap()
    }
}

impl ChunkStorage for ServerChunkStorage {
    fn is_chunk_cached(&self, pos: &ChunkPos) -> bool {
        self.chunk_map.contains_key(&pos.as_long())
    }

    fn get_chunk(&mut self, pos: &ChunkPos) -> &Chunk {
        let id = pos.as_long();
        {
            if self.chunk_map.contains_key(&id) {
                return self.chunk_map.get(&id).unwrap();
            }
        }

        {
            match self.file_loader.get_chunk(pos) {
                Ok(chunk) => {
                    return self.cache_chunk(id, chunk);
                }
                Err(e) => {
                    let _ = match e {
                        ChunkProviderError::ChunkNotGeneratedError => 0,
                        _ => {
                            return &Self::EMPTY_CHUNK;
                        }
                    };
                }
            }
        }

        match self.generator.get_chunk(pos) {
            Ok(chunk) => { self.cache_chunk(id, chunk) }
            Err(err_type) => {
                error!("Couldn't generate chunk! Error of type {:#?}", err_type);
                debug_assert!(false);
                &Self::EMPTY_CHUNK
            }
        }
    }
}