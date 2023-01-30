use bitter::{ LittleEndianReader, BitReader };
use metrohash::MetroHashMap;

use crate::{
    util::pos::{ ChunkPos, BlockPos },
    block::{ state::{ Block, State }, simple::AirState },
};

#[derive(Debug)]
pub struct SubChunk {
    data: [u16; Self::BLOCK_COUNT],
    y_base: u8,
}

impl SubChunk {
    pub const DIMENSIONS: usize = 16;
    pub const BLOCK_COUNT: usize = Self::DIMENSIONS * Self::DIMENSIONS * Self::DIMENSIONS;
    pub const BYTES: usize = Self::BLOCK_COUNT * 2;
    pub fn new(y_base: u8) -> SubChunk {
        SubChunk {
            y_base,
            data: [0u16; Self::BLOCK_COUNT],
        }
    }

    pub fn from_data(y_base: u8, data: [u8; Self::BLOCK_COUNT * 2]) -> SubChunk {
        assert!(data.len() == Self::BLOCK_COUNT * 2);

        SubChunk {
            y_base,
            data: (
                unsafe {
                    std::slice::from_raw_parts(data.as_ptr() as *const u16, Self::BLOCK_COUNT)
                }
            )
                .try_into()
                .unwrap(),
        }
    }

    pub fn y_base(&self) -> u8 {
        self.y_base
    }

    pub fn get_block(&self, x: i16, y: i16, z: i16) -> Block {
        Block::from_id(self.data[((y << 8) | (z << 4) | x) as usize]).unwrap_or(
            Block::Air(AirState::DEFAULT)
        )
    }
}

#[derive(Debug)]
pub struct Chunk {
    non_air_sub_chunks: MetroHashMap<u8, SubChunk>,
}

impl Chunk {
    pub fn empty() -> Chunk {
        Chunk { non_air_sub_chunks: MetroHashMap::default() }
    }

    pub fn from_data(data: &[u8]) -> Self {
        let mut reader = LittleEndianReader::new(data);

        let len = reader.refill_lookahead() as usize;
        assert!(len > 5);
        let subchunk_count = reader.peek(5) as u8;
        reader.consume(5);
        assert!(len == (subchunk_count as usize) * (5 + SubChunk::BYTES * 8));

        let mut sub_chunks = MetroHashMap::default();

        for _ in 0..subchunk_count {
            let y_base = reader.peek(5) as u8;
            let mut data = [0u8; SubChunk::BYTES];

            (0..SubChunk::BYTES).for_each(|i| {
                data[i] = reader.peek(8) as u8;
                reader.consume(8);
            });

            sub_chunks.insert(y_base, SubChunk::from_data(y_base, data));
        }

        Self {
            non_air_sub_chunks: sub_chunks,
        }
    }

    pub fn get_block(&self, pos: BlockPos) -> Block {
        let y = pos.y();
        assert!(y > 0 && y < 256);

        match self.non_air_sub_chunks.get(&((y << 4) as u8)) {
            Some(sc) =>
                sc.get_block((pos.x() & 15) as i16, (pos.y() & 15) as i16, (pos.z() & 15) as i16),
            None => Block::Air(AirState::DEFAULT),
        }
    }
}

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