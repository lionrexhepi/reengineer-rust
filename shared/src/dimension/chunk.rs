use anyhow::{ ensure };
use bitter::{ BigEndianReader, BitReader };
use metrohash::MetroHashMap;

use crate::{
    util::pos::{ ChunkPos, BlockPos },
    block::state::{ Block, BlockId },
    error::dimension::{ InvalidSubChunkDataError, InvalidChunkDataError },
};
use crate::cbs::Packetable;

#[derive(Debug, Clone)]
pub struct SubChunk {
    pub(crate) data: [u16; Self::BLOCK_COUNT],
}

impl SubChunk {
    pub const DIMENSIONS: usize = 16;
    pub const BLOCK_COUNT: usize = Self::DIMENSIONS * Self::DIMENSIONS * Self::DIMENSIONS;
    pub const BYTES: usize = Self::BLOCK_COUNT * 2;
    pub fn new() -> SubChunk {
        SubChunk {
            data: [0u16; Self::BLOCK_COUNT],
        }
    }

    pub fn get_block(&self, x: i16, y: i16, z: i16) -> BlockId {
        BlockId(self.data[((y << 8) | (z << 4) | x) as usize])
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub(crate) non_air_sub_chunks: MetroHashMap<u8, SubChunk>,
}

impl Chunk {
    pub fn empty() -> Chunk {
        Chunk { non_air_sub_chunks: MetroHashMap::default() }
    }

    pub fn get_block(&self, pos: BlockPos) -> anyhow::Result<BlockId> {
        let y = pos.y();
        let _ =pos.validate()?;

        Ok(match self.non_air_sub_chunks.get(&((y << 4) as u8)) {
            Some(sc) =>
                sc.get_block((pos.x() & 15) as i16, (y & 15) as i16, (pos.z() & 15) as i16),
            None => BlockId::default(),
        })
    }
}

pub trait ChunkLoader {
    fn get_chunk(&self, pos: &ChunkPos) -> Option<Chunk>;
}

pub trait ChunkStorage {
    fn is_chunk_cached(&self, pos: &ChunkPos) -> bool;

    fn get_chunk(&mut self, pos: &ChunkPos) -> anyhow::Result<&Chunk>;
}