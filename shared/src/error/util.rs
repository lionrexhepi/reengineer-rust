use anyhow::anyhow;

pub enum PositionDeserializeError {
    BlockPos(u32),
    ChunkPos(u32),
}

impl From<PositionDeserializeError> for anyhow::Error {
    fn from(value: PositionDeserializeError) -> Self {
        match value {
            PositionDeserializeError::BlockPos(bits) =>
                anyhow!("Need at least 64 bits to deserialize a BlockPos, yet there are only {} available.", bits),
            PositionDeserializeError::ChunkPos(bits) =>
                anyhow!("Need at least 48 bits to deserialize a ChunkPos, yet there are only {} available.", bits),
        }
    }
}

#[derive(Debug)]
pub enum InvalidPositionError {
    InvalidX(i32),
    InvalidY(i32),
    InvalidZ(i32),
}

impl From<InvalidPositionError> for anyhow::Error {
    fn from(value: InvalidPositionError) -> Self {
        match value {
            InvalidPositionError::InvalidX(x) =>
                anyhow!(
                    "Invalid x value {}. X Coordinates must be in the range {:?}.",
                    x,
                    BlockPos::VALID_X
                ),
            InvalidPositionError::InvalidY(y) =>
                anyhow!(
                    "Invalid y value {}. Y Coordinates must be in the range {:?}.",
                    y,
                    BlockPos::VALID_Y
                ),
            InvalidPositionError::InvalidZ(z) =>
                anyhow!(
                    "Invalid z value {}. Z Coordinates must be in the range {:?}.",
                    z,
                    BlockPos::VALID_Z
                ),
        }
    }
}