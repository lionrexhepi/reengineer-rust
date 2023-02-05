use crate::{util::pos::ChunkPos, dimension::chunk::SubChunk};
use anyhow::anyhow;

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
