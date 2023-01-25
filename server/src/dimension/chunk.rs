use log::error;
use metrohash::MetroHashMap;
use shared::{ dimension::chunk::*, util::pos::ChunkPos };

pub struct DiskChunkLoader {
    save_folder: Path
};

impl DiskChunkLoader {
    fn get_region_pos(chunk: &ChunkPos) -> (i32, i32) {
        (
            chunk.x() / 32,
            chunk.z() / 32
        )
    }
}

impl ChunkProvider for DiskChunkLoader {

    fn get_chunk(&self, _pos: &shared::util::pos::ChunkPos) -> Result<Chunk, ChunkProviderError> {
        todo!()
    }
}

trait ChunkGenerator: ChunkProvider {}

pub struct ServerChunkStorage {
    file_loader: DiskChunkLoader,
    generator: Box<dyn ChunkGenerator>,
    chunk_map: MetroHashMap<u64, Chunk>,
}

impl ServerChunkStorage {
    pub const EMPTY_CHUNK: Chunk = Chunk {};

    fn try_from_disk(&self, pos: &ChunkPos) -> Result<Chunk, bool> { //the error bool says whether you should try to generate the chunk first
        match self.file_loader.get_chunk(pos) {
            Ok(chunk) => Ok(chunk),
            Err(e) => Err(e == ChunkProviderError::ChunkNotGeneratedError)
        }
       
    }

    fn try_generate(&self, pos: &ChunkPos) -> Result<Chunk, ()> {
        match self.generator.get_chunk(pos) {
            Ok(chunk) => Ok(chunk)
            Err(err_type) => {
                error!("Couldn't generate chunk! Error of type {:#?}", err_type);
                debug_assert!(false);
                Err(())
            }
        }
    }
}

impl ChunkStorage for ServerChunkStorage {
    fn is_chunk_cached(&self, pos: &ChunkPos) -> bool {
        self.chunk_map.contains_key(&pos.as_long())
    }

    fn get_chunk(&mut self, pos: &ChunkPos) -> &Chunk {
        let id = pos.as_long();

        self.chunk_map.entry(id).or_insert(self.try_from_disk(pos).or_else(|should_generate| {
            if should_generate {
                self.try_generate(pos).or(Self::EMPTY_CHUNK)
            } else {
                Self::EMPTY_CHUNK
            }
        }))


        
    }
}