use std::{slice::from_raw_parts, io::Write};

use crate::{block::BlockId, cbs::{Packetable, PacketBuf, FixedSizePacketable}};


#[derive(Debug, Clone)]
pub struct SubChunk {
    pub(crate) data: [u16; Self::BLOCK_COUNT],
}

impl SubChunk {
    pub const DIMENSIONS: usize = 16;
    pub const BLOCK_COUNT: usize = Self::DIMENSIONS * Self::DIMENSIONS * Self::DIMENSIONS;
    pub const BYTES: usize = Self::BLOCK_COUNT * 2;
    

    pub fn get_block(&self, x: i16, y: i16, z: i16) -> BlockId {
        BlockId(self.data[((y << 8) | (z << 4) | x) as usize])
    }
}

impl Default for SubChunk {
    fn default() -> SubChunk {
        SubChunk {
            data: [0u16; Self::BLOCK_COUNT],
        }
    }
}


impl Packetable for SubChunk {
    fn write_to_buffer<T: std::io::Write + Unpin + Send>(
        self,
        buffer: &mut std::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.write_all(unsafe { from_raw_parts(self.data.as_ptr() as *const u8, Self::BYTES) })?;
        Ok(())
    }

    fn read_from_buf(reader: &mut PacketBuf) -> anyhow::Result<Self> where Self: Sized {
        let bytes = reader.next_bytes::<{ Self::BYTES }>()?;
        Ok(Self {
            data: (unsafe { from_raw_parts(bytes.as_ptr() as *const u16, Self::BLOCK_COUNT) })
                .try_into()
                .unwrap(),
        })
    }

  
    
}

impl FixedSizePacketable for SubChunk {
    const SIZE_IN_BYTES: usize = Self::BYTES;
}