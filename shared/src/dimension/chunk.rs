use anyhow::{ anyhow, ensure };
use bitter::{ BigEndianReader, BitReader };
use metrohash::MetroHashMap;
use tokio::io::BufReader;

use crate::{ util::pos::{ ChunkPos, BlockPos }, block::state::{ Block }, net::Packetable };

#[derive(Debug, Clone)]
pub struct SubChunk {
    data: [u16; Self::BLOCK_COUNT],
    y_base: u8,
}

pub struct InvalidSubChunkDataError(pub usize);

impl From<InvalidSubChunkDataError> for anyhow::Error {
    fn from(value: InvalidSubChunkDataError) -> Self {
        anyhow!(
            "Chunk data has invalid length. A chunk needs to be exactly {} bytes, while this one is only {} bytes.",
            SubChunk::BYTES,
            value.0
        )
    }
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

    pub fn try_from_data(y_base: u8, data: [u8; Self::BLOCK_COUNT * 2]) -> anyhow::Result<Self> {
        if data.len() != Self::BLOCK_COUNT * 2 {
        }

        ensure!(data.len() == Self::BYTES, InvalidSubChunkDataError(data.len()));

        Ok(SubChunk {
            y_base,
            data: (
                unsafe {
                    std::slice::from_raw_parts(data.as_ptr() as *const u16, Self::BLOCK_COUNT)
                }
            )
                .try_into()
                .unwrap(),
        })
    }

    pub fn y_base(&self) -> u8 {
        self.y_base
    }

    pub fn get_block(&self, x: i16, y: i16, z: i16) -> anyhow::Result<&Block> {
        Block::from_id(self.data[((y << 8) | (z << 4) | x) as usize])
    }
}

impl Packetable for SubChunk {
    fn write_to_buffer<T: tokio::io::AsyncWrite + Unpin>(
        self,
        buffer: &mut tokio::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn read_from_bytes(reader: &mut BigEndianReader) -> anyhow::Result<Self> where Self: Sized {
        todo!()
    }
}

pub enum InvalidChunkDataError {
    InvalidHeaderSize(usize),
    InvalidDataSize(usize, usize),
}

impl From<InvalidChunkDataError> for anyhow::Error {
    fn from(value: InvalidChunkDataError) -> Self {
        match value {
            InvalidChunkDataError::InvalidHeaderSize(size) =>
                anyhow!("Found {}-bit subchunk count instead of the expected 5 bits ", 0),
            InvalidChunkDataError::InvalidDataSize(actual, expected) =>
                anyhow!(
                    "The chunk data is {} bits, or {} bytes large, but it needs to be {} bits / {} bytes",
                    actual,
                    actual / 8,
                    expected,
                    expected / 8
                ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    non_air_sub_chunks: MetroHashMap<u8, SubChunk>,
}

impl Chunk {
    pub fn empty() -> Chunk {
        Chunk { non_air_sub_chunks: MetroHashMap::default() }
    }

    pub fn get_block(&self, pos: BlockPos) -> anyhow::Result<&Block> {
        let y = pos.y();
        pos.validate()?;

        Ok(match self.non_air_sub_chunks.get(&((y << 4) as u8)) {
            Some(sc) =>
                sc.get_block((pos.x() & 15) as i16, (y & 15) as i16, (pos.z() & 15) as i16)?,
            None => <&Block>::default(),
        })
    }
}

impl Packetable for Chunk {
    fn write_to_buffer<T: tokio::io::AsyncWrite + Unpin>(
        self,
        buffer: &mut tokio::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn read_from_bytes(reader: &mut BigEndianReader) -> anyhow::Result<Self> where Self: Sized {
        let len = reader.refill_lookahead();

        ensure!(len > 5, InvalidChunkDataError::InvalidHeaderSize(len as usize));

        let sub_chunk_count = reader.peek(5) as usize;

        let bits_needed = sub_chunk_count * (SubChunk::BYTES * 8 + 5);

        let len = reader.refill_lookahead() as usize;

        ensure!(len >= bits_needed, InvalidChunkDataError::InvalidDataSize(len, bits_needed));

        let mut sub_chunks = MetroHashMap::default();

        for _ in 0..sub_chunk_count {
            let y_base = (reader.peek(5) as u8) * 16;
            let mut data = [0u8; SubChunk::BYTES];

            (0..SubChunk::BYTES).for_each(|i| {
                data[i] = reader.peek(8) as u8;
                reader.consume(8);
            });

            sub_chunks.insert(y_base, SubChunk::try_from_data(y_base, data)?);
        }

        Ok(Self {
            non_air_sub_chunks: sub_chunks,
        })
    }
}

#[derive(Debug)]
pub enum ChunkProviderError {
    ChunkGenerationFailedError(ChunkPos),
}

impl From<ChunkProviderError> for anyhow::Error {
    fn from(value: ChunkProviderError) -> Self {
        match value {
            ChunkProviderError::ChunkGenerationFailedError(pos) =>
                anyhow!("Couldn't generate chunk {}/{}", pos.x(), pos.z()),
        }
    }
}

pub trait ChunkLoader {
    fn get_chunk(&self, pos: &ChunkPos) -> Option<Chunk>;
}



pub trait ChunkStorage {
    fn is_chunk_cached(&self, pos: &ChunkPos) -> bool;

    fn get_chunk(&mut self, pos: &ChunkPos) -> anyhow::Result<&Chunk>;
}