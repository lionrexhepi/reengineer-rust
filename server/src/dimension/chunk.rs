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


}

impl ChunkStorage for ServerChunkStorage {
    fn is_chunk_cached(&self, pos: &ChunkPos) -> bool {
        self.chunk_map.contains_key(&pos.as_long())
    }

    fn get_chunk(&mut self, pos: &ChunkPos) -> &Chunk {
        let id = pos.as_long();
        self.chunk_map.entry(id).or_insert(match self.file_loader.get_chunk(pos) {
            Ok(chunk) => { chunk }
            Err(e) => {
                match e {
                    ChunkProviderError::ChunkNotGeneratedError =>
                        match self.generator.get_chunk(pos) {
                            Ok(chunk) => chunk,
                            Err(err_type) => {
                                error!("Couldn't generate chunk! Error of type {:#?}", err_type);
                                debug_assert!(false);
                                return &Self::EMPTY_CHUNK;
                            }
                        }
                    _ => { Self::EMPTY_CHUNK }
                }
            }
        })
    }
}