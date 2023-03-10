use std::{ ops::Range };


use glam::{ IVec3, Vec3 };

use anyhow::{ ensure };

use crate::{
    cbs::{Packetable, PacketBuf, FixedSizePacketable, WriteExt},
    error::util::{ InvalidPositionError },
};

use super::chunk_pos::ChunkPos;




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
    fn write_to_buffer<T: std::io::Write + Unpin + Send>(
        self,
        buffer: &mut std::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.write_u64(self.as_long())?;
        Ok(())
    }

    fn read_from_buf(reader: &mut PacketBuf) -> anyhow::Result<Self>
        where Self: Sized
    {
        let long = reader.next_u64()?;

        Ok(Self::from_long(long))
    }

    
}

impl FixedSizePacketable for BlockPos {
    const SIZE_IN_BYTES: usize = 8;
}