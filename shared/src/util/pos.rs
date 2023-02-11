use std::{ ops::Range };

use bitter::BitReader;
use glam::{ IVec3, Vec3 };
use tokio::io::AsyncWriteExt;

use anyhow::{ ensure };

use crate::{
    cbs::Packetable,
    wait,
    error::util::{ PositionDeserializeError, InvalidPositionError },
};

#[derive(Debug, Clone)]
pub struct ChunkPos {
    x: i32,
    z: i32,
}

impl ChunkPos {
    pub fn as_long(&self) -> u64 {
        //mostly used as a hash map key
        ((self.x as u64) << 32) | (self.z as u64)
    }

    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn z(&self) -> i32 {
        self.z
    }
}

impl Packetable for ChunkPos {
    fn write_to_buffer<T: tokio::io::AsyncWrite + Unpin>(
        self,
        buffer: &mut tokio::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        wait!(
            buffer.write(
                &[((self.x >> 16) & 255) as u8, ((self.x >> 8) & 255) as u8, (self.x & 255) as u8]
            ) // only the first 23 bits are actually used, so we write 3 bytes(24 bits) per coordinate
        )?;
        wait!(
            buffer.write(
                &[((self.z >> 16) & 255) as u8, ((self.z >> 8) & 255) as u8, (self.z & 255) as u8]
            )
        )?;
        Ok(())
    }

    fn read_from_buf(reader: &mut bitter::BigEndianReader) -> anyhow::Result<Self> {
        let len = reader.refill_lookahead();
        ensure!(len >= 48, PositionDeserializeError::ChunkPos(len));
        let x = reader.peek(24) as i32;
        let z = reader.peek(24) as i32;
        reader.consume(48);
        Ok(Self { x, z })
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BlockPos {
    // A Block position serializable to a 64-bit large space
    x: i32,
    y: i32,
    z: i32,
}

impl BlockPos {
    pub const X_BITS: u64 = 27;
    pub const Y_BITS: u64 = 10;
    pub const Z_BITS: u64 = 27;

    pub const X_SHIFT: u64 = Self::Y_BITS + Self::Z_BITS;
    pub const Y_SHIFT: u64 = 0;
    pub const Z_SHIFT: u64 = Self::Y_BITS;

    pub const X_MASK: u64 = (1 << Self::X_BITS) - 1;
    pub const Y_MASK: u64 = (1 << Self::Y_BITS) - 1;
    pub const Z_MASK: u64 = (1 << Self::Z_BITS) - 1;

    pub const MAX_X_DISTANCE: i32 = (1 << Self::X_BITS) / 2;
    pub const MAX_Y_DISTANCE: i32 = (1 << Self::Y_BITS) / 2;
    pub const MAX_Z_DISTANCE: i32 = (1 << Self::Z_BITS) / 2;

    pub const VALID_X: Range<i32> = -Self::MAX_X_DISTANCE..Self::MAX_X_DISTANCE;

    pub const VALID_Y: Range<i32> = -Self::MAX_Y_DISTANCE..Self::MAX_Y_DISTANCE;

    pub const VALID_Z: Range<i32> = -Self::MAX_Z_DISTANCE..Self::MAX_Z_DISTANCE;

    pub fn new(x: i32, y: i32, z: i32) -> BlockPos {
        debug_assert!(Self::VALID_X.contains(&x));
        debug_assert!(Self::VALID_Y.contains(&y));
        debug_assert!(Self::VALID_Z.contains(&z));

        Self { x, y, z }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn as_long(&self) -> u64 {
        let x = self.x as u64;
        let y = self.y as u64;
        let z = self.z as u64;
        ((x & Self::X_MASK) << Self::X_SHIFT) |
            ((y & Self::Y_MASK) << Self::Y_SHIFT) |
            ((z & Self::Z_MASK) << Self::Z_SHIFT)
    }

    pub fn from_long(long: u64) -> Self {
        let x = long << (64 - Self::X_SHIFT - Self::X_BITS);
        let y = long << (64 - Self::Y_SHIFT - Self::Y_BITS);
        let z = long << (64 - Self::Z_SHIFT - Self::Z_BITS);

        let x = x >> (64 - Self::X_BITS);
        let y = y >> (64 - Self::Y_BITS);
        let z = z >> (64 - Self::Z_BITS);

        Self {
            x: x as i32,
            y: y as i32,
            z: z as i32,
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        ensure!(Self::VALID_X.contains(&self.x), InvalidPositionError::InvalidX(self.x));
        ensure!(Self::VALID_Y.contains(&self.y), InvalidPositionError::InvalidY(self.y));
        ensure!(Self::VALID_Z.contains(&self.z), InvalidPositionError::InvalidZ(self.z));

        Ok(())
    }

    pub fn offset_up(&self) -> Self {
        Self {
            y: self.y + 1,
            ..*self
        }
    }

    pub fn offset_down(&self) -> Self {
        Self {
            y: self.y - 1,
            ..*self
        }
    }

    pub fn offset_north(&self) -> Self {
        Self {
            z: self.z - 1,
            ..*self
        }
    }

    pub fn offset_south(&self) -> Self {
        Self {
            z: self.z + 1,
            ..*self
        }
    }

    pub fn offset_west(&self) -> Self {
        Self {
            x: self.x - 1,
            ..*self
        }
    }

    pub fn offset_east(&self) -> Self {
        Self {
            x: self.x + 1,
            ..*self
        }
    }

    pub fn get_chunk(&self) -> ChunkPos {
        ChunkPos::new(self.x >> 4, self.z >> 4)
    }
}

impl From<BlockPos> for IVec3 {
    fn from(value: BlockPos) -> Self {
        Self {
            x: value.x(),
            y: value.y(),
            z: value.z(),
        }
    }
}

impl From<BlockPos> for Vec3 {
    fn from(value: BlockPos) -> Self {
        Self {
            x: value.x() as f32,
            y: value.y() as f32,
            z: value.z() as f32,
        }
    }
}

impl Packetable for BlockPos {
    fn write_to_buffer<T: tokio::io::AsyncWrite + Unpin>(
        self,
        buffer: &mut tokio::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        wait!(buffer.write_u64(self.as_long()))?;
        Ok(())
    }

    fn read_from_buf(reader: &mut bitter::BigEndianReader) -> anyhow::Result<Self> {
        let len = reader.refill_lookahead();
        ensure!(len >= 64, PositionDeserializeError::BlockPos(len));
        let long = reader.peek(64);
        reader.consume(64);
        Ok(Self::from_long(long))
    }
}